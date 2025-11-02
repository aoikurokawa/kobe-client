use std::time::Duration;

use chrono::Utc;
use kobe_client::{client_builder::KobeApiClientBuilder, types::StakePoolStatsRequest};

#[tokio::main]
async fn main() {
    let client = KobeApiClientBuilder::new()
        .timeout(Duration::from_secs(45))
        .retry(true)
        .max_retries(5)
        .build();

    let current_epoch = client.get_current_epoch().await.unwrap();
    println!("Current epoch: {}\n", current_epoch);

    let end = Utc::now();

    let day = 60 * 60 * 24;
    let start = end - Duration::from_secs(day);
    let request = StakePoolStatsRequest::new()
        .with_bucket_type("Daily")
        .with_range_filter(start, end)
        .sort_asc();

    let stake_pool_stats = client.get_stake_pool_stats(Some(&request)).await.unwrap();

    println!("Stake Pool Stats: {:?}", stake_pool_stats);
}
