use kobe_client::client_builder::KobeApiClientBuilder;

#[tokio::main]
async fn main() {
    let client = KobeApiClientBuilder::new()
        .base_url("http://localhost:8080")
        .build();

    let blacklist = client.get_bam_delegation_blacklist().await.unwrap();

    println!("Bam Delegation Blacklist: {:?}", blacklist);
}
