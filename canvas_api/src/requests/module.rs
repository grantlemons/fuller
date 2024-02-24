use crate::types::Module;
use reqwest::{Client, Result};

pub async fn list_course_modules(client: Client, course_id: u64) -> Result<Vec<Module>> {
    super::get_generic(
        client,
        &format!("/api/v1/courses/{course_id}/modules"),
        None,
    )
    .await
}

pub async fn list_course_modules_with_items(client: Client, course_id: u64) -> Result<Vec<Module>> {
    super::get_generic(
        client,
        &format!("/api/v1/courses/{course_id}/modules"),
        Some(&[("include", "items")]),
    )
    .await
}
