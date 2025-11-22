use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::services::web3::Project;
use crate::services::web3::TokenData;
pub fn main_menu() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    keyboard.push(vec![
        InlineKeyboardButton::callback("👑 Creator Studio", "creator:studio"),
    ]);

    keyboard.push(vec![
        InlineKeyboardButton::callback("🔍 Discover Projects", "projects:list:0"),
    ]);

    keyboard.push(vec![
        InlineKeyboardButton::callback("💎 My Investments", "investments:my_investments"),
    ]);

    keyboard.push(vec![
        InlineKeyboardButton::callback("💰 Wallet", "wallet:view"),
    ]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn creator_studio() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![
            InlineKeyboardButton::callback("🚀 Create Launchpad", "creator:create"),
        ],
        vec![
            InlineKeyboardButton::callback("📊 My Projects", "creator:my_projects"),
        ],
        vec![
            InlineKeyboardButton::callback("👤 Creator Profile", "creator:view_performance"),
        ],
        vec![
            InlineKeyboardButton::callback("⬅️ Back", "home:main_menu"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn creator_create_launchpad() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("🤖 Quick Launch (AI)", "creator:quick_start")],
        vec![InlineKeyboardButton::callback("🛠️ Manual Launch", "creator:manual_start")],
        vec![InlineKeyboardButton::callback("📚 Guide", "creator:creator_guide")],
        vec![
            InlineKeyboardButton::callback("⬅️ Back", "creator:studio"),
            InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn creator_quick_launch_confirm(suggestions: &crate::routes::creator::AiTokenSuggestions) -> InlineKeyboardMarkup {
    let token_data = format!("{}:{}", suggestions.name, suggestions.symbol);
    let keyboard = vec![
        vec![
            InlineKeyboardButton::callback("✅ Deploy Token", format!("creator:deploy_token:{}", token_data)),
            InlineKeyboardButton::callback("🔄 Regenerate", "creator:quick_start"),
        ],
        vec![
            InlineKeyboardButton::callback("🛠️ Switch to Manual", "creator:manual_start"),
            InlineKeyboardButton::callback("⬅️ Back", "creator:create"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn creator_manual_launch_start() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("📝 Enter Token Name", "creator:enter_name")],
        vec![
            InlineKeyboardButton::callback("⬅️ Back", "creator:create"),
            InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn creator_back_to_manual() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("⬅️ Back to Manual Setup", "creator:manual_start")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn creator_manual_launch_confirm(symbol: &str) -> InlineKeyboardMarkup {
    let token_data = format!("ManualToken:{}", symbol);
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("✅ Deploy Token", format!("creator:deploy_token:{}", token_data))],
        vec![
            InlineKeyboardButton::callback("🔄 Start Over", "creator:manual_start"),
            InlineKeyboardButton::callback("⬅️ Back", "creator:create"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn creator_deployment_success() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("📊 View Project", "creator:my_projects")],
        vec![InlineKeyboardButton::callback("📢 Share Token", "creator:share")],
        vec![
            InlineKeyboardButton::callback("🚀 Create Another", "creator:create"),
            InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn creator_my_projects() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("📋 View Current Launch", "creator:view_current_launch")],
        vec![InlineKeyboardButton::callback("❌ Cancel Current Launch", "creator:cancel_launch")],
        vec![InlineKeyboardButton::callback("📈 View Studio Performace", "creator:view_performance")],
        vec![
            InlineKeyboardButton::callback("⬅️ Back", "creator:studio"),
            InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}
pub fn creator_guide() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("⬅️ Back", "creator:create")],
        vec![InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
    ],
    

    ];

    InlineKeyboardMarkup::new(keyboard)
}
pub fn view_current_launch() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("🔄 Refresh", "creator:view_current_launch")],
        vec![InlineKeyboardButton::callback("⬅️ Back To My Projects", "creator:my_projects")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn view_studio_performance() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("🔄 Refresh", "creator:view_performance")],
        vec![InlineKeyboardButton::callback("⬅️ Back To Studios", "creator:studio")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}
pub fn wallet_menu() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![
            InlineKeyboardButton::callback("📥 Deposit", "wallet:deposit"),
            InlineKeyboardButton::callback("📤 Withdraw", "wallet:withdraw"),
        ],
        vec![
            InlineKeyboardButton::callback("📊 Balance", "wallet:balance"),
            InlineKeyboardButton::callback("📜 Transaction History", "wallet:history"),
        ],
        vec![
            InlineKeyboardButton::callback("🔗 Connect Wallet", "wallet:connect"),
            InlineKeyboardButton::callback("⚙️ Settings", "wallet:settings"),
        ],
        vec![
            InlineKeyboardButton::callback("⬅️ Back", "home:main_menu"),
            InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}
fn format_address_short(address: &web3::types::Address) -> String {
    let full = format!("{:#x}", address);
    if full.len() >= 10 {
        format!("0x…{}", &full[full.len()-4..])
    } else {
        full
    }
}

fn format_address_middle(address: &web3::types::Address) -> String {
    let full = format!("{:#x}", address);
    if full.len() >= 12 {
        format!("{}…{}", &full[0..6], &full[full.len()-4..])
    } else {
        full
    }
}

pub fn projects_list(projects: &[Project], page: usize) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    // MAIN NAVIGATION: Prev, Select, Next - ALWAYS VISIBLE
    keyboard.push(vec![
        InlineKeyboardButton::callback("⬅️ Prev", format!("projects:list:{}", page.saturating_sub(1))),
        InlineKeyboardButton::callback("🎯 Select", format!("projects:select:{}", page)),
        InlineKeyboardButton::callback("Next ➡️", format!("projects:list:{}", page + 1)),
    ]);

    // QUICK ACCESS BUTTONS
    keyboard.push(vec![
        InlineKeyboardButton::callback("🆕 New Projects", "projects:new"),
        InlineKeyboardButton::callback("🚀 Trending Projects", "projects:trending"),
    ]);
    
    keyboard.push(vec![
        InlineKeyboardButton::callback("➕ Insert Token", "projects:insert"),
    ]);

    // BOTTOM NAVIGATION
    keyboard.push(vec![
        InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu")
    ]);

    InlineKeyboardMarkup::new(keyboard)
}
pub fn projects_new_list(projects: &[Project]) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
    
    for project in projects.iter().take(5) {
        keyboard.push(vec![
            InlineKeyboardButton::callback(
                format!("🆕 {}", project.name),
                format!("projects:detail:{}", project.id)
            )
        ]);
    }
    
    // Navigation at bottom
    keyboard.push(vec![
        InlineKeyboardButton::callback("⬅️ Back to Projects", "projects:list:0"),
        InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
    ]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn projects_trending_list(projects: &[Project]) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
    
    // Show trending projects (sorted by total raised + AI score)
    for (i, project) in projects.iter().take(5).enumerate() {
        let rank_emoji = match i {
            0 => "🥇",
            1 => "🥈", 
            2 => "🥉",
            3 => "4️⃣",
            4 => "5️⃣",
            _ => "🔸"
        };
        keyboard.push(vec![
            InlineKeyboardButton::callback(
                format!("{} {}", rank_emoji, project.name),
                format!("projects:detail:{}", project.id)
            )
        ]);
    }
    
    // Navigation at bottom
    keyboard.push(vec![
        InlineKeyboardButton::callback("⬅️ Back to Projects", "projects:list:0"),
        InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
    ]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn projects_insert_token() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("📝 Enter Token Address", "projects:insert:address")],
        vec![
            InlineKeyboardButton::callback("⬅️ Back to Projects", "projects:list:0"),
            InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn project_detail(project: &Project) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    keyboard.push(vec![
        InlineKeyboardButton::callback("🤖 AI Analysis", format!("projects:ai_analysis:{}", project.id)),
        InlineKeyboardButton::callback("💰 Invest", format!("projects:invest:{}", project.id)),
    ]);

    keyboard.push(vec![
        InlineKeyboardButton::callback("📈 View Chart", format!("projects:chart:{}", project.id)),
        InlineKeyboardButton::callback("👥 Community", format!("projects:community:{}", project.id)),
    ]);

    keyboard.push(vec![
        InlineKeyboardButton::callback("⬅️ Back to Projects", "projects:list:0"),
        InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
    ]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn project_ai_analysis(project: &Project) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    keyboard.push(vec![
        InlineKeyboardButton::callback("💰 Invest Now", format!("projects:invest:{}", project.id)),
        InlineKeyboardButton::callback("🔄 Refresh Analysis", format!("projects:ai_analysis:{}", project.id)),
    ]);

    keyboard.push(vec![
        InlineKeyboardButton::callback("⬅️ Back to Project", format!("projects:detail:{}", project.id)),
        InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
    ]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn ai_analytics_menu() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    keyboard.push(vec![
        InlineKeyboardButton::callback("🛡️ Risk Analysis", "ai:risk_analysis"),
        InlineKeyboardButton::callback("📈 Market Insights", "ai:market_insights"),
    ]);

    keyboard.push(vec![
        InlineKeyboardButton::callback("🔄 Refresh Predictions", "ai:predictions"),
        InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
    ]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn projects_empty() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("🔄 Refresh", "projects:list:0")],
        vec![InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn invest_project(project: &Project) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    let quick_amounts = vec![0.1, 0.5, 1.0, 5.0];
    
    for chunk in quick_amounts.chunks(2) {
        let row: Vec<InlineKeyboardButton> = chunk.iter().map(|&amount| {
            InlineKeyboardButton::callback(
                format!("{} ETH", amount),
                format!("invest:amount:{}:{}", project.id, amount)
            )
        }).collect();
        keyboard.push(row);
    }

    keyboard.push(vec![
        InlineKeyboardButton::callback("⬅️ Back", format!("projects:detail:{}", project.id)),
        InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
    ]);

    InlineKeyboardMarkup::new(keyboard)
}
pub fn token_detail(token_data: &TokenData, token_address: &str) -> InlineKeyboardMarkup {
    let short_address = &token_address[..16]; // Use first 16 chars
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("💰 Invest", format!("projects:invest:{}", short_address))], // Changed to "projects:invest:"
        vec![InlineKeyboardButton::callback("⬅️ Back", "projects:main")],
    ])
}
pub fn investment_amounts(token_data: &TokenData, token_address: &str) -> InlineKeyboardMarkup {
    let amounts = vec!["0.01","0.05","0.1", "0.5", "1.0", "2.0", "5.0"];
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
    let mut row: Vec<InlineKeyboardButton> = Vec::new();
    let short_address = &token_address[..16]; // Use first 16 chars

    for amount in amounts {
        row.push(InlineKeyboardButton::callback(
            format!("{} ETH", amount),
            format!("projects:amt:{}:{}", short_address, amount) // Changed to "projects:amt:"
        ));
        
        if row.len() == 3 {
            keyboard.push(row);
            row = Vec::new();
        }
    }
    
    if !row.is_empty() {
        keyboard.push(row);
    }
    
    keyboard.push(vec![
        InlineKeyboardButton::callback("⬅️ Back", format!("projects:invest:{}", short_address)) // Changed to "projects:invest:"
    ]);
    
    InlineKeyboardMarkup::new(keyboard)
}

pub fn confirm_investment(token_address: &str, amount: &str) -> InlineKeyboardMarkup {
    let short_address = &token_address[..16]; // Use first 16 chars
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("✅ Confirm", format!("projects:confirm:{}:{}", short_address, amount)), // Changed to "projects:confirm:"
            InlineKeyboardButton::callback("❌ Cancel", format!("projects:invest:{}", short_address)), // Changed to "projects:invest:"
        ],
        vec![InlineKeyboardButton::callback("⬅️ Back to Main", "projects:main")],
    ])
}
pub fn cancel_launch() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![
            InlineKeyboardButton::callback("✅ Confirm Cancel", "creator:confirm_cancel"),
            InlineKeyboardButton::callback("❌ Keep Launching", "creator:create"),
        ],
        vec![
            InlineKeyboardButton::callback("⬅️ Back to Studio", "creator:studio"),
            InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn cancel_success() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("🚀 Start New Project", "creator:create")],
        vec![
            InlineKeyboardButton::callback("⬅️ Creator Studio", "creator:studio"),
            InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}
pub fn cancel_failed() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("🔄 Try Again", "creator:cancel_launch")],
        vec![
            InlineKeyboardButton::callback("📞 Contact Support", "support:contact"),
            InlineKeyboardButton::callback("⬅️ Creator Studio", "creator:studio"),
        ],
        vec![InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}
