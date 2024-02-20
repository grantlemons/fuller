#![allow(dead_code)]

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Course; // TODO: Expand
#[derive(Debug, Deserialize)]
struct User; // TODO: Expand
#[derive(Debug, Deserialize)]
struct SubmissionComment; // TODO: Expand
#[derive(Debug, Deserialize)]
struct LateStatus; // TODO: Expand

#[derive(Debug, Deserialize)]
struct Submission {
    pub assignment_id: u64,
    pub course: Option<Course>,
    pub attempt: u64,
    pub body: Option<String>,
    pub grade: String,
    pub grade_matches_current_submission: bool,
    pub html_url: String,
    pub preview_url: String,
    pub score: f32,
    pub submission_comments: Option<Vec<SubmissionComment>>,
    pub submission_type: SubmissionType,
    pub submitted_at: DateTime<Local>,
    pub url: Option<String>,
    pub user_id: u64,
    pub user: Option<User>,
    pub grader_id: Option<u64>,
    pub graded_at: Option<DateTime<Local>>,
    pub late: bool,
    pub assignment_visible: bool,
    pub excused: bool,
    pub missing: bool,
    pub late_policy_status: LateStatus,
    pub points_deducted: f32,
    pub seconds_late: u64,
}

#[derive(Debug, Deserialize)]
struct Assignment {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub due_at: DateTime<Local>,
    pub lock_at: DateTime<Local>,
    pub unlock_at: DateTime<Local>,
    pub has_overrides: bool,
    pub all_dates: Option<()>,
    pub course_id: u64,
    pub html_url: String,
    pub submissions_download_url: String,
    pub assignment_group_id: u64,
    pub due_date_required: bool,
    pub allowed_extensions: Vec<String>,
    pub max_name_length: u64,
    pub turnitin_enabled: bool,
    pub vericite_enabled: bool,
    pub turnitin_settings: Option<()>,
    pub grade_group_students_individually: bool,
    pub external_tool_tag_attributes: Option<()>,
    pub peer_reviews: bool,
    pub automatic_peer_reviews: bool,
    pub peer_review_count: u64,
    pub peer_reviews_assign_at: DateTime<Local>,
    pub intra_group_peer_reviews: bool,
    pub group_category_id: u64,
    pub position: u64,
    pub post_to_sis: bool,
    pub points_possible: f32,
    pub submission_types: Vec<SubmissionType>,
    pub has_submitted_submissions: bool,
    pub grading_type: GradingType,
    pub only_visible_to_overrides: bool,
    pub locked_for_user: bool,
    pub lock_info: Option<String>,
    pub lock_explanation: Option<String>,
    pub discussion_topic: Option<String>,
    pub overrides: (),
    pub omit_from_final_grade: Option<bool>,
    pub grader_count: u64,
    pub final_grader_id: u64,
    pub allowed_attemps: u64,
    pub post_manually: bool,
    pub score_statistics: (),
    pub can_submit: Option<bool>,
    pub annotatable_attachment_id: Option<u64>,
    pub require_lockdown_browser: Option<bool>,
    pub important_dates: Option<bool>,
    pub graded_submissions_exist: bool,
    pub is_quiz_assignment: bool,
}

#[derive(Debug, Deserialize)]
enum GradingType {
    #[serde(alias = "pass_fail")]
    PassFail,
    #[serde(alias = "percent")]
    Percent,
    #[serde(alias = "letter_grade")]
    LetterGrade,
    #[serde(alias = "gpa_scale")]
    GpaScale,
    #[serde(alias = "points")]
    Points,
}

#[derive(Debug, Deserialize)]
enum SubmissionType {
    #[serde(alias = "discussion_topic")]
    DiscussionTopic,
    #[serde(alias = "online_quiz")]
    OnlineQuiz,
    #[serde(alias = "on_paper")]
    OnPaper,
    #[serde(alias = "none")]
    None,
    #[serde(alias = "external_tool")]
    ExternalTool,
    #[serde(alias = "online_text_entry")]
    OnlineTextEntry,
    #[serde(alias = "online_url")]
    OnlineURL,
    #[serde(alias = "online_upload")]
    OnlineUpload,
    #[serde(alias = "media_recording")]
    MediaRecording,
    #[serde(alias = "student_annotation")]
    StudentAnnotation,
}

#[derive(Debug, Deserialize)]
enum ContextType {
    Course,
    Group,
}

#[derive(Debug, Deserialize)]
enum TodoType {
    Submitting,
    Grading,
}

// GET /api/v1/courses/:course_id/todo
#[derive(Debug, Deserialize)]
struct TodoBody {
    #[serde(alias = "type")]
    pub todo_type: TodoType,
    pub assignment: Assignment,
    #[serde(alias = "ignore")]
    pub ignore_url: String,
    #[serde(alias = "ignore_permanently")]
    pub ignore_permanently_url: String,
    pub html_url: String,
    pub context_type: ContextType,
    pub group_id: u64,
}
