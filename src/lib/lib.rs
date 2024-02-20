use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Submission {
    assignment_id: u64,
    // assignment: Option<Assignment>,
    course: Option<Course>,
    attempt: u64,
    body: Option<String>,
    grade: String,
    grade_matches_current_submission: bool,
    html_url: String,
    preview_url: String,
    score: f32,
    submission_comments: Option<Vec<SubmissionComment>>,
    submission_type: SubmissionType,
    submitted_at: String, // ISO 8601
    url: Option<String>,
    user_id: u64,
    grader_id: Option<u64>,
    graded_at: Option<String>, // ISO 8601
    // user: Option<User>,
    late: bool,
    assignment_visible: bool,
    excused: bool,
    missing: bool,
    late_policy_status: LateStatus,
    points_deducted: f32,
    seconds_late: u64,
    workflow_state: String,
    extra_attempts: u64,
    anonymous_id: Option<String>,
    posted_at: String, // ISO 8601
    read_status: Option<String>,
    redo_request: bool,
}

#[derive(Debug, Deserialize)]
struct Assignment {
    id: u64,
    name: String,
    description: String,
    created_at: String, // ISO 8601
    updated_at: String, // ISO 8601
    due_at: String,     // ISO 8601
    lock_at: String,    // ISO 8601
    unlock_at: String,  // ISO 8601
    has_overrides: bool,
    all_dates: Option<()>,
    course_id: u64,
    html_url: String,
    submissions_download_url: String,
    assignment_group_id: u64,
    due_date_required: bool,
    allowed_extensions: Vec<String>,
    max_name_length: u64,
    turnitin_enabled: bool,
    vericite_enabled: bool,
    turnitin_settings: Option<()>,
    grade_group_students_individually: bool,
    external_tool_tag_attributes: Option<()>,
    peer_reviews: bool,
    automatic_peer_reviews: bool,
    peer_review_count: u64,
    peer_reviews_assign_at: String, // ISO 8601
    intra_group_peer_reviews: bool,
    group_category_id: u64,
    // grading info here
    position: u64,
    post_to_sis: bool,
    // 3rd party integration data here
    points_possible: f32,
    submission_types: Vec<SubmissionType>,
    has_submitted_submissions: bool,
    grading_type: GradingType,
    grading_standard_id: Option<u64>,
    published: bool,
    unpublishable: bool,
    only_visible_to_overrides: bool,
    locked_for_user: bool,
    lock_info: Option<String>,
    lock_explanation: Option<String>,
    discussion_topic: Option<String>,
    // freeze settings
    submission: Option<Submission>,
    // rubric settings
    assignment_visibility: Option<Vec<String>>,
    overrides: (),
    omit_from_final_grade: Option<bool>,
    moderated_grading: bool,
    grader_count: u64,
    final_grader_id: u64,
    grader_comments_visible_to_graders: bool,
    graders_anonymous_to_graders: bool,
    grader_names_visible_to_final_grader: bool,
    anonymous_grading: bool,
    allowed_attemps: u64,
    post_manually: bool,
    score_statistics: (),
    can_submit: Option<bool>,
    annotatable_attachment_id: Option<u64>,
    anonymize_students: Option<bool>,
    require_lockdown_browser: Option<bool>,
    important_dates: Option<bool>,
    anonymous_peer_reviews: bool,
    anonymous_instructor_annotations: bool,
    graded_submissions_exist: bool,
    is_quiz_assignment: bool,
    in_closed_grading_period: bool,
    can_duplicate: bool,
    original_course_id: Option<u64>,
    original_assignment_id: Option<u64>,
    original_lti_resource_link_id: Option<u64>,
    original_assignment_name: Option<String>,
    original_quiz_id: Option<u64>,
    workflow_state: String,
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

#[derive(Debug, Serialize)]
enum TodoType {
    Submitting,
    Grading,
}

// GET /api/v1/courses/:course_id/todo
//
// Example Response
// [{
//   'type': 'submitting', (submitting|grading)
//   'assignment': Assignment Object,
//   'ignore': Url, (String)
//   'ignore_permanently': Url, (String)
//   'html_url': Url, (String)
//   'context_type': 'course', (course|group)
//   'course_id': 1,
// }]
#[derive(Debug, Deserialize)]
struct TodoBody {
    #[serde(alias = "type")]
    todo_type: TodoType,
    assignment: Assignment,
    #[serde(alias = "ignore")]
    ignore_url: String,
    #[serde(alias = "ignore_permanently")]
    ignore_permanently_url: String,
    html_url: String,
    context_type: ContextType,
    group_id: u64,
}
