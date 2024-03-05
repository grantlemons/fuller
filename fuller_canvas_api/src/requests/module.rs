use crate::types::Module;
use crate::ApiError;
use fuller_config::Config;
use reqwest::Client;
use std::borrow::Borrow;

pub async fn list_course_modules(
    client: Client,
    config: impl Borrow<Config>,
    course_id: u64,
) -> Result<Vec<Module>, ApiError> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/courses/{course_id}/modules"),
        None,
    )
    .await
}

pub async fn list_course_modules_with_items(
    client: Client,
    config: impl Borrow<Config>,
    course_id: u64,
) -> Result<Vec<Module>, ApiError> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/courses/{course_id}/modules"),
        Some(&[("include", "items")]),
    )
    .await
}
