use chrono::{DateTime, Utc, serde::ts_seconds};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BamEpochMetrics {
    /// Allocation tier based on JIP-28 in BPS
    pub allocation_bps: u64,

    /// Available bam delegation stake
    pub available_bam_delegation_stake: u64,

    /// Total stake amount of BAM eligible validators in lamports
    pub bam_stake: u64,

    /// Eligible BAM validator count
    pub eligible_bam_validator_count: u64,

    /// Epoch number
    pub epoch: u64,

    /// Total JitoSOL TVL in lamports
    pub jitosol_stake: u64,

    /// Timestamp
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,

    /// Total stake amount of all validators in lamports
    pub total_stake: u64,
}

/// BAM epoch metrics response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BamEpochMetricsResponse {
    /// BAM Epoch metrics
    pub bam_epoch_metrics: Option<BamEpochMetrics>,
}
