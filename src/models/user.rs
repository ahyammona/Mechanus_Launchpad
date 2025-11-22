use serde::{Deserialize, Serialize};
use web3::types::Address;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub telegram_id: i64,
    pub wallet_address: Option<Address>,
    pub tier: UserTier,
    pub total_invested: u128, // in wei
    pub total_projects_invested: u32,
    pub join_date: i64,
    pub last_active: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserTier {
    None,
    Bronze,
    Silver,
    Gold,
    Platinum,
}

impl UserTier {
    pub fn from_investment_amount(amount: u128) -> Self {
        match amount {
            n if n >= 25_000_000_000_000_000_000 => UserTier::Platinum, // 25+ ETH
            n if n >= 10_000_000_000_000_000_000 => UserTier::Gold,     // 10+ ETH
            n if n >= 5_000_000_000_000_000_000 => UserTier::Silver,    // 5+ ETH
            n if n >= 1_000_000_000_000_000_000 => UserTier::Bronze,    // 1+ ETH
            _ => UserTier::None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            UserTier::Platinum => "🏆 PLATINUM",
            UserTier::Gold => "🥇 GOLD",
            UserTier::Silver => "🥈 SILVER",
            UserTier::Bronze => "🥉 BRONZE",
            UserTier::None => "👤 MEMBER",
        }
    }

    pub fn benefits(&self) -> Vec<&'static str> {
        match self {
            UserTier::Platinum => vec![
                "🚀 Guaranteed allocation in all projects",
                "🎯 Early access to private sales",
                "🤖 Priority AI analysis",
                "📊 Advanced analytics dashboard",
                "👥 VIP community access",
            ],
            UserTier::Gold => vec![
                "✅ Priority allocation in projects",
                "🔍 Early project access",
                "📈 Enhanced AI insights",
                "💬 Premium support",
            ],
            UserTier::Silver => vec![
                "⬆️ Increased allocation limits",
                "📊 Basic AI analytics",
                "🔔 Project notifications",
            ],
            UserTier::Bronze => vec![
                "📈 Investment tracking",
                "🔍 Project discovery",
                "📱 Basic features",
            ],
            UserTier::None => vec![
                "🔍 Browse projects",
                "💰 Basic investing",
                "📊 Public analytics",
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub telegram_id: i64,
    pub current_state: UserState,
    pub current_project: Option<Address>,
    pub page: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserState {
    MainMenu,
    ViewingProjects,
    ViewingProjectDetail(Address),
    ViewingInvestments,
    ViewingProfile,
    ViewingAnalytics,
    Investing(Address),
    ClaimingTokens(Address),
}