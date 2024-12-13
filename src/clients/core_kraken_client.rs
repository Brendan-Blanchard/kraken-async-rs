//! A base implementation of [KrakenClient]
use crate::clients::errors::ClientError;
use crate::clients::errors::KrakenError;
use crate::clients::http_response_types::ResultErrorResponse;
use crate::clients::kraken_client::endpoints::*;
use crate::clients::kraken_client::KrakenClient;
use crate::crypto::nonce_provider::NonceProvider;
use crate::crypto::nonce_request::NonceRequest;
use crate::crypto::signatures::{generate_signature, Signature};
use crate::request_types::*;
use crate::response_types::*;
use crate::secrets::secrets_provider::SecretsProvider;
#[allow(unused)]
use crate::secrets::secrets_provider::StaticSecretsProvider;
use http_body_util::BodyExt;
use hyper::http::request::Builder;
use hyper::{Method, Request, Uri};
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use to_query_params::{QueryParams, ToQueryParams};
use tokio::sync::Mutex;
use tracing::{debug, trace, warn};
use url::{form_urlencoded, Url};

#[derive(QueryParams, Default)]
struct EmptyRequest {}

/// The base implementation of [KrakenClient]. It has no rate limiting, and uses whatever
/// [SecretsProvider] and [NonceProvider] it is given.
///
/// This is most useful for one-off calls, or building more complex behavior on top of.
///
/// # Example: Making a Public API Call
/// Creating a [CoreKrakenClient] is as simple as providing a [SecretsProvider] and [NonceProvider].
/// For public calls, a [StaticSecretsProvider] with empty strings will work, since there is no auth
/// required for public endpoints.
///
/// Requests follow a builder pattern, with required parameters in the `::builder()` call, if there
/// are any. Here, only the pair (optional) is provided.
///
/// ```
/// # use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
/// # use kraken_async_rs::clients::kraken_client::KrakenClient;
/// # use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
/// # use kraken_async_rs::clients::http_response_types::ResultErrorResponse;
/// # use kraken_async_rs::request_types::{StringCSV, TradableAssetPairsRequest};
/// # use kraken_async_rs::secrets::secrets_provider::StaticSecretsProvider;
/// # use std::sync::Arc;
/// # use tokio::sync::Mutex;
///
/// #[tokio::main]
/// async fn main() {
///     // credentials aren't needed for public endpoints
///     use kraken_async_rs::secrets::secrets_provider::SecretsProvider;
/// let secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>> = Box::new(Arc::new(Mutex::new(StaticSecretsProvider::new("", ""))));
///     let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
///         Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
///     let mut client = CoreKrakenClient::new(secrets_provider, nonce_provider);
///
///     let request = TradableAssetPairsRequest::builder()
///         .pair(StringCSV::new(vec!["BTCUSD".to_string()]))
///         .build();
///
///     let open_orders_response = client.get_tradable_asset_pairs(&request).await;
///
///     // Note that Kraken will return assets in their own naming scheme, e.g. a request for
///     // "BTCUSD" will return as "XXBTZUSD"
///     // For a reasonable understanding of their mappings, see: https://gist.github.com/brendano257/975a395d73a6d7bb53e53d292534d6af
///     if let Ok(ResultErrorResponse {
///         result: Some(tradable_assets),
///         ..
///     }) = open_orders_response
///     {
///         for (asset, details) in tradable_assets {
///             println!("{asset}: {details:?}")
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct CoreKrakenClient {
    pub api_url: String,
    secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>>,
    nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
    http_client: Client<HttpsConnector<HttpConnector>, String>,
    user_agent: Option<String>,
    trace_inbound: bool,
}

impl KrakenClient for CoreKrakenClient {
    fn new(
        secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
    ) -> Self {
        if cfg!(feature = "debug-inbound") {
            warn!("Feature `debug-inbound` is deprecated - use `new_with_tracing` method to set tracing flag")
        }

        let https = HttpsConnector::new();
        let http_client: Client<HttpsConnector<HttpConnector>, String> =
            Client::builder(TokioExecutor::new()).build(https);
        CoreKrakenClient {
            api_url: KRAKEN_BASE_URL.into(),
            secrets_provider,
            nonce_provider,
            http_client,
            user_agent: None,
            trace_inbound: false,
        }
    }

    fn new_with_url(
        secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
        url: String,
    ) -> Self {
        let https = HttpsConnector::new();
        let http_client = Client::builder(TokioExecutor::new()).build(https);
        CoreKrakenClient {
            api_url: url,
            secrets_provider,
            nonce_provider,
            http_client,
            user_agent: None,
            trace_inbound: false,
        }
    }

    fn new_with_tracing(
        secrets_provider: Box<Arc<Mutex<dyn SecretsProvider>>>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
        trace_inbound: bool,
    ) -> Self {
        let https = HttpsConnector::new();
        let http_client = Client::builder(TokioExecutor::new()).build(https);
        CoreKrakenClient {
            api_url: KRAKEN_BASE_URL.into(),
            secrets_provider,
            nonce_provider,
            http_client,
            user_agent: None,
            trace_inbound,
        }
    }

