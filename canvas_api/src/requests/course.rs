use crate::types::Course;
use reqwest::{Client, Result};

pub async fn get_courses(
    client: Client,
    config: &canvas_cli_config::Config,
) -> Result<Vec<Course>> {
    super::get_generic(
        client,
        config,
        "/api/v1/courses",
        Some(&[("enrollment_state", "active")]),
    )
    .await
}
