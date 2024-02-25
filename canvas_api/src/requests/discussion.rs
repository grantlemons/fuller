use crate::types::{Discussion, DiscussionEntry};
use reqwest::{Client, Result};

pub async fn list_course_discussions(client: Client, course_id: u64) -> Result<Vec<Discussion>> {
    super::get_generic(
        client,
        &format!("/api/v1/courses/{course_id}/discussion_topics"),
        None,
    )
    .await
}

pub async fn list_course_discussion_replies(
    client: Client,
    course_id: u64,
    discussion_id: u64,
) -> Result<Vec<DiscussionEntry>> {
    super::get_generic(
        client,
        &format!("/api/v1/courses/{course_id}/discussion_topics/{discussion_id}/entry_list"),
        None,
    )
    .await
}
