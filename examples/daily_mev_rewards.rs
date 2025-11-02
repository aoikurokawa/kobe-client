use std::time::Duration;

use kobe_client::client_builder::KobeApiClientBuilder;

#[tokio::main]
async fn main() {
    let client = KobeApiClientBuilder::new()
        .timeout(Duration::from_secs(45))
        .retry(true)
        .max_retries(5)
        .build();

    let daily_mev_tips = client.get_daily_mev_rewards().await.unwrap();

    println!("Daily MEV Tips: {:?}", daily_mev_tips);
}
