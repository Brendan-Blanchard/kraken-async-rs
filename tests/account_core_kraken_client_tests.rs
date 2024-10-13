use crate::resources::test_auth::get_null_secrets_provider;

mod resources;

use crate::resources::kraken_responses::account_response_json::{
    get_account_balance_json, get_closed_orders_json, get_delete_export_report_json,
    get_export_report_response, get_export_report_status_json, get_extended_balance_json,
    get_ledgers_info_json, get_open_orders_json, get_open_positions_json,
    get_open_positions_json_do_calc_optional_fields, get_order_amends_json, get_query_ledgers_json,
    get_query_order_info_json, get_query_trades_info_json, get_request_export_report_json,
    get_trade_balance_json, get_trade_volume_json, get_trades_history_json,
};
use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::{
    ClosedOrdersRequestBuilder, DeleteExportRequest, DeleteExportType, ExportReportRequest,
    ExportReportStatusRequest, LedgersInfoRequest, OpenOrdersRequest, OpenPositionsRequest,
    OrderAmendsRequest, OrderRequest, QueryLedgerRequest, ReportFormatType, ReportType,
    RetrieveExportReportRequest, StringCSV, TradeBalanceRequest, TradeInfoRequest,
    TradeVolumeRequest, TradesHistoryRequest,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use wiremock::http::Method;
use wiremock::matchers::{body_string_contains, header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_account_balance() {
    let secrets_provider = get_null_secrets_provider();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/Balance"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_account_balance_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_account_balance);
}

#[tokio::test]
async fn test_get_extended_balance() {
    let secrets_provider = get_null_secrets_provider();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/BalanceEx"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_extended_balance_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_extended_balances);
}

#[tokio::test]
async fn test_get_trade_balance() {
    let secrets_provider = get_null_secrets_provider();
    let request = TradeBalanceRequest::builder()
        .asset("XXBTZUSD".to_string())
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/TradeBalance"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_trade_balance_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_trade_balances, &request);
}

#[tokio::test]
async fn test_get_open_orders() {
    let secrets_provider = get_null_secrets_provider();
    let request = OpenOrdersRequest::builder()
        .trades(true)
        .client_order_id("some-uuid".to_string())
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/OpenOrders"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("trades=true"))
        .and(body_string_contains("cl_ord_id=some-uuid"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_open_orders_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_open_orders, &request);
}

#[tokio::test]
async fn test_get_closed_orders() {
    let secrets_provider = get_null_secrets_provider();
    let request = ClosedOrdersRequestBuilder::new()
        .trades(true)
        .start(12340000)
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/ClosedOrders"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("trades=true"))
        .and(body_string_contains("start=12340000"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_closed_orders_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_closed_orders, &request);
}

#[tokio::test]
async fn test_query_orders_info() {
    let secrets_provider = get_null_secrets_provider();

    let tx_ids = StringCSV::new(vec!["uuid_1".to_string(), "uuid_2".to_string()]);

    let request = OrderRequest::builder(tx_ids)
        .trades(true)
        .consolidate_taker(false)
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/QueryOrders"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("trades=true"))
        .and(body_string_contains("consolidate_taker=false"))
        // comma-delimited and url-encoded, "," -> "%2C"
        .and(body_string_contains("txid=uuid_1%2Cuuid_2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_query_order_info_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, query_orders_info, &request);
}

#[tokio::test]
async fn test_get_order_amends() {
    let secrets_provider = get_null_secrets_provider();

    let request = OrderAmendsRequest::builder("some-tx-id".to_string()).build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/OrderAmends"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains(r#""order_id":"some-tx-id""#))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_order_amends_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_order_amends, &request);
}

#[tokio::test]
async fn test_get_trades_history() {
    let secrets_provider = get_null_secrets_provider();
    let request = TradesHistoryRequest::builder()
        .start(0)
        .end(1234)
        .trades(true)
        .ledgers(true)
        .consolidate_taker(false)
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/TradesHistory"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("trades=true"))
        .and(body_string_contains("consolidate_taker=false"))
        .and(body_string_contains("ledgers=true"))
        .and(body_string_contains("start=0"))
        .and(body_string_contains("end=1234"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_trades_history_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_trades_history, &request);
}

#[tokio::test]
async fn test_query_trades_info() {
    let secrets_provider = get_null_secrets_provider();

    let tx_ids = StringCSV::new(vec!["some-unique-id".to_string()]);

    let request = TradeInfoRequest::builder(tx_ids).trades(true).build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/QueryTrades"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("txid=some-unique-id"))
        .and(body_string_contains("trades=true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_query_trades_info_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, query_trades_info, &request);
}

#[tokio::test]
async fn test_get_open_positions() {
    let secrets_provider = get_null_secrets_provider();
    let request = OpenPositionsRequest::builder()
        .do_calcs(true)
        .consolidation("market".to_string())
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/OpenPositions"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("docalcs=true"))
        .and(body_string_contains("consolidation=market"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_open_positions_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_open_positions, &request);
}

#[tokio::test]
async fn test_get_open_positions_do_calc_optional_fields() {
    let secrets_provider = get_null_secrets_provider();
    let request = OpenPositionsRequest::builder().do_calcs(false).build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/OpenPositions"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("docalcs=false"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(get_open_positions_json_do_calc_optional_fields()),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_open_positions, &request);
}

#[tokio::test]
async fn test_get_ledgers_info() {
    let secrets_provider = get_null_secrets_provider();

    let assets = StringCSV(vec!["ETH".into(), "BTC".into()]);

    let request = LedgersInfoRequest::builder().start(0).asset(assets).build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/Ledgers"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("start=0"))
        .and(body_string_contains("asset=ETH%2CBTC"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_ledgers_info_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_ledgers_info, &request);
}

#[tokio::test]
async fn test_query_ledgers() {
    let secrets_provider = get_null_secrets_provider();

    let ids = StringCSV(vec![
        "5SF4EW-YDZWO-BB2Q63".into(),
        "4JIKSC-VCT2L-8V13T8".into(),
        "GJZ3K2-4TNMP-DD1C59".into(),
    ]);

    let request = QueryLedgerRequest::builder(ids.clone())
        .trades(true)
        .build();

    let expected_ids = format!("id={}", ids.0.join("%2C"));

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/QueryLedgers"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("trades=true"))
        .and(body_string_contains(expected_ids.as_str()))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_query_ledgers_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, query_ledgers, &request);
}

#[tokio::test]
async fn test_get_trade_volume() {
    let secrets_provider = get_null_secrets_provider();

    let pairs = StringCSV(vec!["XXBTZUSD".into(), "XETHXXBT".into()]);

    let request = TradeVolumeRequest::builder().pair(pairs.clone()).build();

    let expected_pairs = pairs.0.join("%2C");

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/TradeVolume"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains(expected_pairs))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_trade_volume_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(secrets_provider, mock_server, get_trade_volume, &request);
}

#[tokio::test]
async fn test_request_export_report() {
    let secrets_provider = get_null_secrets_provider();
    let request = ExportReportRequest::builder(ReportType::Ledgers, "TestExport".to_string())
        .format(ReportFormatType::Csv)
        .build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/AddExport"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("report=ledgers"))
        .and(body_string_contains("description=TestExport"))
        .and(body_string_contains("format=CSV"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_request_export_report_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        request_export_report,
        &request
    );
}

#[tokio::test]
async fn test_get_export_report_status() {
    let secrets_provider = get_null_secrets_provider();
    let request = ExportReportStatusRequest::builder(ReportType::Trades).build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/ExportStatus"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("report=trades"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_export_report_status_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        get_export_report_status,
        &request
    );
}

#[tokio::test]
async fn test_retrieve_export_report() {
    let secrets_provider = get_null_secrets_provider();
    let request = RetrieveExportReportRequest::builder("HI1M0S-BCRBJ-P01V9R".to_string()).build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/RetrieveExport"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("id=HI1M0S-BCRBJ-P01V9R"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(get_export_report_response()))
        .expect(1)
        .mount(&mock_server)
        .await;

    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
    let mut client =
        CoreKrakenClient::new_with_url(secrets_provider, nonce_provider, mock_server.uri());

    let resp = client.retrieve_export_report(&request).await;

    mock_server.verify().await;
    assert!(resp.is_ok());
    assert_eq!(get_export_report_response(), resp.unwrap());
}

#[tokio::test]
async fn test_delete_export_report() {
    let secrets_provider = get_null_secrets_provider();
    let request =
        DeleteExportRequest::builder("54E7".to_string(), DeleteExportType::Delete).build();

    let mock_server = MockServer::start().await;

    Mock::given(method(Method::POST))
        .and(path("/0/private/RemoveExport"))
        .and(header_exists("User-Agent"))
        .and(header_exists("API-Key"))
        .and(header_exists("API-Sign"))
        .and(body_string_contains("id=54E7"))
        .and(body_string_contains("type=delete"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_delete_export_report_json()))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_core_endpoint!(
        secrets_provider,
        mock_server,
        delete_export_report,
        &request
    );
}
