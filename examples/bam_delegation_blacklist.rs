use kobe_client::client::KobeClient;

#[tokio::main]
async fn main() {
    let client = KobeClient::testnet();

    let blacklist = client.get_bam_delegation_blacklist().await.unwrap();

    println!("Bam Delegation Blacklist: {:?}", blacklist);
}
