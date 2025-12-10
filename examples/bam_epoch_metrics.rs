use kobe_client::client_builder::KobeApiClientBuilder;

#[tokio::main]
async fn main() {
    let client = KobeApiClientBuilder::new()
        .base_url("http://localhost:8080")
        .build();

    let bam_epoch_metrics = client.get_bam_epoch_metrics(892).await.unwrap();

    println!("Bam Epoch Metrics: {:?}", bam_epoch_metrics);
}
