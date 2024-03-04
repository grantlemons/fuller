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

impl super::Viewable for Assignment {
    fn view(&self, config: &canvas_cli_config::Config) -> String {
        let due_at = match self.due_at {
            Some(date) => format!(
                "\nDue At: {}",
                DateTime::<Local>::from(date).format(&crate::datetime_format(config))
            ),
            None => String::default(),
        };
        let allowed_extensions = match self.allowed_extensions.clone() {
            Some(vec)
                if self
                    .submission_types
                    .contains(&AllowedSubmissionType::OnlineUpload) =>
            {
                format!("\n\nAllowed Extensions =============================================================
{}
================================================================================", display_vec(vec))
            }
            _ => String::default(),
        };
        let lock_explanation = match (
            self.lock_explanation.to_owned(),
            self.locked_for_user,
            self.unlock_at,
        ) {
            (Some(explanation), true, None) => format!("\nAssignment is Locked!\n{}", explanation),
            (Some(explanation), true, Some(unlock_at)) => {
                format!(
                    "\n\nAssignment is Locked!\n{}\nWill unlock at {}",
                    html2text::from_read(&mut explanation.as_bytes(), 80),
                    DateTime::<Local>::from(unlock_at).format(&crate::datetime_format(config))
                )
            }
            (_, true, None) => "\n\nAssignment is Locked!".to_owned(),
            (_, true, Some(unlock_at)) => format!(
                "\n\nAssignment is Locked!\nWill unlock at {}",
                DateTime::<Local>::from(unlock_at).format(&crate::datetime_format(config))
            ),
            _ => String::default(),
        };
        let description = match self.description.to_owned() {
            Some(description) => {
                format!(
                    "\n\n{}",
                    html2text::from_read(&mut description.as_bytes(), 80)
                )
            }
            None => String::default(),
        };
        format!(
            "[{}] {}
HTML Link: {}{}{}{}
Allowed Submission Types =======================================================
{}
================================================================================{}", // TODO: Investigate formatting w/ termcolor
            self.id,
            self.name,
            self.html_url,
            due_at,
            lock_explanation,
            description,
            display_vec(self.submission_types.to_owned()),
            allowed_extensions,
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

#[derive(Debug, Deserialize, PartialEq, Clone)]
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
            AllowedSubmissionType::DiscussionTopic => "Discussion Topic",
            AllowedSubmissionType::OnlineQuiz => "Online Quiz (Not Supported!)",
            AllowedSubmissionType::OnPaper => "On Paper (Not Supported!)",
            AllowedSubmissionType::None => "No Submission Needed!",
            AllowedSubmissionType::ExternalTool => "External Tool (Not Supported!)",
            AllowedSubmissionType::MediaRecording => "Media Recording (Not Supported!)",
            AllowedSubmissionType::StudentAnnotation => "Student Annotation (Not Supported!)",
            AllowedSubmissionType::NotGraded => "Assignment Not Graded!",
        };

        write!(f, "{}", str)
    }
}

fn display_vec<T: std::fmt::Display>(vec: Vec<T>) -> String {
    let mut res_str: String;
    if let Some(initial_value) = vec.first() {
        res_str = initial_value.to_string();
    } else {
        return String::default();
    }

    let mut line_index = 0;
    for v in vec[1..].iter() {
        // newline if 80 chars
        let new_string = v.to_string();
        if res_str.len() + new_string.len() + 1 - line_index >= 80 {
            res_str.push_str(",\n");
            line_index = res_str.len();

            res_str.push_str(&new_string);
        } else {
            res_str.push_str(&format!(", {}", new_string));
        }
    }

    res_str
}
