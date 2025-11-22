use teloxide::prelude::*;
use teloxide::types::ParseMode;
use crate::services::web3::Web3Service;
use crate::utils::{keyboards, formatters};

pub struct InvestmentsRoute;

impl InvestmentsRoute {
    pub async fn my_investments(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let user_address = format!("0x{:040x}", msg.chat.id.0);
        
        match web3.get_user_investments(&user_address).await {
            Ok(investments) if !investments.is_empty() => {
                let total_invested: u128 = investments.iter()
                    .map(|inv| inv.amount.as_u128())
                    .sum();
                let total_tokens: u128 = investments.iter()
                    .map(|inv| inv.token_amount.as_u128())
                    .sum();
                let text = format!("💼 <b>My Investments</b>                                                  \n\n\
                                  <b>Total Invested:</b> <code>{}BSC</code>                                       \n\
                                  <b>Total Tokens:</b> <code>{}</code>                                          \n\
                                  <b>Active Investments:</b> <code>{}</code>            \n\n\
                                  <b>Recent Investments: $Quantum, $Sporty, $Nighty</b>",
                    formatters::wei_to_eth(total_invested),
                    formatters::format_token_amount(total_tokens),
                    investments.len()
                );
                
                let keyboard = keyboards::investments_list(&investments);
                
                bot.send_message(msg.chat.id, text)
                    .parse_mode(ParseMode::Html)
                    .reply_markup(keyboard)
                    .await?;
            }
            Ok(_) => {
                let text = "💼 <b>My Investments</b>\n\n\
                           You haven't made any investments yet.\n\n\
                           Browse active projects to get started!";
                
                let keyboard = keyboards::investments_empty();
                
                bot.send_message(msg.chat.id, text)
                    .parse_mode(ParseMode::Html)
                    .reply_markup(keyboard)
                    .await?;
            }
            Err(e) => {
                log::error!("Failed to get investments: {}", e);
                bot.send_message(msg.chat.id, "❌ Failed to load investments")
                    .parse_mode(ParseMode::Html)
                    .await?;
            }
        }
        
        Ok(())
    }

    pub async fn investment_detail(bot: Bot, msg: Message, web3: Web3Service, project_id: String) -> ResponseResult<()> {
        let user_address = format!("0x{:040x}", msg.chat.id.0);
        
        match web3.get_project_investment(&project_id, &user_address).await {
            Ok(Some(investment)) => {
                let text = format!("📈 <b>Investment Details</b>\n\n\
                                  <b>Investment Amount:</b> <code>{} ETH</code>\n\
                                  <b>Tokens Allocated:</b> <code>{}</code>\n\
                                  <b>Investment Date:</b> <code>{}</code>\n\
                                  <b>Status:</b> {}\n\
                                  <b>Tokens Claimed:</b> {}",
                    formatters::wei_to_eth(investment.amount.as_u128()),
                    formatters::format_token_amount(investment.token_amount.as_u128()),
                    formatters::format_timestamp(investment.timestamp.as_u64()),
                    if investment.refunded { "❌ Refunded" } else { "✅ Active" },
                    if investment.tokens_claimed { "✅ Yes" } else { "❌ No" }
                );
                
                let keyboard = keyboards::investment_detail(&project_id, &investment);
                
                bot.send_message(msg.chat.id, text)
                    .parse_mode(ParseMode::Html)
                    .reply_markup(keyboard)
                    .await?;
            }
            Ok(None) => {
                bot.send_message(msg.chat.id, "❌ Investment not found")
                    .parse_mode(ParseMode::Html)
                    .await?;
            }
            Err(e) => {
                log::error!("Failed to get investment detail: {}", e);
                bot.send_message(msg.chat.id, "❌ Failed to load investment details")
                    .parse_mode(ParseMode::Html)
                    .await?;
            }
        }
        
        Ok(())
    }

    pub async fn claim_investment_tokens(bot: Bot, msg: Message, web3: Web3Service, project_id: String) -> ResponseResult<()> {
        match web3.claim_tokens(&project_id).await {
            Ok(tx_hash) => {
                let text = format!("✅ <b>Tokens Claimed Successfully!</b>\n\n\
                                  <b>Transaction Hash:</b>\n<code>{}</code>\n\n\
                                  Your tokens have been transferred to your wallet.",
                    tx_hash
                );
                
                bot.send_message(msg.chat.id, text)
                    .parse_mode(ParseMode::Html)
                    .await?;
            }
            Err(e) => {
                log::error!("Failed to claim tokens: {}", e);
                bot.send_message(msg.chat.id, "❌ Failed to claim tokens")
                    .parse_mode(ParseMode::Html)
                    .await?;
            }
        }
        
        Ok(())
    }

    pub async fn handle_callback(
      bot: Bot,
      msg: Message,
      web3: Web3Service,
      parts: &[&str],
     ) -> ResponseResult<()> {
      if parts.len() > 1 {
         match parts[1] {
             "list" => Self::my_investments(bot, msg, web3).await,
             "my_investments" => Self::my_investments(bot,msg, web3).await,
             "detail" => {
                 let project_id = parts[2].to_string();
                 Self::investment_detail(bot, msg, web3, project_id).await
             }
             "claim" => {
                 let project_id = parts[2].to_string();
                 Self::claim_investment_tokens(bot, msg, web3, project_id).await
            }
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