use crate::types::Course;
use reqwest::{Client, Result};

pub async fn get_courses<T: std::borrow::Borrow<canvas_cli_config::Config>>(
    client: Client,
    config: T,
) -> Result<Vec<Course>> {
    super::get_generic(
        client,
        config.borrow(),
        "/api/v1/courses",
        Some(&[("enrollment_state", "active")]),
    )
    .await
}
