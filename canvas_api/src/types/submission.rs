use super::Course;
use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Submission {
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

impl crate::types::ResponseType for Submission {}

#[derive(Debug, Deserialize)]
pub enum SubmissionType {
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
pub struct SubmissionComment; // TODO: Expand
#[derive(Debug, Deserialize)]
pub struct LateStatus; // TODO: Expand
