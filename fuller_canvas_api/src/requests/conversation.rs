use crate::types::{Conversation, ConversationOverview, UnreadCount};
use crate::ApiError;
use fuller_config::Config;
use reqwest::Client;
use std::borrow::Borrow;

pub async fn get_conversation(
    client: Client,
    config: impl Borrow<Config>,
    conversation_id: u64,
) -> Result<Conversation, ApiError> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/conversations/{conversation_id}"),
        None,
    )
    .await
}

pub async fn list_conversations(
    client: Client,
    config: impl Borrow<Config>,
) -> Result<Vec<ConversationOverview>, ApiError> {
    super::get_generic(client, config.borrow(), "/api/v1/conversations", None).await
}

pub async fn unread_count(client: Client, config: impl Borrow<Config>) -> Result<u64, ApiError> {
    Ok(client
        .get(&format!(
            "{}/api/v1/conversations/unread_count",
            config.borrow().network.url
        ))
        .send()
        .await?
        .json::<UnreadCount>()
        .await?
        .unread_count)
}
