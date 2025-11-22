use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BotState {
    MainMenu,
    ViewProjects,
    ViewProjectDetail(String),
    MyInvestments,
    MyProfile,
    AdminPanel,
    AIPredictions,
    CreateProject,
    ProjectAnalytics(String),
}

#[derive(Clone, Debug)]
pub struct UserSession {
    pub user_id: i64,
    pub state: BotState,
    pub current_project: Option<String>,
    pub page: usize,
}