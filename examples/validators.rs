use std::time::Duration;

use kobe_client::client_builder::KobeApiClientBuilder;

#[tokio::main]
async fn main() {
    let client = KobeApiClientBuilder::new()
        .timeout(Duration::from_secs(45))
        .retry(true)
        .max_retries(5)
        .build();

    let current_epoch = client.get_current_epoch().await.unwrap();
    println!("   Current epoch: {}\n", current_epoch);

    let validators = client.get_validators(Some(current_epoch)).await.unwrap();

    println!("Validators: {validators:?}");
}
