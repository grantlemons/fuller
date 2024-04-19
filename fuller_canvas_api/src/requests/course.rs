use crate::types::{Course, Todo};
use crate::ApiError;
use fuller_config::Config;
use reqwest::Client;
use std::borrow::Borrow;

pub async fn get_courses(
    client: Client,
    config: impl Borrow<Config>,
) -> Result<Vec<Course>, ApiError> {
    super::get_generic(
        client,
        config.borrow(),
        "/api/v1/courses",
        Some(&[("enrollment_state", "active")]),
    )
    .await
}

pub async fn get_course(
    client: Client,
    config: impl Borrow<Config>,
    course_id: u64,
) -> Result<Course, ApiError> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/courses/{course_id}"),
        None,
    )
    .await
}

pub async fn get_course_todo(
    client: Client,
    config: impl Borrow<Config>,
    course_id: u64,
) -> Result<Vec<Todo>, ApiError> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/courses/{course_id}/todo"),
        None,
    )
    .await
}
