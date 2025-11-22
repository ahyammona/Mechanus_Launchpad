use serde::{Deserialize, Serialize};
use web3::types::{Address, U256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Investment {
    pub investor: Address,
    pub project_id: Address,
    pub amount: U256,
    pub token_amount: U256,
    pub timestamp: U256,
    pub refunded: bool,
    pub tokens_claimed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentSummary {
    pub total_invested: U256,
    pub total_tokens: U256,
    pub active_investments: usize,
    pub claimed_investments: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentAction {
    pub investment_type: InvestmentType,
    pub amount: U256,
    pub timestamp: i64,
    pub tx_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvestmentType {
    Invest,
    ClaimTokens,
    Refund,
}