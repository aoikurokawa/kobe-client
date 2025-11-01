use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Staker rewards response from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakerRewardsResponse {
    pub rewards: Vec<StakerReward>,
    pub total: Option<u64>,
}

/// Individual staker reward entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakerReward {
    /// The public key of the stake account
    pub stake_account: String,

    /// The stake authority
    pub stake_authority: String,

    /// The withdraw authority
    pub withdraw_authority: String,

    /// Epoch when the reward was earned
    pub epoch: u64,

    /// MEV rewards in lamports
    pub mev_rewards: u64,

    /// Priority fee rewards in lamports
    pub priority_fee_rewards: Option<u64>,

    /// Whether MEV rewards have been claimed
    pub mev_claimed: bool,

    /// Whether priority fee rewards have been claimed
    pub priority_fee_claimed: Option<bool>,

    /// Validator vote account
    pub vote_account: String,
}

/// Validator rewards response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorRewardsResponse {
    pub validators: Vec<ValidatorReward>,
}

/// Validator reward entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorReward {
    /// Validator vote account public key
    pub vote_account: String,

    /// Epoch
    pub epoch: u64,

    /// MEV commission in basis points (10000 = 100%)
    pub mev_commission_bps: u16,

    /// Total MEV rewards in lamports
    pub mev_rewards: u64,

    /// Priority fee commission in basis points
    pub priority_fee_commission_bps: Option<u16>,

    /// Total priority fee rewards in lamports
    pub priority_fee_rewards: Option<u64>,

    /// Number of stakers
    pub num_stakers: Option<u64>,

    /// Total active stake
    pub active_stake: Option<u64>,
}

// ============================================================================
// Stake Pool API Types
// ============================================================================

/// Response for validators endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorsResponse {
    pub validators: Vec<ValidatorInfo>,
}

/// Validator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    /// Active stake
    pub active_stake: Option<u64>,

    /// Commission number
    pub commission: Option<u8>,

    /// Consensus mods score
    pub consensus_mods_score: Option<i8>,

    /// Data center concentration score
    pub data_center_concentration_score: Option<i64>,

    /// Delinquent
    pub delinquent: Option<bool>,

    /// Epoch number
    pub epoch: Option<u64>,

    /// Epoch credits
    pub epoch_credits: Option<u64>,

    /// Identity account
    pub identity_account: Option<String>,

    /// MEV Commission BPS
    pub mev_commission_bps: Option<u16>,

    /// MEV revenue lamports
    pub mev_revenue_lamports: Option<u64>,

    /// Priority fee commission bps
    pub priority_fee_commission_bps: Option<u16>,

    /// Priority fee revenue lamports
    pub priority_fee_revenue_lamports: Option<u64>,

    /// Name
    pub name: Option<String>,

    /// Published information score
    pub published_information_score: Option<i64>,

    /// Root distance score
    pub root_distance_score: Option<i64>,

    /// Whether or not running Jito client
    pub running_jito: bool,

    /// Whether or not running BAM client
    pub running_bam: Option<bool>,

    /// Software version
    pub software_version: Option<String>,

    /// Software version score
    pub software_version_score: Option<i64>,

    /// Skipped slot percent
    pub skipped_slot_percent: Option<String>,

    /// Skipped slot score
    pub skipped_slot_score: Option<i64>,

    /// Skipped slots
    pub skipped_slots: Option<u64>,

    /// Stake concentration score
    pub stake_concentration_score: Option<i64>,

    /// Stake percent
    pub stake_percent: Option<f64>,

    /// Target pool is the pool this cranker is managing
    pub target_pool_active_lamports: Option<u64>,

    /// Target pool transient lamports
    pub target_pool_transient_lamports: Option<u64>,

    /// Target pool staked
    pub target_pool_staked: Option<bool>,

    // Timestamp
    // #[serde(with = "ts_seconds_option")]
    // pub timestamp: Option<DateTime<Utc>>,
    /// Vote account
    pub vote_account: String,

    /// Vote credit proportion
    pub vote_credit_proportion: Option<f64>,

    /// WWW URL
    pub www_url: Option<String>,

    /// Inflation lamports
    pub inflation_rewards_lamports: Option<u64>,
}

/// Historical validator data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorHistory {
    /// Epoch
    pub epoch: u64,

    /// MEV commission in basis points
    pub mev_commission_bps: u16,

    /// MEV rewards (lamports)
    pub mev_rewards: u64,

    /// Priority fee commission in basis points
    pub priority_fee_commission_bps: Option<u16>,

    /// Priority fee rewards (lamports)
    pub priority_fee_rewards: Option<u64>,
}

/// MEV rewards network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MevRewards {
    /// Epoch number
    pub epoch: u64,

    /// Total network MEV in lamports
    pub total_network_mev_lamports: u64,

    /// Jito stake weight in lamports
    pub jito_stake_weight_lamports: u64,

    /// MEV reward per lamport staked
    pub mev_reward_per_lamport: f64,
}

/// Daily MEV tips data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyMevTips {
    /// Date of the tips
    pub day: DateTime<Utc>,

    /// Number of MEV tips
    pub count_mev_tips: u64,

    /// Jito tips amount (SOL)
    pub jito_tips: f64,

    /// Number of unique tippers
    pub tippers: u64,

    /// Validator tips amount (SOL)
    pub validator_tips: f64,
}

