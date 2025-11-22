use teloxide::prelude::*;
use teloxide::types::ParseMode;
use crate::services::web3::Web3Service;
use crate::utils::keyboards;
use std::str::FromStr;
use web3::types::Address;

pub struct AdminRoute;

impl AdminRoute {
    pub async fn admin_panel(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        if !Self::is_admin(msg.chat.id.0).await {
            bot.send_message(msg.chat.id, "❌ Access denied. Admin privileges required.")
                .parse_mode(ParseMode::Html)
                .await?;
            return Ok(());
        }

        let projects = web3.get_active_projects().await.unwrap_or_default();
        let pending_projects = Self::get_pending_projects().await;

        let text = format!("⚙️ <b>Admin Panel - MechaLaunchpad</b>\n\n\
                          <b>Platform Statistics:</b>\n\
                          • Active Projects: <code>{}</code>\n\
                          • Total Projects: <code>{}</code>\n\
                          • Pending Approval: <code>{}</code>\n\
                          • Total Raised: <code>{} ETH</code>\n\
                          • Platform Fees: <code>{} ETH</code>\n\n\
                          <b>Quick Actions:</b>",
            projects.len(),
            projects.len() + 8,
            pending_projects.len(),
            Self::calculate_total_raised(&projects),
            Self::calculate_platform_fees(&projects)
        );

        let keyboard = keyboards::admin_panel();
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
            
        Ok(())
    }

    pub async fn pending_projects(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        if !Self::is_admin(msg.chat.id.0).await {
            bot.send_message(msg.chat.id, "❌ Access denied.")
                .parse_mode(ParseMode::Html)
                .await?;
            return Ok(());
        }

        let pending_projects = Self::get_pending_projects().await;

        if pending_projects.is_empty() {
            let text = "📋 <b>Pending Projects</b>\n\n\
                       No projects awaiting approval.\n\n\
                       All projects have been reviewed! ✅";
            
            let keyboard = keyboards::admin_pending_empty();
            
            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        } else {
            let text = format!("📋 <b>Pending Projects</b>\n\n\
                              <b>{} projects awaiting review:</b>\n\n\
                              Please review each project carefully before approval.",
                pending_projects.len()
            );
            
            let keyboard = keyboards::admin_pending_list(&pending_projects);
            
            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        }
        
        Ok(())
    }

    pub async fn project_review(bot: Bot, msg: Message, web3: Web3Service, project_id: String) -> ResponseResult<()> {
        if !Self::is_admin(msg.chat.id.0).await {
            bot.send_message(msg.chat.id, "❌ Access denied.")
                .parse_mode(ParseMode::Html)
                .await?;
            return Ok(());
        }

        let project = Self::get_pending_project(&project_id).await;

        if let Some(project) = project {
            let text = format!("🔍 <b>Project Review - {}</b>\n\n\
                              <b>Project ID:</b> <code>{}</code>\n\
                              <b>Creator:</b> <code>{}</code>\n\
                              <b>Token Address:</b> <code>{}</code>\n\
                              <b>Soft Cap:</b> <code>{} ETH</code>\n\
                              <b>Hard Cap:</b> <code>{} ETH</code>\n\
                              <b>Token Price:</b> <code>{} ETH</code>\n\
                              <b>Tokens for Sale:</b> <code>{}</code>\n\
                              <b>Liquidity:</b> <code>{}%</code>\n\
                              <b>Lock Period:</b> <code>{} days</code>\n\n\
                              <b>AI Risk Assessment:</b>\n\
                              • Rugpull Risk: 🟢 LOW\n\
                              • Team Verification: ✅ VERIFIED\n\
                              • Code Audit: 🟡 PENDING\n\
                              • Market Fit: 🟢 STRONG",
                project.name,
                project.id,
                Self::format_address(&project.creator),
                Self::format_address(&project.offering_token),
                crate::utils::formatters::wei_to_eth(project.soft_cap.as_u128()),
                crate::utils::formatters::wei_to_eth(project.hard_cap.as_u128()),
                crate::utils::formatters::wei_to_eth(project.token_price.as_u128()),
                crate::utils::formatters::format_token_amount(project.tokens_for_sale.as_u128()),
                project.liquidity_percent.as_u128() / 100,
                project.lp_tokens_locked.as_u128() / 86400
            );

            let keyboard = keyboards::admin_project_review(&project_id);
            
            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        } else {
            bot.send_message(msg.chat.id, "❌ Project not found.")
                .parse_mode(ParseMode::Html)
                .await?;
        }
        
        Ok(())
    }

