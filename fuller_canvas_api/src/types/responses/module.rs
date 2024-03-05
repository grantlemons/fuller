use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Module {
    pub id: u64,
    // pub workflow_state: ModuleWorkflowState,
    pub position: u64, // 1 based
    pub name: String,
    pub unlock_at: Option<DateTime<Utc>>,
    pub items_count: u64,
    pub items: Option<Vec<ModuleItem>>,
    pub state: Option<ModuleState>, // None only if user is teacher
}

impl crate::types::ResponseType for Module {}

impl std::cmp::PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.id, self.name)
    }
}

#[derive(Debug, Deserialize)]
pub struct ModuleItem {
    pub id: u64,
    pub module_id: u64,
    pub position: u64, // 1 based
    pub title: String,
    pub indent: u64, // unsure if should be represented
    #[serde(alias = "type")]
    pub content_type: ModuleItemType,
    pub content_id: Option<u64>,
    pub html_url: Option<String>,
}

impl crate::types::ResponseType for ModuleItem {}

impl std::cmp::PartialEq for ModuleItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Display for ModuleItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.id, self.title)
    }
}

#[derive(Debug, Deserialize)]
// #[serde(rename_all = "snake_case")]
pub enum ModuleItemType {
    File,
    Page,
    Discussion,
    Assignment,
    Quiz,
    SubHeader,
    ExternalUrl,
    ExternalTool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleWorkflowState {
    Active,
    Deleted,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleState {
    Locked,
    Unlocked,
    Started,
    Completed,
}
