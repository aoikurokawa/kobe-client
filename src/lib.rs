use reqwest::Client;

pub mod client;
pub mod client_builder;
pub mod config;
pub mod error;
pub mod staker_reward;
pub mod types;

/// Default base URL for mainnet Kobe API
pub const MAINNET_BASE_URL: &str = "https://kobe.mainnet.jito.network";

/// API version
pub const API_VERSION: &str = "v1";

/// Kobe API Client
pub struct KobeClient {
    /// Request Client
    pub client: Client,

    /// Base URL
    pub base_url: String,
}

impl KobeClient {
    /// Initialize [`KobeClient`]
    pub const fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }
}
