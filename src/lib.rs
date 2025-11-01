use reqwest::Client;

mod staker_reward;

/// Kobe API Client
pub struct KobeClient {
    /// Request Client
    client: Client,

    /// Base URL
    base_url: String,
}

impl KobeClient {
    /// Initialize [`KobeClient`]
    pub const fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }

    ///
    pub fn get_staker_rewards(&self) -> Result {}
}
