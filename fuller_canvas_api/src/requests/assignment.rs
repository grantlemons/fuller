use crate::types::{Assignment, Submission, SubmissionRequest};
use crate::ApiError;
use fuller_config::Config;
use reqwest::Client;
use std::borrow::Borrow;

pub async fn get_submissions(
    client: Client,
    config: impl Borrow<Config>,
    course_id: u64,
    assignment_id: u64,
) -> Result<Submission, ApiError> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/courses/{course_id}/assignments/{assignment_id}/submissions"),
        None,
    )
    .await
}

pub async fn submit_assignment(
    client: Client,
    config: impl Borrow<Config>,
    course_id: u64,
    assignment_id: u64,
    submission_request: SubmissionRequest,
) -> Result<(), ApiError> {
    client
        .post(&format!(
            "{}/api/v1/courses/{course_id}/assignments/{assignment_id}/submissions",
            config.borrow().network.url
        ))
        .json(&submission_request)
        .send()
        .await?;

    Ok(())
}

pub async fn get_assignment(
    client: Client,
    config: impl Borrow<Config>,
    course_id: u64,
    assignment_id: u64,
) -> Result<Assignment, ApiError> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/courses/{course_id}/assignments/{assignment_id}"),
        None,
    )
    .await
}

pub async fn list_course_assignments(
    client: Client,
    config: impl Borrow<Config>,
    course_id: u64,
) -> Result<Vec<Assignment>, ApiError> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/courses/{course_id}/assignments"),
        Some(&[("order_by", "due_at")]),
    )
    .await
}
