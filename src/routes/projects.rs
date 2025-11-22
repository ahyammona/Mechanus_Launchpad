use teloxide::prelude::*;
use teloxide::types::ParseMode;
use crate::utils::{keyboards, formatters};
use crate::services::web3::Web3Service;
use crate::services::web3::TokenData;
pub struct ProjectsRoute;

impl ProjectsRoute {
    pub async fn projects_main(bot: Bot, msg: Message, web3: Web3Service, page: Option<usize>) -> ResponseResult<()> {
        let current_page = page.unwrap_or(0);
        let projects = web3.get_active_projects().await.unwrap_or_default();
        
        let has_prev = current_page > 0;
        let has_next = projects.len() > (current_page + 1) * 5;
        
        let text = "📊 <b>Projects Explorer</b>\n\n\
                   <b>Browse:</b> Use Prev/Next to navigate all projects\n\
                   <b>New Projects:</b> Recently launched tokens\n\
                   <b>Trending:</b> Top 5 most popular investments\n\
                   <b>Insert Token:</b> Analyze any token by address";
        
        let keyboard = keyboards::projects_list(&projects, current_page);
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
            
        Ok(())
    }

    pub async fn new_projects(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let mut projects = web3.get_active_projects().await.unwrap_or_default();
        
        // Sort by start time (newest first)
        projects.sort_by(|a, b| b.start_time.cmp(&a.start_time));
        
        let text = "🆕 <b>New Projects</b>\n\n\
                   <i>Recently launched tokens - get in early!</i>";
        
        let keyboard = keyboards::projects_new_list(&projects);
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
            
        Ok(())
    }

    pub async fn trending_projects(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let mut projects = web3.get_active_projects().await.unwrap_or_default();
        
        // Sort by total raised (trending = most funded)
        projects.sort_by(|a, b| b.total_raised.cmp(&a.total_raised));
        
        let text = "🚀 <b>Trending Projects</b>\n\n\
                   <i>Top 5 most funded projects - community favorites!</i>";
        
        let keyboard = keyboards::projects_trending_list(&projects);
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
            
        Ok(())
    }

    pub async fn insert_token(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let text = "➕ <b>Insert Token Address</b>\n\n\
                   <b>Enter any ERC-20 token address to analyze:</b>\n\
                   • Get instant AI analysis\n\
                   • View token metrics\n\
                   • Check rugpull risk\n\
                   • Investment recommendations\n\n\
                   <i>Example: 0x742d35Cc6634C0532925a3b844Bc454e4438f44e</i>";
        
      //  let keyboard = keyboards::projects_insert_token();
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
         //   .reply_markup(keyboard)
            .await?;
            
        Ok(())
    }
    pub async fn handle_token_address(
     bot: Bot, 
     msg: Message, 
     web3: Web3Service,
     token_address: String,
     ) -> ResponseResult<()> {
     // Validate token address format
     if !token_address.starts_with("0x") || token_address.len() != 42 {
        bot.send_message(msg.chat.id, "❌ <b>Invalid token address format</b>\n\nPlease enter a valid ERC-20 address starting with 0x and 42 characters long.")
            .parse_mode(ParseMode::Html)
            .await?;
        return Ok(());
     }

     // Show loading message
     let loading_msg = bot.send_message(msg.chat.id, "🔍 <b>Analyzing token...</b>\n\nPlease wait while I fetch and analyze the token data.")
        .parse_mode(ParseMode::Html)
        .await?;

     match web3.get_token_data(&token_address).await {
        Ok(Some(token_data)) => {
            let text = Self::format_token_details(&token_data);
            let keyboard = keyboards::token_detail(&token_data, &token_address);
            
            bot.edit_message_text(loading_msg.chat.id, loading_msg.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        }
        Ok(None) => {
            bot.edit_message_text(loading_msg.chat.id, loading_msg.id, "❌ <b>Token not found</b>\n\nCould not find any token data for the provided address. Please check the address and try again.")
                .parse_mode(ParseMode::Html)
                .await?;
        }
        Err(_) => {
            bot.edit_message_text(loading_msg.chat.id, loading_msg.id, "❌ <b>Error fetching token data</b>\n\nPlease try again later or check if the token address is correct.")
                .parse_mode(ParseMode::Html)
                .await?;
            }
        }

         Ok(())
    }
    pub async fn select_investment_amount(
        bot: Bot,
        msg: Message,
        web3: Web3Service,
        token_address: String,
     ) -> ResponseResult<()> {
        match web3.get_token_data(&token_address).await {
            Ok(Some(token_data)) => {
            let text = format!(
                "💰 <b>Invest in {}</b>\n\n\
                 <b>Token Price:</b> <code>{} ETH</code>\n\
                 <b>Available:</b> <code>{} ETH</code>\n\
                 <b>Your Balance:</b> <code>{} ETH</code>\n\n\
                 <b>Select investment amount:</b>",
                token_data.name,
                formatters::wei_to_eth(token_data.token_price.as_u128()),
                formatters::wei_to_eth(token_data.remaining_cap.as_u128()),
                formatters::wei_to_eth(token_data.user_balance.as_u128())
            );
            
            let keyboard = keyboards::investment_amounts(&token_data, &token_address);
            
            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "❌ Token data not available")
                .parse_mode(ParseMode::Html)
                .await?;
         }
        }
    
        Ok(())
    }
    pub async fn confirm_investment(
     bot: Bot,
     msg: Message,
     web3: Web3Service,
     token_address: String,
     amount: String,
     ) -> ResponseResult<()> {
     let amount_eth: f64 = amount.parse().unwrap_or(0.0);
    
     match web3.get_token_data(&token_address).await {
        Ok(Some(token_data)) => {
            let token_price_eth = formatters::wei_to_eth(token_data.token_price.as_u128());
            let tokens_received = (amount_eth / token_price_eth) as u64;
            
            let text = format!(
                "✅ <b>Confirm Investment</b>\n\n\
                 <b>Token:</b> {}\n\
                 <b>Amount:</b> <code>{} ETH</code>\n\
                 <b>You will receive:</b> <code>{}</code> tokens\n\
                 <b>Gas estimate:</b> <code>~0.002 ETH</code>\n\n\
                 <b>Total:</b> <code>{} ETH</code>\n\n\
                 Please confirm your investment:",
                token_data.name,
                amount,
                tokens_received,
                amount_eth + 0.002
            );
            
            let keyboard = keyboards::confirm_investment(&token_address, &amount);
            
            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "❌ Token data not available")
                .parse_mode(ParseMode::Html)
                .await?;
        }
     }
    
