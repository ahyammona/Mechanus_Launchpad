use teloxide::prelude::*;
use dotenvy::dotenv;

mod bot;
mod routes;
mod models;
mod services;
mod utils;

use services::web3::Web3Service;

async fn handle_start(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
    crate::routes::home::HomeRoute::start(bot, msg, web3).await
}

async fn handle_message(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
    if let Some(text) = msg.clone().text() {
        match text {
            "/start" => handle_start(bot, msg, web3).await,
            "👑 Creator Studio" => crate::routes::creator::CreatorRoute::creator_studio(bot, msg, web3).await,
            "🔍 Discover Projects" => crate::routes::projects::ProjectsRoute::list_projects(bot, msg, web3, 0).await,
            "💎 My Investments" => crate::routes::investments::InvestmentsRoute::my_investments(bot, msg, web3).await,
            "💰 Wallet" => crate::routes::wallet::WalletRoute::wallet_main(bot, msg, web3).await, // Updated wallet handler

            _ => {
                if text.starts_with("0x") && text.len() == 42 {
                    crate::routes::projects::ProjectsRoute::handle_token_address(bot, msg, web3, text.to_string()).await
                } else {
                    crate::routes::home::HomeRoute::main_menu(bot, msg, web3).await
                }
            }
        }
    } else {
        crate::routes::home::HomeRoute::main_menu(bot, msg, web3).await
    }
}

async fn handle_callback(bot: Bot, q: CallbackQuery, web3: Web3Service) -> ResponseResult<()> {
    if let Some(message) = &q.message {
        let _ = bot.delete_message(message.chat.id, message.id).await;
    }
    
    crate::bot::handlers::callback_handler(bot, q, web3).await
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();
    log::info!("Starting MechaLaunchpad Bot...");

    let bot_token = "8509809077:AAFIorfWSPYGG1RXAtbkJqEEI_li0zPsg1Q".to_string();
    
    let web3 = Web3Service::new(
        "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
        "https://mainnet.infura.io/v3/your-key"
    ).expect("Failed to initialize Web3 service");

    let bot = Bot::new(bot_token);

    log::info!("MechaLaunchpad Bot is running...");

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handle_message))
        .branch(Update::filter_callback_query().endpoint(handle_callback));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![web3])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}