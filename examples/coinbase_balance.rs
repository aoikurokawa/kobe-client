use std::time::Duration;

use kobe_client::client_builder::KobeApiClientBuilder;

#[tokio::main]
async fn main() {
    let client = KobeApiClientBuilder::new()
        .base_url("https://kobe.testnet.jito.network")
        .timeout(Duration::from_secs(45))
        .retry(true)
        .max_retries(5)
        .build();

    let epoch = 970;

    let coinbase_balance = client.get_coinbase_balance(epoch).await.unwrap();

    println!("Coinbase Balance: {:?}", coinbase_balance);
}
