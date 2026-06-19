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
    println!("Current epoch: {}\n", current_epoch);

    let validators = client.get_validators(Some(989)).await.unwrap();

    println!("Found {} validators", validators.validators.len());
    println!("Found {:?}", validators.validators[0]);

    let mut count = 0;
    for validator in validators.validators {
        if validator.bam_connection_rate.is_some() {
            count += 1;
        }
    }

    println!("Count: {count}");
}