    async fn set_user_agent(&mut self, user_agent: String) {
        self.user_agent = Some(user_agent);
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn get_server_time(&mut self) -> Result<ResultErrorResponse<SystemTime>, ClientError> {
        let url = Url::from_str(&self.api_url(TIME_ENDPOINT))?;
        let body = self.body_from_url(Method::GET, &url, "".into()).await?;
        Ok(serde_json::from_str(&body)?)
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn get_system_status(
        &mut self,
    ) -> Result<ResultErrorResponse<SystemStatusInfo>, ClientError> {
        let url = Url::from_str(&self.api_url(STATUS_ENDPOINT))?;
        let body = self.body_from_url(Method::GET, &url, "".into()).await?;
        Ok(serde_json::from_str(&body)?)
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_asset_info(
        &mut self,
        request: &AssetInfoRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, AssetInfo>>, ClientError> {
        self.public_get(ASSET_INFO_ENDPOINT, request).await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_tradable_asset_pairs(
        &mut self,
        request: &TradableAssetPairsRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, TradableAssetPair>>, ClientError> {
        self.public_get(TRADABLE_ASSET_PAIRS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_ticker_information(
        &mut self,
        request: &TickerRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, RestTickerInfo>>, ClientError> {
        self.public_get(TICKER_INFO_ENDPOINT, request).await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_ohlc(
        &mut self,
        request: &OHLCRequest,
    ) -> Result<ResultErrorResponse<OhlcResponse>, ClientError> {
        self.public_get(OHLC_ENDPOINT, request).await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_orderbook(
        &mut self,
        request: &OrderbookRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, Orderbook>>, ClientError> {
        self.public_get(ORDER_BOOK_ENDPOINT, request).await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_recent_trades(
        &mut self,
        request: &RecentTradesRequest,
    ) -> Result<ResultErrorResponse<RecentTrades>, ClientError> {
        self.public_get(RECENT_TRADES_ENDPOINT, request).await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_recent_spreads(
        &mut self,
        request: &RecentSpreadsRequest,
    ) -> Result<ResultErrorResponse<RecentSpreads>, ClientError> {
        self.public_get(RECENT_SPREADS_ENDPOINT, request).await
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn get_account_balance(
        &mut self,
    ) -> Result<ResultErrorResponse<AccountBalances>, ClientError> {
        self.private_form_post(ACCOUNT_BALANCE_ENDPOINT, &EmptyRequest::default())
            .await
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn get_extended_balances(
        &mut self,
    ) -> Result<ResultErrorResponse<ExtendedBalances>, ClientError> {
        self.private_form_post(ACCOUNT_BALANCE_EXTENDED_ENDPOINT, &EmptyRequest::default())
            .await
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn get_trade_balances(
        &mut self,
        request: &TradeBalanceRequest,
    ) -> Result<ResultErrorResponse<TradeBalances>, ClientError> {
        self.private_form_post(TRADE_BALANCE_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_open_orders(
        &mut self,
        request: &OpenOrdersRequest,
    ) -> Result<ResultErrorResponse<OpenOrders>, ClientError> {
        self.private_form_post(OPEN_ORDERS_ENDPOINT, request).await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_closed_orders(
        &mut self,
        request: &ClosedOrdersRequest,
    ) -> Result<ResultErrorResponse<ClosedOrders>, ClientError> {
        self.private_form_post(CLOSED_ORDERS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn query_orders_info(
        &mut self,
        request: &OrderRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, Order>>, ClientError> {
        self.private_form_post(QUERY_ORDERS_ENDPOINT, request).await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_order_amends(
        &mut self,
        request: &OrderAmendsRequest,
    ) -> Result<ResultErrorResponse<OrderAmends>, ClientError> {
        self.private_json_post(ORDER_AMENDS_ENDPOINT, request).await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_trades_history(
        &mut self,
        request: &TradesHistoryRequest,
    ) -> Result<ResultErrorResponse<TradesHistory>, ClientError> {
        self.private_form_post(TRADES_HISTORY_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn query_trades_info(
        &mut self,
        request: &TradeInfoRequest,
    ) -> Result<ResultErrorResponse<TradesInfo>, ClientError> {
        self.private_form_post(QUERY_TRADES_ENDPOINT, request).await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_open_positions(
        &mut self,
        request: &OpenPositionsRequest,
    ) -> Result<ResultErrorResponse<OpenPositions>, ClientError> {
        self.private_form_post(OPEN_POSITIONS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_ledgers_info(
        &mut self,
        request: &LedgersInfoRequest,
    ) -> Result<ResultErrorResponse<LedgerInfo>, ClientError> {
        self.private_form_post(LEDGERS_ENDPOINT, request).await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn query_ledgers(
        &mut self,
        request: &QueryLedgerRequest,
    ) -> Result<ResultErrorResponse<QueryLedgerInfo>, ClientError> {
        self.private_form_post(QUERY_LEDGERS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn get_trade_volume(
        &mut self,
        request: &TradeVolumeRequest,
    ) -> Result<ResultErrorResponse<TradeVolume>, ClientError> {
        self.private_form_post(TRADE_VOLUME_ENDPOINT, request).await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn request_export_report(
        &mut self,
        request: &ExportReportRequest,
    ) -> Result<ResultErrorResponse<ExportReport>, ClientError> {
        self.private_form_post(ADD_EXPORT_ENDPOINT, request).await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_export_report_status(
        &mut self,
        request: &ExportReportStatusRequest,
    ) -> Result<ResultErrorResponse<Vec<ExportReportStatus>>, ClientError> {
        self.private_form_post(EXPORT_STATUS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn retrieve_export_report(
        &mut self,
        request: &RetrieveExportReportRequest,
    ) -> Result<Vec<u8>, ClientError> {
        self.private_post_binary::<RetrieveExportReportRequest>(RETRIEVE_EXPORT_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn delete_export_report(
        &mut self,
        request: &DeleteExportRequest,
    ) -> Result<ResultErrorResponse<DeleteExportReport>, ClientError> {
        self.private_form_post(REMOVE_EXPORT_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn add_order(
        &mut self,
        request: &AddOrderRequest,
    ) -> Result<ResultErrorResponse<AddOrder>, ClientError> {
        self.private_form_post(ADD_ORDER_ENDPOINT, request).await
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn add_order_batch(
        &mut self,
        request: &AddBatchedOrderRequest,
    ) -> Result<ResultErrorResponse<AddOrderBatch>, ClientError> {
        self.private_json_post(ADD_ORDER_BATCH_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn amend_order(
        &mut self,
        request: &AmendOrderRequest,
    ) -> Result<ResultErrorResponse<AmendOrder>, ClientError> {
        self.private_json_post(AMEND_ORDER_ENDPOINT, request).await
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn edit_order(
        &mut self,
        request: &EditOrderRequest,
    ) -> Result<ResultErrorResponse<OrderEdit>, ClientError> {
        self.private_form_post(EDIT_ORDER_ENDPOINT, request).await
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn cancel_order(
        &mut self,
        request: &CancelOrderRequest,
    ) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        self.private_form_post(CANCEL_ORDER_ENDPOINT, request).await
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn cancel_all_orders(&mut self) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        self.private_form_post(CANCEL_ALL_ORDERS_ENDPOINT, &EmptyRequest::default())
            .await
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn cancel_all_orders_after(
        &mut self,
        request: &CancelAllOrdersAfterRequest,
    ) -> Result<ResultErrorResponse<CancelAllOrdersAfter>, ClientError> {
        self.private_form_post(CANCEL_ALL_ORDERS_AFTER_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(ret, err(Debug), skip(self))]
    async fn cancel_order_batch(
        &mut self,
        request: &CancelBatchOrdersRequest,
    ) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        self.private_json_post(CANCEL_ORDER_BATCH_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_deposit_methods(
        &mut self,
        request: &DepositMethodsRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositMethod>>, ClientError> {
        self.private_form_post(DEPOSIT_METHODS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_deposit_addresses(
        &mut self,
        request: &DepositAddressesRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositAddress>>, ClientError> {
        self.private_form_post(DEPOSIT_ADDRESSES_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_status_of_recent_deposits(
        &mut self,
        request: &StatusOfDepositWithdrawRequest,
    ) -> Result<ResultErrorResponse<DepositWithdrawResponse>, ClientError> {
        self.private_form_post(DEPOSIT_STATUS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_withdrawal_methods(
        &mut self,
        request: &WithdrawalMethodsRequest,
    ) -> Result<ResultErrorResponse<Vec<WithdrawMethod>>, ClientError> {
        self.private_form_post(WITHDRAW_METHODS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_withdrawal_addresses(
        &mut self,
        request: &WithdrawalAddressesRequest,
    ) -> Result<ResultErrorResponse<Vec<WithdrawalAddress>>, ClientError> {
        self.private_form_post(WITHDRAW_ADDRESSES_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_withdrawal_info(
        &mut self,
        request: &WithdrawalInfoRequest,
    ) -> Result<ResultErrorResponse<Withdrawal>, ClientError> {
        self.private_form_post(WITHDRAW_INFO_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn withdraw_funds(
        &mut self,
        request: &WithdrawFundsRequest,
    ) -> Result<ResultErrorResponse<ConfirmationRefId>, ClientError> {
        self.private_form_post(WITHDRAW_ENDPOINT, request).await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_status_of_recent_withdrawals(
        &mut self,
        request: &StatusOfDepositWithdrawRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositWithdrawal>>, ClientError> {
        self.private_form_post(WITHDRAW_STATUS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn request_withdrawal_cancellation(
        &mut self,
        request: &WithdrawCancelRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        self.private_form_post(WITHDRAW_CANCEL_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn request_wallet_transfer(
        &mut self,
        request: &WalletTransferRequest,
    ) -> Result<ResultErrorResponse<ConfirmationRefId>, ClientError> {
        self.private_form_post(WALLET_TRANSFER_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn create_sub_account(
        &mut self,
        request: &CreateSubAccountRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        self.private_form_post(CREATE_SUB_ACCOUNT_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn account_transfer(
        &mut self,
        request: &AccountTransferRequest,
    ) -> Result<ResultErrorResponse<AccountTransfer>, ClientError> {
        self.private_form_post(ACCOUNT_TRANSFER_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn allocate_earn_funds(
        &mut self,
        request: &AllocateEarnFundsRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        self.private_form_post(EARN_ALLOCATE_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn deallocate_earn_funds(
        &mut self,
        request: &AllocateEarnFundsRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        self.private_form_post(EARN_DEALLOCATE_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_earn_allocation_status(
        &mut self,
        request: &EarnAllocationStatusRequest,
    ) -> Result<ResultErrorResponse<AllocationStatus>, ClientError> {
        self.private_form_post(EARN_ALLOCATE_STATUS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_earn_deallocation_status(
        &mut self,
        request: &EarnAllocationStatusRequest,
    ) -> Result<ResultErrorResponse<AllocationStatus>, ClientError> {
        self.private_form_post(EARN_DEALLOCATE_STATUS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn list_earn_strategies(
        &mut self,
        request: &ListEarnStrategiesRequest,
    ) -> Result<ResultErrorResponse<EarnStrategies>, ClientError> {
        self.private_form_post(EARN_STRATEGIES_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn list_earn_allocations(
        &mut self,
        request: &ListEarnAllocationsRequest,
    ) -> Result<ResultErrorResponse<EarnAllocations>, ClientError> {
        self.private_form_post(EARN_ALLOCATIONS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(err(Debug), skip(self))]
    async fn get_websockets_token(
        &mut self,
    ) -> Result<ResultErrorResponse<WebsocketToken>, ClientError> {
        let url = Url::from_str(&self.api_url(GET_WS_TOKEN_ENDPOINT))?;
        let signature = self
            .get_form_signature(GET_WS_TOKEN_ENDPOINT, &EmptyRequest::default())
            .await;

        let response_body = self
            .body_from_url_and_form_with_auth(Method::POST, &url, signature)
            .await?;

        Ok(serde_json::from_str(&response_body)?)
    }
}

impl CoreKrakenClient {
    fn api_url(&self, endpoint: &str) -> String {
        format!("{}{}", self.api_url, endpoint)
    }

    fn get_user_agent(&self) -> String {
        self.user_agent
            .clone()
            .unwrap_or("KrakenAsyncRsClient".to_string())
    }

    fn add_query_params<T: ToQueryParams>(url: &mut Url, request: &T) {
        for (k, v) in request.to_query_params() {
            url.query_pairs_mut().append_pair(&k, &v);
        }
    }

    fn request_builder_from_url(method: Method, url: &Url) -> Result<Builder, ClientError> {
        let uri = url.as_str().parse::<Uri>()?;
        Ok(Request::builder().method(method).uri(uri.to_string()))
    }

    async fn public_get<T, R>(
        &self,
        url: &str,
        request: &R,
    ) -> Result<ResultErrorResponse<T>, ClientError>
    where
        T: for<'a> Deserialize<'a>,
        R: ToQueryParams,
    {
        let mut url = Url::from_str(&self.api_url(url))?;
        Self::add_query_params(&mut url, request);

        let response_body = self.body_from_url(Method::GET, &url, "".into()).await?;
        Self::parse_body_and_errors(&response_body)
    }

    async fn private_form_post<T, R>(
        &mut self,
        url: &str,
        request: &R,
    ) -> Result<ResultErrorResponse<T>, ClientError>
    where
        T: for<'a> Deserialize<'a>,
        R: ToQueryParams,
    {
        let signature = self.get_form_signature(url, request).await;
        let url = Url::from_str(&self.api_url(url))?;

        let response_body = self
            .body_from_url_and_form_with_auth(Method::POST, &url, signature)
            .await?;

        Self::parse_body_and_errors(&response_body)
    }

    async fn private_json_post<T, R>(
        &mut self,
        url: &str,
        request: &R,
    ) -> Result<ResultErrorResponse<T>, ClientError>
    where
        T: for<'a> Deserialize<'a>,
        R: Serialize,
    {
        let signature = self.get_json_signature(url, request).await?;
        let url = Url::from_str(&self.api_url(url))?;

        let response_body = self
            .body_from_url_and_json_with_auth(Method::POST, &url, signature)
            .await?;

        Self::parse_body_and_errors(&response_body)
    }

    async fn private_post_binary<R>(
        &mut self,
        url: &str,
        request: &R,
    ) -> Result<Vec<u8>, ClientError>
    where
        R: ToQueryParams,
    {
        let signature = self.get_form_signature(url, request).await;
        let url = Url::from_str(&self.api_url(url))?;

        self.body_from_url_as_data(Method::POST, &url, signature)
            .await
    }

    fn parse_body_and_errors<T>(body: &str) -> Result<ResultErrorResponse<T>, ClientError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let result: ResultErrorResponse<T> = serde_json::from_str(body)?;

        if let Some(error) = result.error.first() {
            error
                .try_into()
                .map(|err: KrakenError| Err(ClientError::Kraken(err)))
                .unwrap_or(Ok(result))
        } else {
            Ok(result)
        }
    }

    async fn body_from_url(
        &self,
        method: Method,
        url: &Url,
        request_body: String,
    ) -> Result<String, ClientError> {
        let request = Self::request_builder_from_url(method, url)?
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", self.get_user_agent().as_str())
            .body(request_body)?;

        self.body_from_request(request).await
    }

    async fn body_from_url_and_form_with_auth(
        &mut self,
        method: Method,
        url: &Url,
        signature: Signature,
    ) -> Result<String, ClientError> {
        let request = self.build_form_request(method, url, signature).await?;
        self.body_from_request(request).await
    }

    async fn body_from_url_and_json_with_auth(
        &mut self,
        method: Method,
        url: &Url,
        signature: Signature,
    ) -> Result<String, ClientError> {
        let mut secrets_provider = self.secrets_provider.lock().await;
        let request = Self::request_builder_from_url(method, url)?
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("User-Agent", self.get_user_agent().as_str())
            .header(
                "API-Key",
                secrets_provider.get_secrets().key.expose_secret(),
            )
            .header("API-Sign", signature.signature)
            .body(signature.body_data)?;

        self.body_from_request(request).await
    }

    async fn body_from_url_as_data(
        &mut self,
        method: Method,
        url: &Url,
        signature: Signature,
    ) -> Result<Vec<u8>, ClientError> {
        let request = self.build_form_request(method, url, signature).await?;
        let resp = self.http_client.request(request).await?;

        let status = resp.status();
        let bytes = resp.into_body().collect().await?.to_bytes();

        if !status.is_success() {
            Err(ClientError::HttpStatus(format!(
                "HTTP Status: {}",
                status.as_u16()
            )))
        } else {
            Ok(bytes.to_vec())
        }
    }

    async fn body_from_request(&self, req: Request<String>) -> Result<String, ClientError> {
        let resp = self.http_client.request(req).await?;

        let status = resp.status();
        let bytes = resp.into_body().collect().await?.to_bytes();
        let text = String::from_utf8(bytes.to_vec()).or(Err(ClientError::Parse(
            "Failed to parse bytes from response body.",
        )))?;

        if !status.is_success() {
            Err(ClientError::HttpStatus(text))
        } else {
            if cfg!(feature = "debug-inbound") {
                debug!("Received: {}", text);
            }

            if self.trace_inbound {
                trace!("Received: {}", text);
            }

            Ok(text)
        }
    }

    async fn build_form_request(
        &mut self,
        method: Method,
        url: &Url,
        signature: Signature,
    ) -> Result<Request<String>, ClientError> {
        let mut secrets_provider = self.secrets_provider.lock().await;
        let request = Self::request_builder_from_url(method, url)?
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", self.get_user_agent().as_str())
            .header(
                "API-Key",
                secrets_provider.get_secrets().key.expose_secret(),
            )
            .header("API-Sign", signature.signature)
            .body(signature.body_data)?;
        Ok(request)
    }

    async fn get_form_signature<R>(&mut self, endpoint: &str, request: &R) -> Signature
    where
        R: ToQueryParams,
    {
        let mut secrets_provider = self.secrets_provider.lock().await;
        let mut provider = self.nonce_provider.lock().await;
        let nonce = provider.get_nonce();
        let encoded_data = self.encode_form_request(nonce, request);
        generate_signature(
            nonce,
            secrets_provider.get_secrets().secret.expose_secret(),
            endpoint,
            encoded_data,
        )
    }

    async fn get_json_signature<R>(
        &mut self,
        endpoint: &str,
        request: &R,
    ) -> Result<Signature, ClientError>
    where
        R: Serialize,
    {
        let mut secrets_provider = self.secrets_provider.lock().await;
        let mut nonce_provider = self.nonce_provider.lock().await;
        let nonce = nonce_provider.get_nonce();
        let encoded_data = self.encode_json_request(nonce, request)?;
        Ok(generate_signature(
            nonce,
            secrets_provider.get_secrets().secret.expose_secret(),
            endpoint,
            encoded_data,
        ))
    }

    fn encode_json_request<R>(&self, nonce: u64, request: &R) -> Result<String, ClientError>
    where
        R: Serialize,
    {
        let nonce_request = NonceRequest::new(nonce, request);
        Ok(serde_json::to_string(&nonce_request)?)
    }

    fn encode_form_request<R>(&self, nonce: u64, request: &R) -> String
    where
        R: ToQueryParams,
    {
        let mut query_params = form_urlencoded::Serializer::new(String::new());
        query_params.append_pair("nonce", &nonce.to_string());

        for (key, value) in request.to_query_params().iter() {
            query_params.append_pair(key, value);
        }

        query_params.finish()
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! test_parse_error_matches_pattern {
    ($body: expr, $pattern: pat) => {
        let err = CoreKrakenClient::parse_body_and_errors::<AccountBalances>($body);

        println!("{:?}", err);
        assert!(err.is_err());
        assert!(matches!(err, $pattern));
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::core_kraken_client::CoreKrakenClient;
    use crate::clients::errors::ClientError;
    use crate::clients::errors::KrakenError;
    use crate::crypto::nonce_provider::IncreasingNonceProvider;
    use crate::response_types::AccountBalances;
    use crate::test_core_endpoint;
    use crate::test_data::get_null_secrets_provider;
    use crate::test_data::sub_accounts_json::{
        get_account_transfer_json, get_create_sub_account_json,
    };
    use crate::test_data::trading_response_json::{
        get_add_order_batch_json, get_add_order_json, get_amend_order_json,
        get_cancel_all_orders_after_json, get_cancel_all_orders_json, get_cancel_order_batch_json,
        get_cancel_order_json, get_edit_order_json,
    };
    use crate::test_data::websockets_json::get_websockets_token_json;
    use rust_decimal_macros::dec;
    use serde_json::json;
    use wiremock::matchers::{
        body_partial_json, body_string_contains, header_exists, method, path,
    };
    use wiremock::{Mock, MockServer, ResponseTemplate};

    pub const ERROR_PERMISSION_DENIED: &str = r#"{"error":["EGeneral:Permission denied"]}"#;
    pub const ERROR_INVALID_KEY: &str = r#"{"error":["EAPI:Invalid key"]}"#;
    pub const ERROR_UNKNOWN_ASSET_PAIR: &str = r#"{"error":["EQuery:Unknown asset pair"]}"#;
    pub const ERROR_INVALID_ARGUMENT: &str = r#"{"error":["EGeneral:Invalid arguments:type"]}"#;

    // doc-inferred ones not from true API responses
    pub const ERROR_INVALID_SIGNATURE: &str = r#"{"error":["EAPI:Invalid signature"]}"#;
    pub const ERROR_INVALID_NONCE: &str = r#"{"error":["EAPI:Invalid nonce"]}"#;
    pub const ERROR_INVALID_SESSION: &str = r#"{"error":["ESession:Invalid session"]}"#;
    pub const ERROR_BAD_REQUEST: &str = r#"{"error":["EAPI:Bad request"]}"#;
    pub const ERROR_UNKNOWN_METHOD: &str = r#"{"error":["EGeneral:Unknown Method"]}"#;

    pub const ERROR_API_RATE_LIMIT: &str = r#"{"error":["EAPI:Rate limit exceeded"]}"#;
    pub const ERROR_ORDER_RATE_LIMIT: &str = r#"{"error":["EOrder:Rate limit exceeded"]}"#;
    pub const ERROR_RATE_LIMIT_LOCKOUT: &str = r#"{"error":["EGeneral:Temporary lockout"]}"#;
    pub const ERROR_SERVICE_UNAVAILABLE: &str = r#"{"error":["EService:Unavailable"]}"#;
    pub const ERROR_SERVICE_BUSY: &str = r#"{"error":["EService:Busy"]}"#;
    pub const ERROR_INTERNAL_ERROR: &str = r#"{"error":["EGeneral:Internal error"]}"#;
    pub const ERROR_TRADE_LOCKED: &str = r#"{"error":["ETrade:Locked"]}"#;
    pub const ERROR_FEATURE_DISABLED: &str = r#"{"error":["EAPI:Feature disabled"]}"#;

    #[tokio::test]
    async fn test_create_subaccount() {
        let secrets_provider = get_null_secrets_provider();
        let request =
            CreateSubAccountRequest::builder("username".to_string(), "user@mail.com".to_string())
                .build();

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/0/private/CreateSubaccount"))
            .and(header_exists("User-Agent"))
            .and(header_exists("API-Key"))
            .and(header_exists("API-Sign"))
            .and(body_string_contains("username=username"))
            .and(body_string_contains("email=user%40mail.com"))
            .respond_with(ResponseTemplate::new(200).set_body_json(get_create_sub_account_json()))
            .expect(1)
            .mount(&mock_server)
            .await;

        test_core_endpoint!(secrets_provider, mock_server, create_sub_account, &request);
    }

    #[tokio::test]
    async fn test_account_transfer() {
        let secrets_provider = get_null_secrets_provider();
        let request = AccountTransferRequest::builder(
            "BTC".to_string(),
            dec!(1031.2008),
            "SourceAccount".to_string(),
            "DestAccount".to_string(),
        )
        .build();

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/0/private/AccountTransfer"))
            .and(header_exists("User-Agent"))
            .and(header_exists("API-Key"))
            .and(header_exists("API-Sign"))
            .and(body_string_contains("asset=BTC"))
            .and(body_string_contains("amount=1031.2008"))
            .and(body_string_contains("from=SourceAccount"))
            .and(body_string_contains("to=DestAccount"))
            .respond_with(ResponseTemplate::new(200).set_body_json(get_account_transfer_json()))
            .expect(1)
            .mount(&mock_server)
            .await;

        test_core_endpoint!(secrets_provider, mock_server, account_transfer, &request);
    }

    #[tokio::test]
    async fn test_add_order() {
        let secrets_provider = get_null_secrets_provider();

        let order_flags =
            OrderFlags::new(vec![OrderFlag::NoMarketPriceProtection, OrderFlag::Post]);
        let request = AddOrderRequest::builder(
            OrderType::Market,
            BuySell::Buy,
            dec!(5.0),
            "USDCUSD".to_string(),
        )
        .order_flags(order_flags)
        .price(dec!(0.90))
        .build();

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/0/private/AddOrder"))
            .and(header_exists("User-Agent"))
            .and(header_exists("API-Key"))
            .and(header_exists("API-Sign"))
            .and(body_string_contains("price=0.90"))
            .and(body_string_contains("ordertype=market"))
            .and(body_string_contains("type=buy"))
            .and(body_string_contains("volume=5.0"))
            .and(body_string_contains("pair=USDCUSD"))
            .and(body_string_contains("oflags=nompp%2Cpost"))
            .respond_with(ResponseTemplate::new(200).set_body_json(get_add_order_json()))
            .expect(1)
            .mount(&mock_server)
            .await;

        test_core_endpoint!(secrets_provider, mock_server, add_order, &request);
    }

    #[tokio::test]
    async fn test_add_order_batch() {
        let secrets_provider = get_null_secrets_provider();
        let order_1 = BatchedOrderRequest::builder(OrderType::Limit, BuySell::Buy, dec!(5.1))
            .price(dec!(0.9))
            .start_time("0".to_string())
            .expire_time("+5".to_string())
            .build();

        let order_2 = BatchedOrderRequest::builder(OrderType::Limit, BuySell::Sell, dec!(5.2))
            .price(dec!(0.9))
            .order_flags(vec![OrderFlag::Post])
            .build();

        let request =
            AddBatchedOrderRequest::builder(vec![order_1, order_2], "USDCUSD".to_string()).build();

        let mock_server = MockServer::start().await;

        let expected_json = json!({
            "orders": [
                {"ordertype": "limit", "type": "buy", "volume": "5.1", "price": "0.9", "starttm": "0", "expiretm": "+5"},
                {"ordertype": "limit", "type": "sell", "volume": "5.2", "price": "0.9", "oflags": "post"}
            ],
            "pair":"USDCUSD"
        });

        Mock::given(method("POST"))
            .and(path("/0/private/AddOrderBatch"))
            .and(header_exists("User-Agent"))
            .and(header_exists("API-Key"))
            .and(header_exists("API-Sign"))
            .and(body_partial_json(expected_json))
            .respond_with(ResponseTemplate::new(200).set_body_json(get_add_order_batch_json()))
            .expect(1)
            .mount(&mock_server)
            .await;

        test_core_endpoint!(secrets_provider, mock_server, add_order_batch, &request);
    }

    #[tokio::test]
    async fn test_amend_order() {
        let secrets_provider = get_null_secrets_provider();

        let amend_request = AmendOrderRequest::builder()
            .tx_id("tx-id".to_string())
            .order_quantity(dec!(5.25))
            .limit_price(dec!(0.96).to_string())
            .post_only(true)
            .build();

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/0/private/AmendOrder"))
            .and(header_exists("User-Agent"))
            .and(header_exists("API-Key"))
            .and(header_exists("API-Sign"))
            .and(body_string_contains(r#""txid":"tx-id""#))
            .and(body_string_contains(r#""order_qty":"5.25""#))
            .and(body_string_contains(r#""limit_price":"0.96""#))
            .and(body_string_contains(r#""post_only":true"#))
            .respond_with(ResponseTemplate::new(200).set_body_json(get_amend_order_json()))
            .expect(1)
            .mount(&mock_server)
            .await;

        test_core_endpoint!(secrets_provider, mock_server, amend_order, &amend_request);
    }

    #[tokio::test]
    async fn test_edit_order() {
        let secrets_provider = get_null_secrets_provider();
        let request = EditOrderRequest::builder(
            "7BD466-BKZVM-FT2E2L".to_string(),
            dec!(5.1),
            "USDCUSD".to_string(),
        )
        .price(dec!(0.89))
        .user_ref(1234)
        .build();

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/0/private/EditOrder"))
            .and(header_exists("User-Agent"))
            .and(header_exists("API-Key"))
            .and(header_exists("API-Sign"))
            .and(body_string_contains("price=0.89"))
            .and(body_string_contains("volume=5.1"))
            .and(body_string_contains("userref=1234"))
            .and(body_string_contains("txid=7BD466-BKZVM-FT2E2L"))
            .respond_with(ResponseTemplate::new(200).set_body_json(get_edit_order_json()))
            .expect(1)
            .mount(&mock_server)
            .await;

        test_core_endpoint!(secrets_provider, mock_server, edit_order, &request);
    }

    #[tokio::test]
    async fn test_cancel_order() {
        let secrets_provider = get_null_secrets_provider();

        let txid = IntOrString::String("7BD466-BKZVM-FT2E2L".to_string());
        let request = CancelOrderRequest::builder(txid).build();

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/0/private/CancelOrder"))
            .and(header_exists("User-Agent"))
            .and(header_exists("API-Key"))
            .and(header_exists("API-Sign"))
            .and(body_string_contains("txid=7BD466-BKZVM-FT2E2L"))
            .respond_with(ResponseTemplate::new(200).set_body_json(get_cancel_order_json()))
            .expect(1)
            .mount(&mock_server)
            .await;

        test_core_endpoint!(secrets_provider, mock_server, cancel_order, &request);
    }

    #[tokio::test]
    async fn test_cancel_all_orders() {
        let secrets_provider = get_null_secrets_provider();

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/0/private/CancelAll"))
            .and(header_exists("User-Agent"))
            .and(header_exists("API-Key"))
            .and(header_exists("API-Sign"))
            .respond_with(ResponseTemplate::new(200).set_body_json(get_cancel_all_orders_json()))
            .expect(1)
            .mount(&mock_server)
            .await;

        test_core_endpoint!(secrets_provider, mock_server, cancel_all_orders);
    }

    #[tokio::test]
    async fn test_cancel_all_orders_after() {
        let secrets_provider = get_null_secrets_provider();

        let request = CancelAllOrdersAfterRequest::builder(180).build();

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/0/private/CancelAllOrdersAfter"))
            .and(header_exists("User-Agent"))
            .and(header_exists("API-Key"))
            .and(header_exists("API-Sign"))
            .and(body_string_contains("timeout=180"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(get_cancel_all_orders_after_json()),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        test_core_endpoint!(
            secrets_provider,
            mock_server,
            cancel_all_orders_after,
            &request
        );
    }

    #[tokio::test]
    async fn test_cancel_order_batch() {
        let secrets_provider = get_null_secrets_provider();
        let tx_ids = vec![
            "OZICHZ-FGB63-156I4K".to_string(),
            "BEGNMD-FEJKF-VC6U8Y".to_string(),
        ];
        let request = CancelBatchOrdersRequest::from_tx_ids(tx_ids);

        let mock_server = MockServer::start().await;

        let expected_json = json!({
            "orders": ["OZICHZ-FGB63-156I4K", "BEGNMD-FEJKF-VC6U8Y"],
        });

        Mock::given(method("POST"))
            .and(path("/0/private/CancelOrderBatch"))
            .and(header_exists("User-Agent"))
            .and(header_exists("API-Key"))
            .and(header_exists("API-Sign"))
            .and(body_partial_json(expected_json))
            .respond_with(ResponseTemplate::new(200).set_body_json(get_cancel_order_batch_json()))
            .expect(1)
            .mount(&mock_server)
            .await;

        test_core_endpoint!(secrets_provider, mock_server, cancel_order_batch, &request);
    }

    #[tokio::test]
    async fn test_get_websockets_token() {
        let secrets_provider = get_null_secrets_provider();
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/0/private/GetWebSocketsToken"))
            .respond_with(ResponseTemplate::new(200).set_body_json(get_websockets_token_json()))
            .expect(1)
            .mount(&mock_server)
            .await;

        test_core_endpoint!(secrets_provider, mock_server, get_websockets_token);
    }

    #[test]
    fn test_parse_body_and_errors() {
        test_parse_error_matches_pattern!(
            ERROR_PERMISSION_DENIED,
            Err(ClientError::Kraken(KrakenError::PermissionDenied))
        );

        test_parse_error_matches_pattern!(
            ERROR_INVALID_KEY,
            Err(ClientError::Kraken(KrakenError::InvalidKey))
        );

        test_parse_error_matches_pattern!(
            ERROR_UNKNOWN_ASSET_PAIR,
            Err(ClientError::Kraken(KrakenError::UnknownAssetPair))
        );

        test_parse_error_matches_pattern!(
            ERROR_INVALID_ARGUMENT,
            Err(ClientError::Kraken(KrakenError::InvalidArguments(..)))
        );

        test_parse_error_matches_pattern!(
            ERROR_INVALID_SIGNATURE,
            Err(ClientError::Kraken(KrakenError::InvalidSignature))
        );

        test_parse_error_matches_pattern!(
            ERROR_INVALID_NONCE,
            Err(ClientError::Kraken(KrakenError::InvalidNonce))
        );

        test_parse_error_matches_pattern!(
            ERROR_INVALID_SESSION,
            Err(ClientError::Kraken(KrakenError::InvalidSession))
        );

        test_parse_error_matches_pattern!(
            ERROR_BAD_REQUEST,
            Err(ClientError::Kraken(KrakenError::BadRequest))
        );

        test_parse_error_matches_pattern!(
            ERROR_UNKNOWN_METHOD,
            Err(ClientError::Kraken(KrakenError::UnknownMethod))
        );

        test_parse_error_matches_pattern!(
            ERROR_API_RATE_LIMIT,
            Err(ClientError::Kraken(KrakenError::RateLimitExceeded))
        );

        test_parse_error_matches_pattern!(
            ERROR_ORDER_RATE_LIMIT,
            Err(ClientError::Kraken(KrakenError::TradingRateLimitExceeded))
        );

        test_parse_error_matches_pattern!(
            ERROR_RATE_LIMIT_LOCKOUT,
            Err(ClientError::Kraken(KrakenError::TemporaryLockout))
        );

        test_parse_error_matches_pattern!(
            ERROR_SERVICE_UNAVAILABLE,
            Err(ClientError::Kraken(KrakenError::ServiceUnavailable))
        );

        test_parse_error_matches_pattern!(
            ERROR_SERVICE_BUSY,
            Err(ClientError::Kraken(KrakenError::ServiceBusy))
        );

        test_parse_error_matches_pattern!(
            ERROR_INTERNAL_ERROR,
            Err(ClientError::Kraken(KrakenError::InternalError))
        );

        test_parse_error_matches_pattern!(
            ERROR_TRADE_LOCKED,
            Err(ClientError::Kraken(KrakenError::TradeLocked))
        );

        test_parse_error_matches_pattern!(
            ERROR_FEATURE_DISABLED,
            Err(ClientError::Kraken(KrakenError::FeatureDisabled))
        );
    }
}