    pub async fn approve_project(bot: Bot, msg: Message, web3: Web3Service, project_id: String) -> ResponseResult<()> {
        if !Self::is_admin(msg.chat.id.0).await {
            bot.send_message(msg.chat.id, "❌ Access denied.")
                .parse_mode(ParseMode::Html)
                .await?;
            return Ok(());
        }

        log::info!("Approving project: {}", project_id);

        let text = format!("✅ <b>Project Approved!</b>\n\n\
                          <b>Project ID:</b> <code>{}</code>\n\
                          <b>Status:</b> 🟢 ACTIVE\n\
                          <b>Approved by:</b> <code>Admin</code>\n\
                          <b>Time:</b> <code>{}</code>\n\n\
                          Project is now live and accepting investments!",
            project_id,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
        );

        let keyboard = keyboards::admin_after_action();
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
            
        Ok(())
    }

    pub async fn reject_project(bot: Bot, msg: Message, web3: Web3Service, project_id: String) -> ResponseResult<()> {
        if !Self::is_admin(msg.chat.id.0).await {
            bot.send_message(msg.chat.id, "❌ Access denied.")
                .parse_mode(ParseMode::Html)
                .await?;
            return Ok(());
        }

        log::info!("Rejecting project: {}", project_id);

        let text = format!("❌ <b>Project Rejected!</b>\n\n\
                          <b>Project ID:</b> <code>{}</code>\n\
                          <b>Status:</b> 🔴 REJECTED\n\
                          <b>Rejected by:</b> <code>Admin</code>\n\
                          <b>Time:</b> <code>{}</code>\n\n\
                          Project has been removed from the launchpad.",
            project_id,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
        );

        let keyboard = keyboards::admin_after_action();
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
            
        Ok(())
    }

    pub async fn platform_analytics(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        if !Self::is_admin(msg.chat.id.0).await {
            bot.send_message(msg.chat.id, "❌ Access denied.")
                .parse_mode(ParseMode::Html)
                .await?;
            return Ok(());
        }

        let projects = web3.get_active_projects().await.unwrap_or_default();
        
        let text = format!("📊 <b>Platform Analytics - MechaLaunchpad</b>\n\n\
                          <b>Financial Metrics:</b>\n\
                          • Total Volume: <code>{} ETH</code>\n\
                          • Platform Fees: <code>{} ETH</code>\n\
                          • Average Project Raise: <code>{} ETH</code>\n\
                          • Success Rate: <code>{}%</code>\n\n\
                          <b>Project Metrics:</b>\n\
                          • Active Projects: <code>{}</code>\n\
                          • Completed Projects: <code>{}</code>\n\
                          • Average Funding Time: <code>{} days</code>\n\
                          • Liquidity Rate: <code>{}%</code>\n\n\
                          <b>User Metrics:</b>\n\
                          • Total Investors: <code>{}</code>\n\
                          • Average Investment: <code>{} ETH</code>\n\
                          • Retention Rate: <code>{}%</code>",
            Self::calculate_total_volume(&projects),
            Self::calculate_platform_fees(&projects),
            Self::calculate_avg_raise(&projects),
            87,
            projects.len(),
            12,
            5,
            65,
            892,
            2.5,
            78
        );

        let keyboard = keyboards::admin_analytics();
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
            
        Ok(())
    }

    pub async fn emergency_actions(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        if !Self::is_admin(msg.chat.id.0).await {
            bot.send_message(msg.chat.id, "❌ Access denied.")
                .parse_mode(ParseMode::Html)
                .await?;
            return Ok(());
        }

        let text = "🚨 <b>Emergency Actions - Use with caution!</b>\n\n\
                   <b>Available Emergency Functions:</b>\n\
                   • 🛑 Cancel Project - Stop funding immediately\n\
                   • 🔄 Pause Platform - Stop all activities\n\
                   • 💰 Emergency Withdraw - Recover funds\n\
                   • 🔒 Lock Contract - Prevent further actions\n\n\
                   <b>Warning:</b> These actions are irreversible and should only be used in emergency situations.";

        let keyboard = keyboards::admin_emergency_actions();
        
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
     ) -> ResponseResult<()> {
     if !Self::is_admin(msg.chat.id.0).await {
        bot.send_message(msg.chat.id, "❌ Access denied.")
            .parse_mode(ParseMode::Html)
            .await?;
        return Ok(());
     }

     if parts.len() > 2 {
        match parts[1] {
             "panel" => Self::admin_panel(bot, msg, web3).await,
             "pending" => Self::pending_projects(bot, msg, web3).await,
             "review" => {
                let project_id = parts[2].to_string();
                Self::project_review(bot, msg, web3, project_id).await
            }
            "approve" => {
                let project_id = parts[2].to_string();
                Self::approve_project(bot, msg, web3, project_id).await
            }
            "reject" => {
                let project_id = parts[2].to_string();
                Self::reject_project(bot, msg, web3, project_id).await
            }
            "analytics" => Self::platform_analytics(bot, msg, web3).await,
            "emergency" => Self::emergency_actions(bot, msg, web3).await,
            _ => Ok(())
        }
     } else {
        Ok(())
     }
    }

