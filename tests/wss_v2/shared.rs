use kraken_async_rs::wss::v2::kraken_wss_client::KrakenWSSClient;
use ws_mock::ws_mock_server::WsMockServer;

pub struct WssTestState {
    pub mock_server: WsMockServer,
    pub ws_client: KrakenWSSClient,
}

impl WssTestState {
    pub async fn new() -> Self {
        let mock_server = WsMockServer::start().await;
        let server_uri = mock_server.uri().await;
        let ws_client = KrakenWSSClient::new_with_urls(server_uri.clone(), server_uri);
        WssTestState {
            mock_server,
            ws_client,
        }
    }
}
