use chrono::{DateTime, Utc, serde::ts_seconds};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BamValidator {
    /// Active stake in lamports
    pub active_stake: u64,

    /// Epoch number
    pub epoch: u64,

    /// Identity account public key
    pub identity_account: String,

    /// Is eligible validator
    pub is_eligible: bool,

    // The reason of ineligibility
    pub ineligibility_reason: Option<String>,

    /// Timestamp
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,

    /// Vote account public key
    pub vote_account: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BamValidatorsResponse {
    pub bam_validators: Vec<BamValidator>,
}
