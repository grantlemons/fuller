use crate::types::Course;
use canvas_cli_config::Config;
use reqwest::{Client, Result};
use std::borrow::Borrow;

pub async fn get_courses(client: Client, config: impl Borrow<Config>) -> Result<Vec<Course>> {
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
) -> Result<Course> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/courses/{course_id}"),
        None,
    )
    .await
}
