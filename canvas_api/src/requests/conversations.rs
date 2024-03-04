use crate::types::{Assignment, Submission, SubmissionRequest};
use reqwest::{Client, Result};

pub async fn get_conversation<T: std::borrow::Borrow<canvas_cli_config::Config>>(
    client: Client,
    config: T,
    conversation_id: u64,
) -> Result<Submission> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/conversations/{conversation_id}"),
        None,
    )
    .await
}

pub async fn list_conversations<T: std::borrow::Borrow<canvas_cli_config::Config>>(
    client: Client,
    config: T,
) -> Result<Vec<Assignment>> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/conversations"),
        None,
    )
    .await
}
