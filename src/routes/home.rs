use teloxide::prelude::*;
use teloxide::types::ParseMode;
use crate::utils::keyboards;
use crate::services::web3::Web3Service;

pub struct HomeRoute;

impl HomeRoute {
    pub async fn start(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let text = "🤖 <b>Welcome to MechaLaunchpad!</b>\n\n\
                   <b>The AI-Powered Launchpad for Next-Gen Projects</b> 🚀\n\n\
                   🔍 <b>AI-Powered Analytics</b>\n\
                   🤖 <b>Smart Predictions</b>\n\
                   🛡️ <b>Rugpull Detection</b>\n\
                   🎮 <b>Gamified Experience</b>\n\n\
                   Choose an option below to get started:";
        
        let keyboard = keyboards::main_menu();
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
            
        Ok(())
    }

    pub async fn help(bot: Bot, msg: Message, _web3: Web3Service) -> ResponseResult<()> {
        let text = "📖 <b>MechaLaunchpad Help</b>\n\n\
                   <b>Available Commands:</b>\n\
                   <code>/start</code> - Start the bot\n\
                   <code>/projects</code> - Browse active projects\n\
                   <code>/investments</code> - View your investments\n\
                   <code>/profile</code> - Check your profile\n\
                   <code>/analytics</code> - AI predictions & analytics\n\n\
                   <b>Main Features:</b>\n\
                   • 🤖 AI-Powered project analysis\n\
                   • 🛡️ Rugpull detection system\n\
                   • 📊 Real-time project metrics\n\
                   • 💰 Automated liquidity provision\n\
                   • 🎯 Smart investment suggestions";
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .await?;
            
        Ok(())
    }

    pub async fn main_menu(bot: Bot, msg: Message, _web3: Web3Service) -> ResponseResult<()> {
        let text = "🤖 <b>Welcome to MechaLaunchpad!</b>\n\n\
                   <b>The AI-Powered Launchpad for Next-Gen Projects</b> 🚀\n\n\
                   🔍 <b>AI-Powered Analytics</b>\n\
                   🤖 <b>Smart Predictions</b>\n\
                   🛡️ <b>Rugpull Detection</b>\n\
                   🎮 <b>Gamified Experience</b>\n\n\
                   Choose an option below to get started:";
        
        let keyboard = keyboards::main_menu();
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
            
        Ok(())
    }

    pub async fn handle_callback(
        bot: Bot,
        msg: Message,  
        web3: Web3Service,
        parts: &[&str],
     )  -> ResponseResult<()> {
     if parts.len() > 1 {
        match parts[1] {
            "main_menu" => Self::main_menu(bot, msg, web3).await,
            "help" => Self::help(bot, msg, web3).await,
            _ => Ok(())
            }
        } else {
        Ok(())
        }
    }

   fn create_message_from(message: &Message) -> Message {
        Message {
            id: message.id,
            chat: message.chat.clone(),
            date: message.date,
            kind: message.kind.clone(),
            thread_id: None, // Add this
            via_bot: None,   // Add this
        }
    }
}