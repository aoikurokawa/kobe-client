use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinbaseBalance {
    /// Epoch number
    epoch: u64,

    /// Balance in lamports
    cb_balance_lamports: u64,

    /// Total epoch fee from Dune converted to JitoSOL token units
    total_epoch_fee_jitosol: Option<u64>,

    /// CB proportional share of total_epoch_fee in JitoSOL token units
    cb_epoch_fee_jitosol: Option<u64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CoinbaseBalanceResponse {
    pub coinbase_balance: Option<CoinbaseBalance>,
}
