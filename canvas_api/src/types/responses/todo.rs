use super::Assignment;
use serde::Deserialize;

// GET /api/v1/users/self/todo
#[derive(Debug, Deserialize)]
pub struct Todo {
    pub course_id: u64,
    pub context_name: String,
    // pub todo_type: TodoType,
    pub assignment: Option<Assignment>,
    // pub quiz: Option<Quiz>,
    #[serde(alias = "ignore")]
    pub ignore_url: String,
    #[serde(alias = "ignore_permanently")]
    pub ignore_permanently_url: String,
    pub html_url: String,
    pub context_type: TodoContextType,
    pub group_id: Option<u64>,
}

impl crate::types::ResponseType for Todo {}
impl std::cmp::PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(a1), Some(a2)) = (&self.assignment, &other.assignment) {
            a1 == a2
        } else {
            false
        }
    }
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(a) = &self.assignment {
            write!(f, "{}", a.name)
        } else {
            write!(f, "[{}] {}", self.context_type, self.context_name)
        }
    }
}

impl super::Viewable for Todo {
    fn view(&self, config: &canvas_cli_config::Config) -> String {
        if let Some(assignment) = &self.assignment {
            assignment.view(config)
        } else {
            "No Assignment!".to_owned()
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TodoType {
    Submitting,
    Grading,
}

#[derive(Debug, Deserialize)]
pub enum TodoContextType {
    Course,
    Group,
}

impl std::fmt::Display for TodoContextType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Course => "Course",
            Self::Group => "Group",
        };

        write!(f, "{}", str)
    }
}
