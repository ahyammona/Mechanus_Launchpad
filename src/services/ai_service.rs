use crate::services::web3::Project;

pub struct AIService;

impl AIService {
    pub async fn analyze_project(project: &Project) -> AIAnalysis {
        // Simulate AI analysis - integrate with actual AI model
        let confidence_score = calculate_confidence(project);
        let risk_score = calculate_risk(project);
        let recommendation = generate_recommendation(confidence_score, risk_score);
        
        AIAnalysis {
            project_id: project.id,
            confidence_score,
            risk_score,
            recommendation,
            report: generate_ai_report(project, confidence_score, risk_score),
            generated_at: chrono::Utc::now().timestamp(),
        }
    }
    
    pub async fn get_portfolio_predictions(user_id: u64) -> PortfolioPredictions {
        // Simulate portfolio predictions
        PortfolioPredictions {
            sentiment: "Bullish".to_string(),
            confidence: 85.0,
            recommendations: "• Increase exposure to AI projects\n• Diversify with DeFi tokens\n• Consider taking profits on MEME coins".to_string(),
            insights: "Market showing strong momentum in AI sector. Expected 15-25% growth in next 30 days.".to_string(),
        }
    }
    
    pub async fn get_risk_analysis(user_id: u64) -> RiskAnalysis {
        RiskAnalysis {
            risk_level: 3,
            health_score: 87.5,
            rugpull_risk: false,
            risk_factors: "• High concentration in single project\n• Good diversification otherwise".to_string(),
        }
    }
    
    pub async fn get_market_insights() -> MarketInsights {
        MarketInsights {
            trend: "Bullish",
            hot_sectors: "• AI & Machine Learning\n• Gaming & Metaverse\n• DeFi 2.0".to_string(),
            opportunities: "• Early-stage AI projects\n• Infrastructure tokens\n• Cross-chain bridges".to_string(),
            accuracy: 92.0,
        }
    }
}

// Helper functions - fixed to handle U256 properly
fn calculate_confidence(project: &Project) -> f64 {
    let mut score: f32 = 70.0; // Base score
    
    // Convert U256 to u128 for calculations
    let total_raised = project.total_raised.as_u128();
    let soft_cap = project.soft_cap.as_u128();
    let hard_cap = project.hard_cap.as_u128();
    let liquidity_percent = project.liquidity_percent.as_u128();
    
    // Adjust based on various factors - use integer division
    if soft_cap > 0 && total_raised > soft_cap / 2 {
        score += 15.0;
    }
    
    if liquidity_percent >= 6000 {
        score += 10.0;
    }
    
    // Add progress-based scoring
    if hard_cap > 0 {
        let progress = (total_raised as f64 / hard_cap as f64) * 100.0;
        if progress > 50.0 {
            score += 10.0;
        }
        
        // Bonus for high progress
        if progress > 80.0 {
            score += 5.0;
        }
    }
    
    // Time-based scoring
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let time_passed = current_time.saturating_sub(project.start_time.as_u64());
    let total_duration = project.end_time.as_u64().saturating_sub(project.start_time.as_u64());
    
    if total_duration > 0 {
        let time_ratio = time_passed as f64 / total_duration as f64;
        // Bonus for projects that are gaining traction quickly
        if time_ratio < 0.5 && soft_cap > 0 && (total_raised as f64 / soft_cap as f64) > 0.7 {
            score += 8.0;
        }
    }
    
    score.min(100.0).into()
}

fn calculate_risk(project: &Project) -> f64 {
    let mut risk: f32 = 30.0; // Base risk
    
    // Convert U256 to u128 for calculations
    let total_raised = project.total_raised.as_u128();
    let soft_cap = project.soft_cap.as_u128();
    let hard_cap = project.hard_cap.as_u128();
    
    // Adjust based on various factors - use integer division
    if soft_cap > 0 && total_raised < soft_cap / 5 {
        risk += 20.0;
    }
    
    // Add time-based risk (if project is almost ending)
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let time_left = project.end_time.as_u64().saturating_sub(current_time);
    let days_left = time_left / 86400;
    
    if days_left < 1 {
        risk += 15.0; // High risk if less than 1 day left
    } else if days_left < 3 {
        risk += 10.0; // Medium risk if less than 3 days left
    }
    
    // Risk based on investment concentration
    if hard_cap > 0 && project.min_investment.as_u128() > hard_cap / 20 {
        risk += 5.0; // High minimum investment
    }
    
    risk.min(100.0).into()
}

fn generate_recommendation(confidence: f64, risk: f64) -> String {
    match (confidence, risk) {
        (c, _) if c >= 80.0 => "STRONG BUY - High confidence in project success".to_string(),
        (c, r) if c >= 60.0 && r < 50.0 => "BUY - Good potential with manageable risk".to_string(),
        (c, r) if c >= 40.0 && r < 70.0 => "HOLD - Moderate potential, monitor closely".to_string(),
        _ => "AVOID - High risk or low confidence".to_string(),
    }
}

fn generate_ai_report(project: &Project, confidence: f64, risk: f64) -> String {
    let total_raised = project.total_raised.as_u128();
    let hard_cap = project.hard_cap.as_u128();
    let progress = if hard_cap > 0 {
        (total_raised as f64 / hard_cap as f64) * 100.0
    } else {
        0.0
    };
    
    format!(
        "🤖 *AI Project Analysis Report*\n\n\
        *Fundamentals Score:* {}/100\n\
        *Risk Assessment:* {}/100\n\
        *Team Verification:* ✅ Verified\n\
        *Code Audit:* 🟡 In Progress\n\
        *Market Fit:* 🟢 Strong\n\n\
        *Key Metrics:*\n\
        • Funding Progress: {:.1}%\n\
        • Community Engagement: High\n\
        • Tokenomics: Sustainable\n\n\
        *AI Verdict:* {}",
        confidence as i32,
        risk as i32,
        progress,
        if confidence >= 70.0 { "🟢 PROMISING" } else { "🟡 CAUTION" }
    )
}

#[derive(Debug)]
pub struct AIAnalysis {
    pub project_id: web3::types::Address,
    pub confidence_score: f64,
    pub risk_score: f64,
    pub recommendation: String,
    pub report: String,
    pub generated_at: i64,
}

#[derive(Debug)]
pub struct PortfolioPredictions {
    pub sentiment: String,
    pub confidence: f64,
    pub recommendations: String,
    pub insights: String,
}

#[derive(Debug)]
pub struct RiskAnalysis {
    pub risk_level: i32,
    pub health_score: f64,
    pub rugpull_risk: bool,
    pub risk_factors: String,
}

#[derive(Debug)]
pub struct MarketInsights {
    pub trend: &'static str,
    pub hot_sectors: String,
    pub opportunities: String,
    pub accuracy: f64,
}