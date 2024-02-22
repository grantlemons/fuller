use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Course {
    pub id: u64,
    pub uuid: String,
    pub name: String,
    pub course_code: String,
    pub workflow_state: WorkflowState,
    pub start_at: DateTime<Local>,
    pub end_at: DateTime<Local>,
}

impl crate::types::ResponseType for Course {}

#[derive(Debug, Deserialize)]
pub enum WorkflowState {
    Unpublished,
    Available,
    Completed,
    Deleted,
}
