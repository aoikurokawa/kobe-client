use kobe_client::client_builder::KobeApiClientBuilder;

#[tokio::main]
async fn main() {
    let client = KobeApiClientBuilder::new()
        .base_url("http://localhost:8080")
        .build();

    let bam_validators = client.get_bam_validators(892).await.unwrap();

    println!("Bam Validators: {:?}", bam_validators);
}