/// Jito stake over time data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitoStakeOverTime {
    /// Map of epoch to stake ratio
    pub stake_ratio_over_time: std::collections::HashMap<String, f64>,
}

/// MEV commission average over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MevCommissionAverageOverTime {
    /// Aggregated MEV rewards
    pub aggregated_mev_rewards: u64,

    /// MEV rewards time series
    pub mev_rewards: Vec<TimeSeriesData<u64>>,

    /// Total value locked time series
    pub tvl: Vec<TimeSeriesData<u64>>,

    /// APY time series
    pub apy: Vec<TimeSeriesData<f64>>,

    /// Number of validators time series
    pub num_validators: Vec<TimeSeriesData<u64>>,

    /// JitoSOL supply time series
    pub supply: Vec<TimeSeriesData<f64>>,
}

/// Time series data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesData<T> {
    /// Data value
    pub data: T,

    /// Timestamp
    pub date: DateTime<Utc>,
}

/// JitoSOL to SOL ratio data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitoSolRatio {
    /// Time series of ratio data
    pub ratios: Vec<TimeSeriesData<f64>>,
}

/// Stake pool statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakePoolStats {
    /// Epoch
    pub epoch: u64,

    /// Total stake pool value (lamports)
    pub total_lamports: u64,

    /// JitoSOL supply
    pub jitosol_supply: f64,

    /// Current exchange ratio
    pub exchange_ratio: f64,

    /// Annual percentage yield
    pub apy: f64,

    /// Number of validators
    pub num_validators: u64,

    /// Total MEV earned
    pub total_mev_earned: u64,
}

// ============================================================================
// Request Types
// ============================================================================

/// Request parameters for epoch-based queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochRequest {
    /// Epoch number
    pub epoch: u64,
}

/// Range filter for time-based queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeFilter {
    /// Start time (ISO 8601 format)
    pub start: DateTime<Utc>,

    /// End time (ISO 8601 format)
    pub end: DateTime<Utc>,
}

/// Request with range filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeRequest {
    /// Time range filter
    pub range_filter: RangeFilter,
}

// ============================================================================
// StakeNet API Types (on-chain data)
// ============================================================================

/// Validator history account data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorHistoryAccount {
    /// Validator vote account
    pub vote_account: String,

    /// Historical entries
    pub history: Vec<ValidatorHistoryEntry>,
}

/// Single validator history entry for an epoch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorHistoryEntry {
    /// Epoch
    pub epoch: u64,

    /// Vote credits earned
    pub vote_credits: Option<u32>,

    /// Validator commission
    pub commission: Option<u8>,

    /// MEV commission in basis points
    pub mev_commission_bps: Option<u16>,

    /// Validator version
    pub version: Option<String>,

    /// Client type
    pub client_type: Option<String>,

    /// Active stake
    pub active_stake: Option<u64>,

    /// Stake rank
    pub stake_rank: Option<u32>,

    /// Whether validator is in superminority
    pub is_superminority: Option<bool>,

    /// IP address
    pub ip_address: Option<String>,
}

/// Steward configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StewardConfig {
    /// Stake pool address
    pub stake_pool: String,

    /// Authority
    pub authority: String,

    /// Scoring parameters
    pub scoring_params: ScoringParams,
}

/// Scoring parameters for validator selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringParams {
    /// Minimum vote credits
    pub min_vote_credits: u32,

    /// Maximum commission
    pub max_commission: u8,

    /// Performance weight
    pub performance_weight: f64,

    /// Commission weight
    pub commission_weight: f64,

    /// Stake concentration limit
    pub stake_concentration_limit: f64,
}

// ============================================================================
// Common Types
// ============================================================================

/// Query parameters for paginated requests
#[derive(Debug, Clone, Default)]
pub struct QueryParams {
    /// Limit number of results
    pub limit: Option<u32>,

    /// Offset for pagination
    pub offset: Option<u32>,

    /// Epoch filter
    pub epoch: Option<u64>,

    /// Sort order (asc/desc)
    pub sort_order: Option<String>,
}

impl QueryParams {
    /// Create new query params with limit
    pub fn with_limit(limit: u32) -> Self {
        Self {
            limit: Some(limit),
            ..Default::default()
        }
    }

    /// Create new query params with epoch
    pub fn with_epoch(epoch: u64) -> Self {
        Self {
            epoch: Some(epoch),
            ..Default::default()
        }
    }

    /// Set limit
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set offset
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set epoch
    pub fn epoch(mut self, epoch: u64) -> Self {
        self.epoch = Some(epoch);
        self
    }

    /// Convert to query string
    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(limit) = self.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = self.offset {
            params.push(format!("offset={}", offset));
        }
        if let Some(epoch) = self.epoch {
            params.push(format!("epoch={}", epoch));
        }
        if let Some(ref sort_order) = self.sort_order {
            params.push(format!("sort_order={}", sort_order));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

// ============================================================================
// Error Response Types
// ============================================================================

/// API error response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub error: String,
    pub message: Option<String>,
    pub status_code: Option<u16>,
}
