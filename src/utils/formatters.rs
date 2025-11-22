pub fn wei_to_eth(wei: u128) -> f64 {
    wei as f64 / 1e18
}
pub fn format_token_amount(amount: u128) -> String {
    if amount >= 1_000_000_000_000_000_000_000 {
        format!("{:.1}M", amount as f64 / 1e18 / 1e6)
    } else if amount >= 1_000_000_000_000_000_000 {
        format!("{:.1}K", amount as f64 / 1e18 / 1e3)
    } else {
        format!("{:.0}", amount as f64 / 1e18)
    }
}

pub fn format_timestamp(timestamp: u64) -> String {
    chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn format_project_detail(project: &crate::services::web3::Project) -> String {
    let progress = (project.total_raised.as_u128() as f64 / project.hard_cap.as_u128() as f64) * 100.0;
    let time_left = project.end_time.as_u64() as i64 - chrono::Utc::now().timestamp();
    let days_left = time_left / 86400;
    let hours_left = (time_left % 86400) / 3600;

    format!("🚀 *{}*\n\n\
            🤖 AI Score: {}/100 \\| 🛡️ Risk: {}\n\n\
            💵 Price: `{} ETH`\n\
            🎯 Target: `{} ETH` \\| 📈 Raised: `{} ETH` \\({:.1}%\\)\n\
            ⏰ Ends: {} days {} hours\n\n\
            Strong fundamentals with AI\\-powered trading algorithms and proven team background\\.",
        project.name,
        92, // Mock AI score
        if progress > 70.0 { "Low" } else { "Medium" },
        wei_to_eth(project.token_price.as_u128()),
        wei_to_eth(project.hard_cap.as_u128()),
        wei_to_eth(project.total_raised.as_u128()),
        progress,
        days_left,
        hours_left
    )
}