use serde::{Deserialize, Serialize};
use web3::types::{Address, U256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Address,
    pub name: String,
    pub creator: Address,
    pub offering_token: Address,
    pub soft_cap: U256,
    pub hard_cap: U256,
    pub total_raised: U256,
    pub token_price: U256,
    pub min_investment: U256,
    pub max_investment: U256,
    pub start_time: U256,
    pub end_time: U256,
    pub tokens_for_sale: U256,
    pub liquidity_percent: U256,
    pub status: ProjectStatus,
    pub lp_token_address: Option<Address>,
    pub lp_tokens_locked: U256,
    pub funds_claimed: bool,
    pub liquidity_added: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    Pending,
    Approved,
    Rejected,
    Active,
    Completed,
    Cancelled,
    LiquidityAdded,
}

impl From<u8> for ProjectStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => ProjectStatus::Pending,
            1 => ProjectStatus::Approved,
            2 => ProjectStatus::Rejected,
            3 => ProjectStatus::Active,
            4 => ProjectStatus::Completed,
            5 => ProjectStatus::Cancelled,
            6 => ProjectStatus::LiquidityAdded,
            _ => ProjectStatus::Pending,
        }
    }
}

impl From<ProjectStatus> for u8 {
    fn from(status: ProjectStatus) -> Self {
        match status {
            ProjectStatus::Pending => 0,
            ProjectStatus::Approved => 1,
            ProjectStatus::Rejected => 2,
            ProjectStatus::Active => 3,
            ProjectStatus::Completed => 4,
            ProjectStatus::Cancelled => 5,
            ProjectStatus::LiquidityAdded => 6,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAnalysis {
    pub project_id: Address,
    pub confidence_score: f64,
    pub risk_score: f64,
    pub recommendation: String,
    pub report: String,
    pub generated_at: i64,
}

// Additional project-related structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDetails {
    pub description: String,
    pub website: String,
    pub whitepaper: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStats {
    pub total_investors: u64,
    pub progress_percent: f64,
    pub time_remaining: i64,
    pub ai_score: f64,
}