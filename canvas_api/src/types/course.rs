use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Course {
    pub id: u64,
    pub uuid: String,
    pub name: String,
    pub course_code: String,
    pub workflow_state: WorkflowState,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
}

impl crate::types::ResponseType for Course {}

impl std::cmp::PartialEq for Course {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowState {
    Unpublished,
    Available,
    Completed,
    Deleted,
}
