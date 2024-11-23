/// Takes:
/// -   SecretsProvider
/// -   MockServer
/// -   Endpoint ident (e.g. "get_account_balance")
/// -   Args to provide (e.g. a request)
#[macro_export]
macro_rules! test_core_endpoint {
    ($secrets_provider:expr, $mock_server:expr, $endpoint_call:ident $(, $args:expr)*) => {
        let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> = Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));
        let mut client = CoreKrakenClient::new_with_url($secrets_provider, nonce_provider, $mock_server.uri());

        let resp = client.$endpoint_call($($args),*).await;

        println!("{:?}", resp);
        $mock_server.verify().await;
        assert!(resp.is_ok());
        assert!(resp.unwrap().result.is_some());
    };
}

/// Takes:
/// - endpoint_call: ident - of the method to call
/// - n_calls: number of calls to make  
/// - low_expectation: lower bound on number of seconds it's expected to take
/// - high_expectation: upper bound on number of seconds it's expected to take
/// - verification: user's verification tier (can affect starting rate and replenishment)
/// - args, any args that must be applied to the endpoint, eg `client.$endpoint_call($($args),*)`
#[macro_export]
macro_rules! test_rate_limited_endpoint {
    ($endpoint_call:ident, $n_calls:expr, $low_expectation: expr, $high_expectation: expr, $verification:expr $(, $args:expr)*) => {
        let start = Instant::now();

        let secrets_provider = get_null_secrets_provider();
        let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> = Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));

        let mut client: TestRateLimitedClient =
            RateLimitedKrakenClient::new_with_verification_tier(
                secrets_provider,
                nonce_provider,
                $verification,
            );

        for _ in 0..$n_calls {
            let _ = client.$endpoint_call($($args),*).await;
        }

        let end = Instant::now();
        let elapsed = end - start;

        println!("{:?}", elapsed);

        assert!(elapsed > Duration::from_secs($low_expectation));
        assert!(elapsed < Duration::from_secs($high_expectation));
    };
}
