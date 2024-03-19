use kraken_async_rs::clients::core_kraken_client::CoreKrakenClient;
use kraken_async_rs::clients::kraken_client::KrakenClient;
use kraken_async_rs::clients::rate_limited_kraken_client::RateLimitedKrakenClient;
use kraken_async_rs::crypto::nonce_provider::{IncreasingNonceProvider, NonceProvider};
use kraken_async_rs::request_types::RecentTradesRequest;
use kraken_async_rs::response_types::RecentTrade;
use kraken_async_rs::secrets::secrets_provider::StaticSecretsProvider;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use time::{Duration, OffsetDateTime};
use tokio::sync::Mutex;

const MAX_RECENT_TRADES: i64 = 1000;

/// Retrieve all trades for a pair iteratively by using Kraken's pagination (since, count) parameters
///
/// This creates a RateLimitedKrakenClient to proactively avoid getting rate-limited by the API.
/// The first request is made, then subsequent ones update the `since` parameter until no new trades
/// are seen.
///
/// The result is verified by checking that number of records retrieved is the same as the number
/// of unique ids retrieved.
#[tokio::main]
async fn main() {
    let secrets_provider = Box::new(StaticSecretsProvider::new("", ""));
    let nonce_provider: Box<Arc<Mutex<dyn NonceProvider>>> =
        Box::new(Arc::new(Mutex::new(IncreasingNonceProvider::new())));

    let mut client: RateLimitedKrakenClient<CoreKrakenClient> =
        RateLimitedKrakenClient::new(secrets_provider, nonce_provider);

    let since = OffsetDateTime::now_utc()
        .checked_sub(Duration::hours(1))
        .unwrap()
        .unix_timestamp();

    // get recent trades starting 1 hour ago in blocks of 1000 (max Kraken allows)
    let request = RecentTradesRequest::builder("XXBTZUSD".to_string())
        .since(since)
        .count(MAX_RECENT_TRADES)
        .build();

    let mut results: HashMap<String, Vec<RecentTrade>> = HashMap::new();
    let mut last = request.since;

    // keep retrieving and adding to results until no new data is seen
    loop {
        let mut request = request.clone();
        request.since = last;

        let result = client.get_recent_trades(&request).await.unwrap().result;

        last = result.as_ref().map(|o| o.last);

        if let Some(data) = result {
            let mut no_new_data = false;
            for (pair, mut trades) in data.trades {
                if (trades.len() as i64) < request.count.unwrap_or(MAX_RECENT_TRADES) {
                    no_new_data = true;
                }
                results.entry(pair).or_default().append(&mut trades);
            }

            if no_new_data {
                break;
            }
        } else {
            break;
        }
    }

    let trades = results.get("XXBTZUSD").unwrap();

    println!("{}", trades.len());

    let trade_ids: HashSet<i64> = trades.iter().map(|trade| trade.trade_id).collect();

    // expect that the count of unique trades is identical to the number of trades retrieved by pagination
    assert_eq!(trade_ids.len(), trades.len());
}
