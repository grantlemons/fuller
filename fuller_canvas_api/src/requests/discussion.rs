use crate::types::{Discussion, DiscussionEntry};
use crate::ApiError;
use fuller_config::Config;
use reqwest::Client;
use std::borrow::Borrow;

pub async fn list_course_discussions(
    client: Client,
    config: impl Borrow<Config>,
    course_id: u64,
) -> Result<Vec<Discussion>, ApiError> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/courses/{course_id}/discussion_topics"),
        None,
    )
    .await
}

pub async fn list_course_discussion_replies(
    client: Client,
    config: impl Borrow<Config>,
    course_id: u64,
    discussion_id: u64,
) -> Result<Vec<DiscussionEntry>, ApiError> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/courses/{course_id}/discussion_topics/{discussion_id}/entry_list"),
        None,
    )
    .await
}
