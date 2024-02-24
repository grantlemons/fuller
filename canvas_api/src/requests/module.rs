use crate::types::{Module, ModuleItem};
use reqwest::{Client, Result};

pub async fn list_course_modules(client: Client, course_id: u64) -> Result<Vec<Module>> {
    super::get_generic(
        client,
        &format!("/api/v1/courses/{course_id}/modules"),
        None,
    )
    .await
}

pub async fn list_course_module_items(
    client: Client,
    course_id: u64,
    module_id: u64,
) -> Result<Vec<ModuleItem>> {
    super::get_generic(
        client,
        &format!("/api/v1/courses/{course_id}/modules/{module_id}/items"),
        None,
    )
    .await
}
