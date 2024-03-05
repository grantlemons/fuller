use chrono::{DateTime, Local, Utc};
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

impl std::fmt::Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.id, self.name)
    }
}

impl super::Viewable for Course {
    fn view(&self, config: &fuller_config::Config) -> String {
        let start_at_string = match self.start_at {
            Some(date) => format!(
                "\nStarted On: {}",
                DateTime::<Local>::from(date).format(&config.formatting.date)
            ),
            None => String::default(),
        };
        let end_at_string = match self.end_at {
            Some(date) => format!(
                "\nEnds On:    {}",
                DateTime::<Local>::from(date).format(&config.formatting.date)
            ),
            None => String::default(),
        };
        format!(
            "[{}] {}{}{}", // TODO: Investigate formatting w/ termcolor
            self.id, self.name, start_at_string, end_at_string
        )
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
