use teloxide::prelude::*;
use teloxide::types::ParseMode;
use crate::utils::keyboards;
use crate::services::web3::Web3Service;

// Move structs outside the impl block
struct PortfolioPredictions {
    sentiment: String,
    confidence: i32,
    recommendations: String,
    insights: String,
}

struct RiskAnalysis {
    risk_level: i32,
    health_score: i32,
    rugpull_risk: bool,
    risk_factors: String,
}

struct MarketInsights {
    trend: &'static str,
    hot_sectors: &'static str,
    opportunities: &'static str,
    accuracy: i32,
}

pub struct ProjectAnalysis {
    pub confidence_score: f64,
    pub risk_score: f64,
    pub report: String,
}

pub struct AIAnalyticsRoute;

impl AIAnalyticsRoute {
    pub async fn ai_predictions(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let user_id = msg.chat.id.0 as u64;
        let user_address = format!("0x{:040x}", user_id);
        
        let investments = web3.get_user_investments(&user_address).await.unwrap_or_default();
        let predictions = Self::mock_portfolio_predictions();
        
        let text = format!("🤖 <b>AI Portfolio Predictions</b>\n\n\
                          <b>Overall Sentiment:</b> {}\n\
                          <b>Confidence Score:</b> {}%\n\
                          <b>Your Investments:</b> <code>{} projects</code>\n\
                          <b>Total Value:</b> <code>{} ETH</code>\n\n\
                          <b>Recommended Actions:</b>\n{}\n\n\
                          <b>Market Insights:</b>\n{}",
            predictions.sentiment,
            predictions.confidence,
            investments.len(),
            crate::utils::formatters::wei_to_eth(
                investments.iter().map(|inv| inv.amount.as_u128()).sum()
            ),
            predictions.recommendations,
            predictions.insights
        );
        
        let keyboard = keyboards::ai_analytics_menu();
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
            
        Ok(())
    }

    pub async fn risk_analysis(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let user_id = msg.chat.id.0 as u64;
        let user_address = format!("0x{:040x}", user_id);
        
        let investments = web3.get_user_investments(&user_address).await.unwrap_or_default();
        let risk_analysis = Self::mock_risk_analysis();
        
        let text = format!("🛡️ <b>AI Risk Analysis</b>\n\n\
                          <b>Risk Level:</b> {}/10\n\
                          <b>Portfolio Health:</b> {}%\n\
                          <b>Rugpull Detection:</b> {}\n\
                          <b>Active Investments:</b> <code>{}</code>\n\n\
                          <b>Risk Factors:</b>\n{}",
            risk_analysis.risk_level,
            risk_analysis.health_score,
            if risk_analysis.rugpull_risk { "⚠️ HIGH RISK" } else { "✅ LOW RISK" },
            investments.len(),
            risk_analysis.risk_factors
        );
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .await?;
            
        Ok(())
    }

    pub async fn market_insights(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let projects = web3.get_active_projects().await.unwrap_or_default();
        let insights = Self::mock_market_insights();
        
        let text = format!("📈 <b>AI Market Insights</b>\n\n\
                          <b>Market Trend:</b> {}\n\
                          <b>Active Launches:</b> <code>{} projects</code>\n\
                          <b>Total Raising:</b> <code>{} ETH</code>\n\n\
                          <b>Hot Sectors:</b>\n{}\n\n\
                          <b>Investment Opportunities:</b>\n{}\n\n\
                          <b>AI Prediction Accuracy:</b> {}%",
            insights.trend,
            projects.len(),
            crate::utils::formatters::wei_to_eth(
                projects.iter().map(|p| p.total_raised.as_u128()).sum()
            ),
            insights.hot_sectors,
            insights.opportunities,
            insights.accuracy
        );
        
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .await?;
            
        Ok(())
    }

    pub async fn project_analysis(bot: Bot, msg: Message, web3: Web3Service, project_id: String) -> ResponseResult<()> {
        if let Some(project) = web3.get_project(&project_id).await.unwrap_or(None) {
            let ai_analysis = Self::mock_project_analysis(&project);
            
            let text = format!("🔍 <b>AI Project Analysis - {}</b>\n\n\
                              <b>Confidence Score:</b> {}%\n\
                              <b>Risk Assessment:</b> {}/10\n\
                              <b>AI Verdict:</b> {}\n\n\
                              <b>Detailed Report:</b>\n{}",
                project.name,
                ai_analysis.confidence_score as i32,
                ai_analysis.risk_score as i32,
                if ai_analysis.confidence_score >= 70.0 { "🟢 STRONG BUY" } 
                else if ai_analysis.confidence_score >= 50.0 { "🟡 HOLD" }
                else { "🔴 AVOID" },
                ai_analysis.report
            );
            
            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
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
     if parts.len() > 1 {
        match parts[1] {
            "predictions" => Self::ai_predictions(bot, msg, web3).await,
            "risk_analysis" => Self::risk_analysis(bot, msg, web3).await,
            "market_insights" => Self::market_insights(bot, msg, web3).await,
            "project_analysis" => {
                if parts.len() > 2 {
                    let project_id = parts[2].to_string();
                    Self::project_analysis(bot, msg, web3, project_id).await
                } else {
                    Ok(())
                }
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
            thread_id: None, // Add missing field
            via_bot: None,   // Add missing field
        }
    }

    // Mock functions
    fn mock_portfolio_predictions() -> PortfolioPredictions {
        PortfolioPredictions {
            sentiment: "🟢 BULLISH".to_string(),
            confidence: 87,
            recommendations: "• Consider increasing position in AI projects\n• Diversify with DeFi tokens\n• Monitor market volatility".to_string(),
            insights: "• AI sector showing strong growth\n• DeFi summer 2.0 expected\n• NFT market cooling down".to_string(),
        }
    }

    fn mock_risk_analysis() -> RiskAnalysis {
        RiskAnalysis {
            risk_level: 3,
            health_score: 85,
            rugpull_risk: false,
            risk_factors: "• High concentration in single project\n• Good diversification otherwise\n• Strong project fundamentals".to_string(),
        }
    }

    fn mock_market_insights() -> MarketInsights {
        MarketInsights {
            trend: "🟢 BULLISH",
            hot_sectors: "• AI & Machine Learning\n• DeFi 2.0 Protocols\n• Gaming & Metaverse",
            opportunities: "• Early-stage AI projects\n• Infrastructure tokens\n• Cross-chain bridges",
            accuracy: 92,
        }
    }

    fn mock_project_analysis(project: &crate::services::web3::Project) -> ProjectAnalysis {
        ProjectAnalysis {
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
}