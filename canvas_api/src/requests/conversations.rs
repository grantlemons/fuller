use crate::types::{Conversation, ConversationOverview};
use canvas_cli_config::Config;
use reqwest::{Client, Result};
use std::borrow::Borrow;

pub async fn get_conversation(
    client: Client,
    config: impl Borrow<Config>,
    conversation_id: u64,
) -> Result<Conversation> {
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
) -> Result<Vec<Conversation>> {
    super::get_generic(client, config.borrow(), "/api/v1/conversations", None).await
}
