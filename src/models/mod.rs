pub mod project;
pub mod investment;
pub mod user;

// Re-export for easy access
pub use project::{Project, ProjectStatus, AIAnalysis};
pub use investment::Investment;
pub use user::{User, UserTier};