pub mod home;
pub mod projects;
pub mod investments;
pub mod ai_analytics;
pub mod admin;
pub mod creator;
pub mod wallet;


pub use creator::CreatorRoute;
pub use home::HomeRoute;
pub use projects::ProjectsRoute;
pub use investments::InvestmentsRoute;
pub use ai_analytics::AIAnalyticsRoute;
pub use admin::AdminRoute;
pub use wallet::WalletRoute;