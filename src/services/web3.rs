use web3::types::{Address, U256};
use std::str::FromStr;
use anyhow::Result;
use std::sync::Arc;
use std::error::Error;

// Make Web3Service clonable and thread-safe
#[derive(Clone)]
pub struct Web3Service {
    contract_address: Address,
    rpc_url: String,
    // Use Arc for thread safety if needed
}

#[derive(Debug, Clone)]
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
    pub status: u8,
    pub lp_token_address: Option<Address>,
    pub lp_tokens_locked: U256,
    pub funds_claimed: bool,
    pub liquidity_added: bool,
}

#[derive(Debug, Clone)]
pub struct Investment {
    pub investor: Address,
    pub project_id: Address,
    pub amount: U256,
    pub token_amount: U256,
    pub timestamp: U256,
    pub refunded: bool,
    pub tokens_claimed: bool,
}

#[derive(Debug, Clone)]
pub struct TokenData {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub token_price: U256,
    pub total_raised: U256,
    pub hard_cap: U256,
    pub remaining_cap: U256,
    pub liquidity_percent: U256,
    pub investor_count: u64,
    pub verified: bool,
    pub risk_score: U256,
    pub user_balance: U256,
}
impl Web3Service {
    pub fn new(contract_address: &str, rpc_url: &str) -> Result<Self> {
        let address = Address::from_str(contract_address)?;
        Ok(Self {
            contract_address: address,
            rpc_url: rpc_url.to_string(),
        })
    }
    
    pub async fn get_active_projects(&self) -> Result<Vec<Project>, Box<dyn Error + Send + Sync>> {
        // Mock data - replace with actual contract calls
        log::info!("Fetching active projects from contract: {}", self.contract_address);
        
        Ok(vec![
            Project {
                id: Address::from_str("0x742d35Cc6634C0532925a3b844Bc454e4438f44e").unwrap(),
                name: "NeuralAI Protocol".to_string(),
                creator: Address::from_str("0x1234567890123456789012345678901234567890").unwrap(),
                offering_token: Address::from_str("0x742d35Cc6634C0532925a3b844Bc454e4438f44e").unwrap(),
                soft_cap: U256::from(100_000_000_000_000_000_000u128),
                hard_cap: U256::from(500_000_000_000_000_000_000u128),
                total_raised: U256::from(350_000_000_000_000_000_000u128),
                token_price: U256::from(25_000_000_000_000u128),
                min_investment: U256::from(100_000_000_000_000_000u128),
                max_investment: U256::from(10_000_000_000_000_000_000u128),
                start_time: U256::from(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() - 86400),
                end_time: U256::from(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() + 86400 * 7),
                tokens_for_sale: U256::from(20_000_000_000_000_000_000_000u128),
                liquidity_percent: U256::from(6000),
                status: 3, // ACTIVE
                lp_token_address: None,
                lp_tokens_locked: U256::zero(),
                funds_claimed: false,
                liquidity_added: false,
            }
        ])
    }

    pub async fn get_project(&self, project_id: &str) -> Result<Option<Project>, Box<dyn Error + Send + Sync>> {
        let projects = self.get_active_projects().await?;
        let target_addr = Address::from_str(project_id)?;
        
        Ok(projects.into_iter()
            .find(|p| p.id == target_addr)
        )
    }
    
    pub async fn get_project_investment(
        &self, 
        project_id: &str, 
        user_address: &str
     ) -> Result<Option<Investment>> {
        let user_addr = Address::from_str(user_address)?;
        let project_addr = Address::from_str(project_id)?;
        
        // Get all user investments and find the one for this specific project
        let investments = self.get_user_investments(user_address).await?;
        
        Ok(investments.into_iter()
            .find(|inv| inv.project_id == project_addr)
        )
    }
    
    pub async fn get_token_data(&self, address: &str) -> Result<Option<TokenData>, Box<dyn Error + Send + Sync>> {
        Ok(Some(TokenData {
            address: address.to_string(),
            name: "Example Token".to_string(),
            symbol: "EXMPL".to_string(),
            token_price: U256::from(1000000000000000u64),
            total_raised: U256::from(5000000000000000000u64),
            hard_cap: U256::from(10000000000000000000u64),
            remaining_cap: U256::from(5000000000000000000u64),
            liquidity_percent: U256::from(60),
            investor_count: 150,
            verified: true,
            risk_score: U256::from(4),
            user_balance: U256::from(1000000000000000000u64),
        }))
    }

    pub async fn invest_in_token(&self, token_address: &str, amount_eth: f64) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Simple mock implementation without rand
        Ok(format!("0x{}", "a1b2c3d4e5f6789012345678901234567890"))
    }
    pub async fn get_user_investments(&self, user_address: &str) -> Result<Vec<Investment>> {
        let user_addr = Address::from_str(user_address)?;
        
        // Mock investments
        Ok(vec![
            Investment {
                investor: user_addr,
                project_id: Address::from_str("0x742d35Cc6634C0532925a3b844Bc454e4438f44e").unwrap(),
                amount: U256::from(1_000_000_000_000_000_000u128),
                token_amount: U256::from(40_000_000_000_000_000_000u128),
                timestamp: U256::from(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() - 86400),
                refunded: false,
                tokens_claimed: true,
            }
        ])
    }

    pub async fn invest_in_project(&self, project_id: &str, amount: U256) -> Result<String> {
        log::info!("Simulating investment of {} in project {}", amount, project_id);
        Ok(format!("0x{:064x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()))
    }

    pub async fn claim_tokens(&self, project_id: &str) -> Result<String> {
        log::info!("Simulating token claim for project {}", project_id);
        Ok(format!("0x{:064x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()))
    }
}