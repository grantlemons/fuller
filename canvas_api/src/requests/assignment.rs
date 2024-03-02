use crate::types::{Assignment, Submission, SubmissionRequest};
use reqwest::{Client, Result};

pub async fn get_submissions<T: std::borrow::Borrow<canvas_cli_config::Config>>(
    client: Client,
    config: T,
    course_id: u64,
    assignment_id: u64,
) -> Result<Submission> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/courses/{course_id}/assignments/{assignment_id}/submissions"),
        None,
    )
    .await
}

pub async fn submit_assignment<T: std::borrow::Borrow<canvas_cli_config::Config>>(
    client: Client,
    config: T,
    course_id: u64,
    assignment_id: u64,
    submission_request: SubmissionRequest,
) -> Result<()> {
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

pub async fn get_assignment<T: std::borrow::Borrow<canvas_cli_config::Config>>(
    client: Client,
    config: T,
    course_id: u64,
    assignment_id: u64,
) -> Result<Assignment> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/courses/{course_id}/assignments/{assignment_id}"),
        None,
    )
    .await
}

pub async fn list_course_assignments<T: std::borrow::Borrow<canvas_cli_config::Config>>(
    client: Client,
    config: T,
    course_id: u64,
) -> Result<Vec<Assignment>> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/courses/{course_id}/assignments"),
        None,
    )
    .await
}
