use crate::types::Course;
use reqwest::{Client, Result};

pub async fn get_courses(client: Client) -> Result<Vec<Course>> {
    super::get_generic(client, "/api/v1/courses", None).await
}
