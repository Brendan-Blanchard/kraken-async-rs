use crate::clients::rate_limited_kraken_client::RateLimitedKrakenClient;
use crate::test_data::test_client_impl::TestClient;

pub type TestRateLimitedClient = RateLimitedKrakenClient<TestClient>;
