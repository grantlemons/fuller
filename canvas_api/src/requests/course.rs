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