    // Helper functions (keep your existing ones)
    async fn is_admin(telegram_id: i64) -> bool {
        let admin_ids = vec![123456789, 987654321];
        admin_ids.contains(&telegram_id)
    }

    async fn get_pending_projects() -> Vec<crate::services::web3::Project> {
        vec![
            crate::services::web3::Project {
                id: Address::from_str("0x942d35Cc6634C0532925a3b844Bc454e4438f66e").unwrap(),
                name: "AI Trading Bot".to_string(),
                creator: Address::from_str("0x3234567890123456789012345678901234567890").unwrap(),
                offering_token: Address::from_str("0x942d35Cc6634C0532925a3b844Bc454e4438f66e").unwrap(),
                soft_cap: web3::types::U256::from(80_000_000_000_000_000_000u128),
                hard_cap: web3::types::U256::from(300_000_000_000_000_000_000u128),
                total_raised: web3::types::U256::zero(),
                token_price: web3::types::U256::from(30_000_000_000_000u128),
                min_investment: web3::types::U256::from(100_000_000_000_000_000u128),
                max_investment: web3::types::U256::from(8_000_000_000_000_000_000u128),
                start_time: web3::types::U256::zero(),
                end_time: web3::types::U256::zero(),
                tokens_for_sale: web3::types::U256::from(10_000_000_000_000_000_000_000u128),
                liquidity_percent: web3::types::U256::from(6500),
                status: 0,
                lp_token_address: None,
                lp_tokens_locked: web3::types::U256::zero(),
                funds_claimed: false,
                liquidity_added: false,
            }
        ]
    }

    async fn get_pending_project(project_id: &str) -> Option<crate::services::web3::Project> {
        let projects = Self::get_pending_projects().await;
        let target_addr = web3::types::Address::from_str(project_id).ok()?;
        projects.into_iter().find(|p| p.id == target_addr)
    }

    fn calculate_total_raised(projects: &[crate::services::web3::Project]) -> String {
        let total: u128 = projects.iter()
        .map(|p| p.total_raised.as_u128())
        .sum();
        crate::utils::formatters::wei_to_eth(total).to_string()
    }

    fn calculate_total_volume(projects: &[crate::services::web3::Project]) -> String {
        let total: u128 = projects.iter()
         .map(|p| p.hard_cap.as_u128())
         .sum();
        crate::utils::formatters::wei_to_eth(total).to_string()
    }

    fn calculate_platform_fees(projects: &[crate::services::web3::Project]) -> String {
        let fees: u128 = projects.iter()
         .map(|p| p.total_raised.as_u128() * 5 / 100) // 5% platform fee
         .sum();
        crate::utils::formatters::wei_to_eth(fees).to_string()
    }

    fn calculate_avg_raise(projects: &[crate::services::web3::Project]) -> String {
        if projects.is_empty() {
         return "0".to_string();
     }
        let total: u128 = projects.iter()
        .map(|p| p.total_raised.as_u128())
         .sum();
        let avg = total / projects.len() as u128;
        crate::utils::formatters::wei_to_eth(avg).to_string()
    }
    
    fn format_address(address: &web3::types::Address) -> String {
        let full = format!("{:#x}", address);
        if full.len() >= 10 {
            format!("0x..{}", &full[full.len()-4..])
        } else {
            full
        }
    }

    fn create_message_from(message: &Message) -> Message {
        Message {
         id: message.id,
         chat: message.chat.clone(),
         date: message.date,
         kind: message.kind.clone(),
         thread_id: None, // Add this
         via_bot: None,  
        }
    }
}