pub fn investment_success() -> InlineKeyboardMarkup {
    
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("📊 View Portfolio", "portfolio:main")],
        vec![InlineKeyboardButton::callback("🏠 Main Menu", "projects:main")],
    ])
}

pub fn investment_failed(token_address: &str, amount: &str) -> InlineKeyboardMarkup {
    let short_address = &token_address[..16]; // Use first 16 chars
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("🔄 Try Again", format!("projects:confirm:{}:{}", short_address, amount))], // Changed to "projects:confirm:"
        vec![InlineKeyboardButton::callback("⬅️ Back", format!("projects:invest:{}", short_address))], // Changed to "projects:invest:"
    ])
}
pub fn admin_panel() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![
            InlineKeyboardButton::callback("📋 Pending Projects", "admin:pending"),
            InlineKeyboardButton::callback("📊 Analytics", "admin:analytics"),
        ],
        vec![
            InlineKeyboardButton::callback("👁️ All Projects", "projects:list:0"), // Point to projects_list
            InlineKeyboardButton::callback("🚨 Emergency", "admin:emergency"),
        ],
        vec![InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn admin_pending_list(projects: &[Project]) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    for project in projects.iter().take(5) {
        let short_id = format_address_short(&project.id);
        keyboard.push(vec![
            InlineKeyboardButton::callback(
                format!("🔍 Review {}", project.name),
                format!("admin:review:{}", project.id)
            )
        ]);
    }

    keyboard.push(vec![
        InlineKeyboardButton::callback("🔄 Refresh", "admin:pending"),
        InlineKeyboardButton::callback("⬅️ Back", "admin:panel"),
    ]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn admin_pending_empty() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("🔄 Refresh", "admin:pending")],
        vec![InlineKeyboardButton::callback("⬅️ Back", "admin:panel")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn admin_project_review(project_id: &str) -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![
            InlineKeyboardButton::callback("✅ Approve", format!("admin:approve:{}", project_id)),
            InlineKeyboardButton::callback("❌ Reject", format!("admin:reject:{}", project_id)),
        ],
        vec![
            InlineKeyboardButton::callback("🤖 AI Analysis", format!("projects:ai_analysis:{}", project_id)),
            InlineKeyboardButton::callback("📊 Due Diligence", format!("admin:diligence:{}", project_id)),
        ],
        vec![
            InlineKeyboardButton::callback("⬅️ Back to Pending", "admin:pending"),
            InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn admin_after_action() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("📋 Back to Pending", "admin:pending")],
        vec![InlineKeyboardButton::callback("⚙️ Admin Panel", "admin:panel")],
        vec![InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn admin_analytics() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![
            InlineKeyboardButton::callback("📈 Financial Report", "admin:financial"),
            InlineKeyboardButton::callback("👥 User Stats", "admin:userstats"),
        ],
        vec![
            InlineKeyboardButton::callback("🔄 Refresh", "admin:analytics"),
            InlineKeyboardButton::callback("📤 Export", "admin:export"),
        ],
        vec![
            InlineKeyboardButton::callback("⬅️ Back", "admin:panel"),
            InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn admin_emergency_actions() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![
            InlineKeyboardButton::callback("🛑 Cancel Project", "admin:cancel_project"),
            InlineKeyboardButton::callback("🔄 Pause Platform", "admin:pause_platform"),
        ],
        vec![
            InlineKeyboardButton::callback("💰 Emergency Withdraw", "admin:emergency_withdraw"),
            InlineKeyboardButton::callback("🔒 Lock Contract", "admin:lock_contract"),
        ],
        vec![
            InlineKeyboardButton::callback("⚠️ Confirm Emergency", "admin:confirm_emergency"),
        ],
        vec![
            InlineKeyboardButton::callback("⬅️ Back", "admin:panel"),
            InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}
pub fn wallet_initial() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("🆕 Create Wallet", "wallet:create_prompt")],
        vec![InlineKeyboardButton::callback("📂 Already Have Wallet", "wallet:existing")],
        vec![InlineKeyboardButton::callback("⬅️ Back", "home:main_menu")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

// Confirm wallet creation when user already has one
pub fn wallet_confirm_create() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("✅ Yes, Create New Wallet", "wallet:create")],
        vec![InlineKeyboardButton::callback("❌ No, Keep Current Wallet", "wallet:existing")],
        vec![InlineKeyboardButton::callback("⬅️ Back", "wallet:main")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

// When user has no wallet
pub fn wallet_no_wallet() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("🆕 Create Wallet", "wallet:create_prompt")],
        vec![InlineKeyboardButton::callback("⬅️ Back", "wallet:main")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

// Main wallet screen (when user has wallet)
pub fn wallet_main() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("📥 Receive", "wallet:receive")],
        vec![InlineKeyboardButton::callback("📤 Send", "wallet:send")],
        vec![InlineKeyboardButton::callback("⬅️ Back", "wallet:main")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn wallet_created() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("📥 Receive Funds", "wallet:receive")],
        vec![InlineKeyboardButton::callback("⬅️ Back to Wallet", "wallet:existing")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

// For receive funds page
pub fn wallet_receive() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("⬅️ Back to Wallet", "wallet:existing")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

// For send funds page  
pub fn wallet_send() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("⬅️ Back to Wallet", "wallet:existing")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}
pub fn wallet_create() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("🆕 Create Wallet", "wallet:create")],
        vec![InlineKeyboardButton::callback("⬅️ Back", "home:main_menu")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn investments_list(investments: &[crate::services::web3::Investment]) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    keyboard.push(vec![
        InlineKeyboardButton::callback("🔄 Refresh", "investments:list"),
    ]);

    keyboard.push(vec![
        InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
    ]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn investments_empty() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("📊 Browse Projects", "projects:list:0")], // Point to projects_list
        vec![InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn investment_detail(project_id: &str, investment: &crate::services::web3::Investment) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    if !investment.tokens_claimed && !investment.refunded {
        keyboard.push(vec![
            InlineKeyboardButton::callback("🎯 Claim Tokens", format!("investments:claim:{}", project_id)),
        ]);
    }

    keyboard.push(vec![
        InlineKeyboardButton::callback("⬅️ Back to Investments", "investments:list"),
        InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu"),
    ]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn profile_menu() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![
            InlineKeyboardButton::callback("💼 My Investments", "investments:list"),
            InlineKeyboardButton::callback("🔄 Refresh", "profile:refresh"),
        ],
        vec![InlineKeyboardButton::callback("🏠 Main Menu", "home:main_menu")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

fn wei_to_eth(wei: u128) -> String {
    format!("{:.4}", wei as f64 / 1e18)
}