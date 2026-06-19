use std::time::Duration;

use kobe_client::client_builder::KobeApiClientBuilder;

#[tokio::main]
async fn main() {
    let client = KobeApiClientBuilder::new()
        .timeout(Duration::from_secs(45))
        .retry(true)
        .max_retries(5)
        .build();

    let current_epoch = 989;
    println!("Current epoch: {}\n", current_epoch);

    let validators = client.get_validators(Some(current_epoch)).await.unwrap();

    println!("Found {} validators", validators.validators.len());

    if let Some(first) = validators.validators.first() {
        println!("First validator: {:?}", first);
    }

    let mut count = 0;
    for validator in validators.validators {
        if validator.bam_connection_rate.is_some() {
            count += 1;
        }
    }

    println!("Count: {count}");
}
