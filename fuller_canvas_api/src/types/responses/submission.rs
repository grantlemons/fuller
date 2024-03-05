use super::Course;
use chrono::{DateTime, Utc};
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
    pub submitted_at: DateTime<Utc>,
    pub url: Option<String>,
    pub user_id: u64,
    pub grader_id: Option<u64>,
    pub graded_at: Option<DateTime<Utc>>,
    pub late: bool,
    pub assignment_visible: bool,
    pub excused: bool,
    pub missing: bool,
    pub late_policy_status: Option<LateStatus>,
    pub points_deducted: f32,
    pub seconds_late: u64,
}

impl crate::types::ResponseType for Submission {}

impl std::cmp::PartialEq for Submission {
    fn eq(&self, other: &Self) -> bool {
        self.assignment_id == other.assignment_id && self.attempt == other.attempt
    }
}

impl std::fmt::Display for Submission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Attempt [{}]", self.attempt)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmissionType {
    OnlineQuiz,
    OnlineTextEntry,
    OnlineUrl,
    OnlineUpload,
    MediaRecording,
    StudentAnnotation,
}

#[derive(Debug, Deserialize)]
pub struct SubmissionComment {
    pub id: u64,
    pub author_id: u64,
    pub author_name: String,
    pub comment: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LateStatus {
    Late,
    Missing,
    Extended,
    None,
}