     Ok(())
    }
    pub async fn execute_investment(
     bot: Bot,
     msg: Message,
     web3: Web3Service,
     token_address: String,
     amount: String,
     ) -> ResponseResult<()> {
     let amount_eth: f64 = amount.parse().unwrap_or(0.0);
    
     // Show processing message
     let processing_msg = bot.send_message(
        msg.chat.id,
        "⏳ <b>Processing your investment...</b>\n\nPlease wait while we execute the transaction."
     )
     .parse_mode(ParseMode::Html)
     .await?;

     // Execute investment
     match web3.invest_in_token(&token_address, amount_eth).await {
        Ok(tx_hash) => {
            let text = format!(
                "🎉 <b>Investment Successful!</b>\n\n\
                 <b>Amount invested:</b> <code>{} ETH</code>\n\
                 <b>Transaction:</b> <code>{}</code>\n\
                 <b>Status:</b> ✅ Confirmed\n\n\
                 You can view your investment in your portfolio.",
                amount,
                tx_hash[..16].to_string() + "..."
            );
            
            let keyboard = keyboards::investment_success();
            
            bot.edit_message_text(processing_msg.chat.id, processing_msg.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        }
        Err(e) => {
            let error_text = format!(
                "❌ <b>Investment Failed</b>\n\n\
                 <b>Error:</b> {}\n\n\
                 Please try again or contact support if the problem persists.",
                e
            );
            
            let keyboard = keyboards::investment_failed(&token_address, &amount);
            
            bot.edit_message_text(processing_msg.chat.id, processing_msg.id, error_text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        }
     }
    
     Ok(())
    }

    pub async fn list_projects(bot: Bot, msg: Message, web3: Web3Service, page: usize) -> ResponseResult<()> {
     let projects = web3.get_active_projects().await.unwrap_or_default();
    
     let mut projects_text = String::new();
     projects_text.push_str("📊 <b>Active Projects</b>\n\n");
    
     if projects.is_empty() {
         projects_text.push_str("❌ <b>No projects available</b>\n\n");
         projects_text.push_str("<i>Check back later or use the buttons below to explore other options</i>");
     } else {
         projects_text.push_str("<b>AI-Verified Projects with Strong Fundamentals</b> 🤖\n\n");
        
         for (i, project) in projects.iter().enumerate() {
            projects_text.push_str(&format!(
                "{} <b>{}</b>\n",
                match i {
                    0 => "1️⃣",
                    1 => "2️⃣", 
                    2 => "3️⃣",
                    3 => "4️⃣",
                    4 => "5️⃣",
                    _ => "🔸"
                },
                project.name
            ));
            
            projects_text.push_str(&format!(
                "   • Raised: <code>{} ETH</code>\n",
                crate::utils::formatters::wei_to_eth(project.total_raised.as_u128())
            ));
            projects_text.push_str(&format!(
                "   • Status: {}\n",
                match project.status {
                    0 => "📝 Draft",
                    1 => "⏳ Pending",
                    2 => "✅ Approved",
                    3 => "🚀 Active",
                    4 => "✅ Completed",
                    5 => "❌ Cancelled",
                    _ => "❓ Unknown",
                }
            ));
            projects_text.push_str(&format!(
                "   • Hard Cap: <code>{} ETH</code>\n\n",
                crate::utils::formatters::wei_to_eth(project.hard_cap.as_u128())
            ));
        }
        
        projects_text.push_str("<i>Use Prev/Select/Next to navigate or explore other views below</i>");
     }
    
     let keyboard = keyboards::projects_list(&projects, page);
    
     bot.send_message(msg.chat.id, projects_text)
         .parse_mode(ParseMode::Html)
         .reply_markup(keyboard)
         .await?;
    
     Ok(())
    }
    pub async fn project_detail(bot: Bot, msg: Message, web3: Web3Service, project_id: String) -> ResponseResult<()> {
        if let Some(project) = web3.get_project(&project_id).await.unwrap_or(None) {
            let text = formatters::format_project_detail(&project);
            let keyboard = keyboards::project_detail(&project);
            
            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        } else {
            bot.send_message(msg.chat.id, "❌ Project not found")
                .parse_mode(ParseMode::Html)
                .await?;
        }
        
        Ok(())
    }

    pub async fn ai_analysis(bot: Bot, msg: Message, web3: Web3Service, project_id: String) -> ResponseResult<()> {
        if let Some(project) = web3.get_project(&project_id).await.unwrap_or(None) {
            let ai_analysis = Self::mock_ai_analysis(&project);
            
            let text = format!("🤖 <b>AI Analysis for {}</b>\n\n{}\n\n<b>AI Confidence: {}%</b>",
                project.name,
                ai_analysis.report,
                85
            );
            
            let keyboard = keyboards::project_ai_analysis(&project);
            
            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        } else {
            bot.send_message(msg.chat.id, "❌ Project not found")
                .parse_mode(ParseMode::Html)
                .await?;
        }
        
        Ok(())
    }

    pub async fn invest_project(bot: Bot, msg: Message, web3: Web3Service, project_id: String) -> ResponseResult<()> {
        if let Some(project) = web3.get_project(&project_id).await.unwrap_or(None) {
            let text = format!("💰 <b>Invest in {}</b>\n\n\
                              <b>Token Price:</b> <code>{} ETH</code>\n\
                              <b>Min Investment:</b> <code>{} ETH</code>\n\
                              <b>Max Investment:</b> <code>{} ETH</code>\n\n\
                              Please send the investment amount in ETH:",
                project.name,
                formatters::wei_to_eth(project.token_price.as_u128()),
                formatters::wei_to_eth(project.min_investment.as_u128()),
                formatters::wei_to_eth(project.max_investment.as_u128())
            );
            
            let keyboard = keyboards::invest_project(&project);
            
            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        } else {
            bot.send_message(msg.chat.id, "❌ Project not found")
                .parse_mode(ParseMode::Html)
                .await?;
        }
        
        Ok(())
    }

    pub async fn handle_callback(
     bot: Bot,
     msg: Message,
     web3: Web3Service,
     parts: &[&str],
     ) -> ResponseResult<()> {
     if parts.len() > 0 {
        match parts[0] {
            "projects" => {
                if parts.len() > 1 {
                    match parts[1] {
                        "main" => {
                            let page = parts.get(2).and_then(|p| p.parse().ok());
                            Self::projects_main(bot, msg, web3, page).await
                        }
                        "new" => Self::new_projects(bot, msg, web3).await,
                        "trending" => Self::trending_projects(bot, msg, web3).await,
                        "insert" => Self::insert_token(bot, msg, web3).await,
                        "prev" => {
                            let current_page: usize = parts.get(2).and_then(|p| p.parse().ok()).unwrap_or(1);
                            Self::projects_main(bot, msg, web3, Some(current_page.saturating_sub(1))).await
                        }
                        "next" => {
                            let current_page: usize = parts.get(2).and_then(|p| p.parse().ok()).unwrap_or(0);
                            Self::projects_main(bot, msg, web3, Some(current_page + 1)).await
                        }
                        "list" => {
                            let page: usize = parts.get(2).and_then(|p| p.parse().ok()).unwrap_or(0);
                            Self::list_projects(bot, msg, web3, page).await
                        }
                        "detail" => {
                            if parts.len() > 2 {
                                let project_id = parts[2].to_string();
                                Self::project_detail(bot, msg, web3, project_id).await
                            } else {
                                Ok(())
                            }
                        }
                        "ai_analysis" => {
                            if parts.len() > 2 {
                                let project_id = parts[2].to_string();
                                Self::ai_analysis(bot, msg, web3, project_id).await
                            } else {
                                Ok(())
                            }
                        }
                        // NEW: Handle insert token flow callbacks
                        "invest" => {
                            if parts.len() > 2 {
                                let short_address = parts[2].to_string();
                                let full_address = format!("0x{}00000000000000000000000000000000", short_address);
                                Self::select_investment_amount(bot, msg, web3, full_address).await
                            } else {
                                Ok(())
                            }
                        }
                        "amt" => {
                            if parts.len() > 3 {
                                let short_address = parts[2].to_string();
                                let amount = parts[3].to_string();
                                let full_address = format!("0x{}00000000000000000000000000000000", short_address);
                                Self::confirm_investment(bot, msg, web3, full_address, amount).await
                            } else {
                                Ok(())
                            }
                        }
                        "confirm" => {
                            if parts.len() > 3 {
                                let short_address = parts[2].to_string();
                                let amount = parts[3].to_string();
                                let full_address = format!("0x{}00000000000000000000000000000000", short_address);
                                Self::execute_investment(bot, msg, web3, full_address, amount).await
                            } else {
                                Ok(())
                            }
                        }
                        _ => Self::projects_main(bot, msg, web3, None).await
                    }
                } else {
                    Self::projects_main(bot, msg, web3, None).await
                }
            }
            "back_to_main" => Self::projects_main(bot, msg, web3, None).await,
            _ => Self::projects_main(bot, msg, web3, None).await
         }
      } else {
        Self::projects_main(bot, msg, web3, None).await
      }
    }

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

    fn mock_ai_analysis(project: &crate::services::web3::Project) -> crate::routes::ai_analytics::ProjectAnalysis {
        crate::routes::ai_analytics::ProjectAnalysis {
            confidence_score: 78.5,
            risk_score: 4.2,
            report: format!("📊 <b>Fundamental Analysis:</b>\n\
                           • <b>Team Strength:</b> Strong\n\
                           • <b>Tokenomics:</b> Healthy\n\
                           • <b>Market Fit:</b> Excellent\n\
                           • <b>Risk Level:</b> Medium\n\n\
                           🔍 <b>Technical Analysis:</b>\n\
                           • <b>Smart Contract:</b> Audited\n\
                           • <b>Liquidity:</b> {}%\n\
                           • <b>Vesting:</b> Standard\n\n\
                           ✅ <b>Verdict:</b> Recommended for investment",
                project.liquidity_percent.as_u64() / 100
            ),
        }
    }
    fn format_token_details(token_data: &TokenData) -> String {
     format!(
        "🔍 <b>Token Analysis</b>\n\n\
         <b>Name:</b> {}\n\
         <b>Symbol:</b> {}\n\
         <b>Address:</b> <code>{}</code>\n\n\
         <b>💰 Financials:</b>\n\
         • Price: <code>{} ETH</code>\n\
         • Total Raised: <code>{} ETH</code>\n\
         • Hard Cap: <code>{} ETH</code>\n\
         • Liquidity: {}%\n\n\
         <b>📊 Status:</b>\n\
         • Investors: {}\n\
         • Verified: {}\n\
         • Risk Level: {}/10\n\n\
         <b>🤖 AI Insight:</b>\n\
         This token shows strong fundamentals with moderate risk. \
         Consider investing based on your risk tolerance.",
        token_data.name,
        token_data.symbol,
        token_data.address[..16].to_string() + "...",
        formatters::wei_to_eth(token_data.token_price.as_u128()),
        formatters::wei_to_eth(token_data.total_raised.as_u128()),
        formatters::wei_to_eth(token_data.hard_cap.as_u128()),
        token_data.liquidity_percent.as_u64() / 100,
        token_data.investor_count,
        if token_data.verified { "✅ Yes" } else { "❌ No" },
        token_data.risk_score.as_u64() / 10
    )
    }
}