use std::time::Duration;

use kobe_client::client_builder::KobeApiClientBuilder;

#[tokio::main]
async fn main() {
    let client = KobeApiClientBuilder::new()
        .timeout(Duration::from_secs(45))
        .retry(true)
        .max_retries(5)
        .build();

    let stake_over_time = client.get_jito_stake_over_time().await.unwrap();

    println!("Stake over time: {:?}", stake_over_time);
}
