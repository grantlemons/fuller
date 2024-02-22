use crate::types::{Assignment, Submission};
use reqwest::{Client, Result};

pub async fn get_submissions(
    client: Client,
    course_id: u64,
    assignment_id: u64,
) -> Result<Submission> {
    super::get_generic(
        client,
        &format!("/api/v1/courses/{course_id}/assignments/{assignment_id}/submissions"),
        None,
    )
    .await
}

pub async fn get_assignment(
    client: Client,
    course_id: u64,
    assignment_id: u64,
) -> Result<Assignment> {
    super::get_generic(
        client,
        &format!("/api/v1/courses/{course_id}/assignments/{assignment_id}"),
        None,
    )
    .await
}

pub async fn list_assignments(client: Client, course_id: u64) -> Result<Vec<Assignment>> {
    super::get_generic(
        client,
        &format!("/api/v1/courses/{course_id}/assignments"),
        None,
    )
    .await
}
