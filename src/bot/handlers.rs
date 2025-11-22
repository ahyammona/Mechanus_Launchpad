use teloxide::prelude::*;
use teloxide::types::ParseMode;
use crate::routes::{
    home::HomeRoute,
    projects::ProjectsRoute,
    investments::InvestmentsRoute,
    admin::AdminRoute,  
    wallet::WalletRoute, // Add wallet route
    ai_analytics::AIAnalyticsRoute,
    creator::CreatorRoute, 
};
use crate::services::web3::Web3Service;

#[derive(Clone)]
pub enum Command {
    Start,
    Help,
    Projects,
    Investments,
    Admin,
    Analytics,
}
impl Command {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "/start" => Some(Command::Start),
            "/help" => Some(Command::Help),
            "/projects" => Some(Command::Projects),
            "/investments" => Some(Command::Investments),
            "/admin" => Some(Command::Admin),
            "/analytics" => Some(Command::Analytics),
            _ => None,
        }
    }
}
async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    web3: Web3Service,
) -> ResponseResult<()> {
    match cmd {
        Command::Start => HomeRoute::start(bot, msg, web3).await,
        Command::Help => HomeRoute::help(bot, msg, web3).await,
        Command::Projects => ProjectsRoute::list_projects(bot, msg, web3, 0).await,
        Command::Investments => InvestmentsRoute::my_investments(bot, msg, web3).await,
        Command::Admin => AdminRoute::admin_panel(bot, msg, web3).await,
        Command::Analytics => AIAnalyticsRoute::ai_predictions(bot, msg, web3).await,
    }
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    web3: Web3Service,
) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        // Check if it's a command first
        if let Some(cmd) = Command::from_str(text) {
            return handle_command(bot, msg, cmd, web3).await;
        }
        
        match text {
            "👑 Creator Studio" => crate::routes::creator::CreatorRoute::creator_studio(bot, msg, web3).await,
            "🔍 Discover Projects" => ProjectsRoute::list_projects(bot, msg, web3, 0).await,
            "💎 My Investments" => InvestmentsRoute::my_investments(bot, msg, web3).await,
            "💰 Wallet" => WalletRoute::wallet_main(bot, msg, web3).await, // Updated wallet handler

            _ => HomeRoute::main_menu(bot, msg, web3).await,
        }
    } else {
        HomeRoute::main_menu(bot, msg, web3).await
    }
}

pub async fn callback_handler(bot: Bot, q: CallbackQuery, web3: Web3Service) -> ResponseResult<()> {
    if let Some(data) = &q.data {
        let parts: Vec<&str> = data.split(':').collect();
        
        if let Some(message) = &q.message {
            // Extract the message from the callback query
            let msg = message.clone();
            
            match parts[0] {
                "home" => crate::routes::home::HomeRoute::handle_callback(bot, msg, web3, &parts).await,
                "projects" => crate::routes::projects::ProjectsRoute::handle_callback(bot, msg, web3, &parts).await,
                "investments" => crate::routes::investments::InvestmentsRoute::handle_callback(bot, msg, web3, &parts).await,
                "ai" => crate::routes::ai_analytics::AIAnalyticsRoute::handle_callback(bot, msg, web3, &parts).await,
                "admin" => crate::routes::admin::AdminRoute::handle_callback(bot, msg, web3, &parts).await,
                "creator" => crate::routes::creator::CreatorRoute::handle_callback(bot, msg, web3, &parts).await,
                "wallet" => crate::routes::wallet::WalletRoute::handle_callback(bot, msg, web3, &parts).await, 
               
                _ => crate::routes::home::HomeRoute::main_menu(bot, msg, web3).await,
            }
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

// Helper function to create message for handlers
fn create_message_from(message: &Message) -> Message {
    Message {
        id: message.id,
        chat: message.chat.clone(),
        date: message.date,
        kind: message.kind.clone(),
        thread_id: None,
        via_bot: None,
    }
}