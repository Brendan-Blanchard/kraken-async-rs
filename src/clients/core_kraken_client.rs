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
use tracing::debug;
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
///     let secrets_provider = Box::new(StaticSecretsProvider::new("", ""));
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
pub struct CoreKrakenClient {
    pub api_url: String,
    secrets_provider: Box<dyn SecretsProvider>,
    nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
    http_client: Client<HttpsConnector<HttpConnector>, String>,
    user_agent: Option<String>,
}

impl KrakenClient for CoreKrakenClient {
    fn new(
        secrets_provider: Box<dyn SecretsProvider>,
        nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>>,
    ) -> Self {
        let https = HttpsConnector::new();
        let http_client: Client<HttpsConnector<HttpConnector>, String> =
            Client::builder(TokioExecutor::new()).build(https);
        CoreKrakenClient {
            api_url: KRAKEN_BASE_URL.into(),
            secrets_provider,
            nonce_provider,
            http_client,
            user_agent: None,
        }
    }

    fn new_with_url(
        secrets_provider: Box<dyn SecretsProvider>,
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
        }
    }

    async fn set_user_agent(&mut self, user_agent: String) {
        self.user_agent = Some(user_agent);
    }

    #[tracing::instrument(skip(self))]
    async fn get_server_time(&mut self) -> Result<ResultErrorResponse<SystemTime>, ClientError> {
        let url = Url::from_str(&self.api_url(TIME_ENDPOINT))?;
        let body = self.body_from_url(Method::GET, &url, "".into()).await?;
        Ok(serde_json::from_str(&body)?)
    }

    #[tracing::instrument(skip(self))]
    async fn get_system_status(
        &mut self,
    ) -> Result<ResultErrorResponse<SystemStatusInfo>, ClientError> {
        let url = Url::from_str(&self.api_url(STATUS_ENDPOINT))?;
        let body = self.body_from_url(Method::GET, &url, "".into()).await?;
        Ok(serde_json::from_str(&body)?)
    }

    #[tracing::instrument(skip(self))]
    async fn get_asset_info(
        &mut self,
        request: &AssetInfoRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, AssetInfo>>, ClientError> {
        self.public_get(ASSET_INFO_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_tradable_asset_pairs(
        &mut self,
        request: &TradableAssetPairsRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, TradableAssetPair>>, ClientError> {
        self.public_get(TRADABLE_ASSET_PAIRS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_ticker_information(
        &mut self,
        request: &TickerRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, RestTickerInfo>>, ClientError> {
        self.public_get(TICKER_INFO_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_ohlc(
        &mut self,
        request: &OHLCRequest,
    ) -> Result<ResultErrorResponse<OhlcResponse>, ClientError> {
        self.public_get(OHLC_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_orderbook(
        &mut self,
        request: &OrderbookRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, Orderbook>>, ClientError> {
        self.public_get(ORDER_BOOK_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_recent_trades(
        &mut self,
        request: &RecentTradesRequest,
    ) -> Result<ResultErrorResponse<RecentTrades>, ClientError> {
        self.public_get(RECENT_TRADES_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_recent_spreads(
        &mut self,
        request: &RecentSpreadsRequest,
    ) -> Result<ResultErrorResponse<RecentSpreads>, ClientError> {
        self.public_get(RECENT_SPREADS_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_account_balance(
        &mut self,
    ) -> Result<ResultErrorResponse<AccountBalances>, ClientError> {
        self.private_form_post(ACCOUNT_BALANCE_ENDPOINT, &EmptyRequest::default())
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_extended_balances(
        &mut self,
    ) -> Result<ResultErrorResponse<ExtendedBalances>, ClientError> {
        self.private_form_post(ACCOUNT_BALANCE_EXTENDED_ENDPOINT, &EmptyRequest::default())
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_trade_balances(
        &mut self,
        request: &TradeBalanceRequest,
    ) -> Result<ResultErrorResponse<TradeBalances>, ClientError> {
        self.private_form_post(TRADE_BALANCE_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_open_orders(
        &mut self,
        request: &OpenOrdersRequest,
    ) -> Result<ResultErrorResponse<OpenOrders>, ClientError> {
        self.private_form_post(OPEN_ORDERS_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_closed_orders(
        &mut self,
        request: &ClosedOrdersRequest,
    ) -> Result<ResultErrorResponse<ClosedOrders>, ClientError> {
        self.private_form_post(CLOSED_ORDERS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn query_orders_info(
        &mut self,
        request: &OrderRequest,
    ) -> Result<ResultErrorResponse<HashMap<String, Order>>, ClientError> {
        self.private_form_post(QUERY_ORDERS_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_trades_history(
        &mut self,
        request: &TradesHistoryRequest,
    ) -> Result<ResultErrorResponse<TradesHistory>, ClientError> {
        self.private_form_post(TRADES_HISTORY_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn query_trades_info(
        &mut self,
        request: &TradeInfoRequest,
    ) -> Result<ResultErrorResponse<TradesInfo>, ClientError> {
        self.private_form_post(QUERY_TRADES_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_open_positions(
        &mut self,
        request: &OpenPositionsRequest,
    ) -> Result<ResultErrorResponse<OpenPositions>, ClientError> {
        self.private_form_post(OPEN_POSITIONS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_ledgers_info(
        &mut self,
        request: &LedgersInfoRequest,
    ) -> Result<ResultErrorResponse<LedgerInfo>, ClientError> {
        self.private_form_post(LEDGERS_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn query_ledgers(
        &mut self,
        request: &QueryLedgerRequest,
    ) -> Result<ResultErrorResponse<QueryLedgerInfo>, ClientError> {
        self.private_form_post(QUERY_LEDGERS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_trade_volume(
        &mut self,
        request: &TradeVolumeRequest,
    ) -> Result<ResultErrorResponse<TradeVolume>, ClientError> {
        self.private_form_post(TRADE_VOLUME_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn request_export_report(
        &mut self,
        request: &ExportReportRequest,
    ) -> Result<ResultErrorResponse<ExportReport>, ClientError> {
        self.private_form_post(ADD_EXPORT_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_export_report_status(
        &mut self,
        request: &ExportReportStatusRequest,
    ) -> Result<ResultErrorResponse<Vec<ExportReportStatus>>, ClientError> {
        self.private_form_post(EXPORT_STATUS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn retrieve_export_report(
        &mut self,
        request: &RetrieveExportReportRequest,
    ) -> Result<Vec<u8>, ClientError> {
        self.private_post_binary::<RetrieveExportReportRequest>(RETRIEVE_EXPORT_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn delete_export_report(
        &mut self,
        request: &DeleteExportRequest,
    ) -> Result<ResultErrorResponse<DeleteExportReport>, ClientError> {
        self.private_form_post(REMOVE_EXPORT_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn add_order(
        &mut self,
        request: &AddOrderRequest,
    ) -> Result<ResultErrorResponse<AddOrder>, ClientError> {
        self.private_form_post(ADD_ORDER_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn add_order_batch(
        &mut self,
        request: &AddBatchedOrderRequest,
    ) -> Result<ResultErrorResponse<AddOrderBatch>, ClientError> {
        self.private_json_post(ADD_ORDER_BATCH_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn edit_order(
        &mut self,
        request: &EditOrderRequest,
    ) -> Result<ResultErrorResponse<OrderEdit>, ClientError> {
        self.private_form_post(EDIT_ORDER_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn cancel_order(
        &mut self,
        request: &CancelOrderRequest,
    ) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        self.private_form_post(CANCEL_ORDER_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn cancel_all_orders(&mut self) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        self.private_form_post(CANCEL_ALL_ORDERS_ENDPOINT, &EmptyRequest::default())
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn cancel_all_orders_after(
        &mut self,
        request: &CancelAllOrdersAfterRequest,
    ) -> Result<ResultErrorResponse<CancelAllOrdersAfter>, ClientError> {
        self.private_form_post(CANCEL_ALL_ORDERS_AFTER_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn cancel_order_batch(
        &mut self,
        request: &CancelBatchOrdersRequest,
    ) -> Result<ResultErrorResponse<CancelOrder>, ClientError> {
        self.private_json_post(CANCEL_ORDER_BATCH_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_deposit_methods(
        &mut self,
        request: &DepositMethodsRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositMethod>>, ClientError> {
        self.private_form_post(DEPOSIT_METHODS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_deposit_addresses(
        &mut self,
        request: &DepositAddressesRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositAddress>>, ClientError> {
        self.private_form_post(DEPOSIT_ADDRESSES_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_status_of_recent_deposits(
        &mut self,
        request: &StatusOfDepositWithdrawRequest,
    ) -> Result<ResultErrorResponse<DepositWithdrawResponse>, ClientError> {
        self.private_form_post(DEPOSIT_STATUS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_withdrawal_methods(
        &mut self,
        request: &WithdrawalMethodsRequest,
    ) -> Result<ResultErrorResponse<Vec<WithdrawMethod>>, ClientError> {
        self.private_form_post(WITHDRAW_METHODS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_withdrawal_addresses(
        &mut self,
        request: &WithdrawalAddressesRequest,
    ) -> Result<ResultErrorResponse<Vec<WithdrawalAddress>>, ClientError> {
        self.private_form_post(WITHDRAW_ADDRESSES_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_withdrawal_info(
        &mut self,
        request: &WithdrawalInfoRequest,
    ) -> Result<ResultErrorResponse<Withdrawal>, ClientError> {
        self.private_form_post(WITHDRAW_INFO_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn withdraw_funds(
        &mut self,
        request: &WithdrawFundsRequest,
    ) -> Result<ResultErrorResponse<ConfirmationRefId>, ClientError> {
        self.private_form_post(WITHDRAW_ENDPOINT, request).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_status_of_recent_withdrawals(
        &mut self,
        request: &StatusOfDepositWithdrawRequest,
    ) -> Result<ResultErrorResponse<Vec<DepositWithdrawal>>, ClientError> {
        self.private_form_post(WITHDRAW_STATUS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn request_withdrawal_cancellation(
        &mut self,
        request: &WithdrawCancelRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        self.private_form_post(WITHDRAW_CANCEL_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn request_wallet_transfer(
        &mut self,
        request: &WalletTransferRequest,
    ) -> Result<ResultErrorResponse<ConfirmationRefId>, ClientError> {
        self.private_form_post(WALLET_TRANSFER_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn create_sub_account(
        &mut self,
        request: &CreateSubAccountRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        self.private_form_post(CREATE_SUB_ACCOUNT_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn account_transfer(
        &mut self,
        request: &AccountTransferRequest,
    ) -> Result<ResultErrorResponse<AccountTransfer>, ClientError> {
        self.private_form_post(ACCOUNT_TRANSFER_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn allocate_earn_funds(
        &mut self,
        request: &AllocateEarnFundsRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        self.private_form_post(EARN_ALLOCATE_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn deallocate_earn_funds(
        &mut self,
        request: &AllocateEarnFundsRequest,
    ) -> Result<ResultErrorResponse<bool>, ClientError> {
        self.private_form_post(EARN_DEALLOCATE_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_earn_allocation_status(
        &mut self,
        request: &EarnAllocationStatusRequest,
    ) -> Result<ResultErrorResponse<AllocationStatus>, ClientError> {
        self.private_form_post(EARN_ALLOCATE_STATUS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_earn_deallocation_status(
        &mut self,
        request: &EarnAllocationStatusRequest,
    ) -> Result<ResultErrorResponse<AllocationStatus>, ClientError> {
        self.private_form_post(EARN_DEALLOCATE_STATUS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn list_earn_strategies(
        &mut self,
        request: &ListEarnStrategiesRequest,
    ) -> Result<ResultErrorResponse<EarnStrategies>, ClientError> {
        self.private_form_post(EARN_STRATEGIES_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn list_earn_allocations(
        &mut self,
        request: &ListEarnAllocationsRequest,
    ) -> Result<ResultErrorResponse<EarnAllocations>, ClientError> {
        self.private_form_post(EARN_ALLOCATIONS_ENDPOINT, request)
            .await
    }

    #[tracing::instrument(skip(self))]
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
        let request = self.build_form_request(method, url, signature)?;
        self.body_from_request(request).await
    }

    async fn body_from_url_and_json_with_auth(
        &mut self,
        method: Method,
        url: &Url,
        signature: Signature,
    ) -> Result<String, ClientError> {
        let request = Self::request_builder_from_url(method, url)?
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("User-Agent", self.get_user_agent().as_str())
            .header(
                "API-Key",
                self.secrets_provider.get_secrets().key.expose_secret(),
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
        let request = self.build_form_request(method, url, signature)?;
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
            debug!("Received: {}", text);
            Ok(text)
        }
    }

    fn build_form_request(
        &mut self,
        method: Method,
        url: &Url,
        signature: Signature,
    ) -> Result<Request<String>, ClientError> {
        let request = Self::request_builder_from_url(method, url)?
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", self.get_user_agent().as_str())
            .header(
                "API-Key",
                self.secrets_provider.get_secrets().key.expose_secret(),
            )
            .header("API-Sign", signature.signature)
            .body(signature.body_data)?;
        Ok(request)
    }

    async fn get_form_signature<R>(&mut self, endpoint: &str, request: &R) -> Signature
    where
        R: ToQueryParams,
    {
        let mut provider = self.nonce_provider.lock().await;
        let nonce = provider.get_nonce();
        let encoded_data = self.encode_form_request(nonce, request);
        generate_signature(
            nonce,
            self.secrets_provider.get_secrets().secret.expose_secret(),
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
        let mut provider = self.nonce_provider.lock().await;
        let nonce = provider.get_nonce();
        let encoded_data = self.encode_json_request(nonce, request)?;
        Ok(generate_signature(
            nonce,
            self.secrets_provider.get_secrets().secret.expose_secret(),
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
    use crate::clients::core_kraken_client::CoreKrakenClient;
    use crate::clients::errors::ClientError;
    use crate::clients::errors::KrakenError;
    use crate::response_types::AccountBalances;

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
