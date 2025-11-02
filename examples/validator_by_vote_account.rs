use std::time::Duration;

use kobe_client::client_builder::KobeApiClientBuilder;

#[tokio::main]
async fn main() {
    let client = KobeApiClientBuilder::new()
        .timeout(Duration::from_secs(45))
        .retry(true)
        .max_retries(5)
        .build();

    let vote_account = "J1to1yufRnoWn81KYg1XkTWzmKjnYSnmE2VY8DGUJ9Qv";

    let validator_info = client
        .get_validator_info_by_vote_account(vote_account)
        .await
        .unwrap();

    println!("Validator Info Length: {}", validator_info.len());
}
