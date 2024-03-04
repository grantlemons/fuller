use super::Discussion;
use chrono::{DateTime, Local, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Assignment {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_at: Option<DateTime<Utc>>,
    pub lock_at: Option<DateTime<Utc>>,
    pub unlock_at: Option<DateTime<Utc>>,
    // pub all_dates: Option<()>,
    pub course_id: u64,
    pub html_url: String,
    pub submissions_download_url: Option<String>,
    pub assignment_group_id: u64,
    pub allowed_extensions: Option<Vec<String>>,
    pub max_name_length: u64,
    // pub turnitin_enabled: bool,
    // pub vericite_enabled: bool,
    // pub turnitin_settings: Option<()>,
    pub grade_group_students_individually: bool,
    // pub external_tool_tag_attributes: Option<()>,
    pub peer_reviews: bool,
    pub automatic_peer_reviews: bool,
    pub peer_review_count: Option<u64>,
    pub peer_reviews_assign_at: Option<DateTime<Utc>>,
    pub intra_group_peer_reviews: bool,
    pub group_category_id: Option<u64>,
    pub position: u64,
    // pub post_to_sis: bool,
    pub points_possible: Option<f32>,
    pub submission_types: Vec<AllowedSubmissionType>,
    pub has_submitted_submissions: bool,
    pub grading_type: GradingType,
    // pub only_visible_to_overrides: bool,
    pub locked_for_user: bool,
    // pub lock_info: Option<>, // is seperate object
    pub lock_explanation: Option<String>,
    pub discussion_topic: Option<Discussion>,
    // pub overrides: (),
    pub omit_from_final_grade: Option<bool>,
    // pub grader_count: u64,
    pub final_grader_id: Option<u64>,
    pub allowed_attemps: Option<i64>,
    pub post_manually: bool,
    // pub score_statistics: (),
    pub can_submit: Option<bool>,
    // pub annotatable_attachment_id: Option<u64>,
    pub require_lockdown_browser: Option<bool>,
    // pub important_dates: Option<bool>,
    pub graded_submissions_exist: bool,
    pub is_quiz_assignment: bool,
}

impl crate::types::ResponseType for Assignment {}

impl std::cmp::PartialEq for Assignment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Assignment {
    pub fn view(&self, config: &canvas_cli_config::Config) -> String {
        let due_at = match self.due_at {
            Some(date) => format!(
                "\nDue At: {}",
                DateTime::<Local>::from(date).format(&crate::datetime_format(config))
            ),
            None => String::default(),
        };
        let lock_explanation = match (self.lock_explanation.to_owned(), self.locked_for_user) {
            (Some(explanation), true) => format!("\nAssignment is Locked!\n{}", explanation),
            (_, true) => "\nAssignment is Locked!".to_owned(),
            _ => String::default(),
        };
        let description = match self.description.to_owned() {
            Some(description) => {
                format!(
                    "\n\n{}",
                    html2text::from_read(&mut description.as_bytes(), 40)
                )
            }
            None => String::default(),
        };
        format!(
            "[{}] {}\nHTML Link: {}{}{}{}", // TODO: Investigate formatting w/ termcolor
            self.id, self.name, self.html_url, due_at, lock_explanation, description
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GradingType {
    PassFail,
    Percent,
    LetterGrade,
    GpaScale,
    Points,
    NotGraded,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AllowedSubmissionType {
    DiscussionTopic,
    OnlineQuiz,
    OnPaper,
    None,
    ExternalTool,
    OnlineTextEntry,
    OnlineUrl,
    OnlineUpload,
    MediaRecording,
    StudentAnnotation,
    NotGraded,
}

impl std::fmt::Display for AllowedSubmissionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AllowedSubmissionType::OnlineTextEntry => "Text Entry",
            AllowedSubmissionType::OnlineUrl => "Online URL",
            AllowedSubmissionType::OnlineUpload => "Previously Attached File",
            _ => "Not Supported!",
        };

        write!(f, "{}", str)
    }
}
