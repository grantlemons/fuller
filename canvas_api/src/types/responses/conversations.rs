use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConversationOverview {
    pub id: u64,
    pub subject: String,
    #[serde(alias = "workflow_state")]
    pub state: ConversationState,
    #[serde(alias = "last_message")]
    pub preview: String,
    pub audience: Vec<u64>,
    pub last_message_at: DateTime<Utc>,
    pub message_count: u64,
    pub subscribed: bool,
    pub private: bool,
    pub starred: bool,
    pub participants: Vec<ConversationParticipant>,
    pub context_name: String,
}

impl std::cmp::PartialEq for ConversationOverview {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl crate::types::ResponseType for ConversationOverview {}

#[derive(Debug, Deserialize)]
pub struct Conversation {
    pub id: u64,
    pub subject: String,
    #[serde(alias = "workflow_state")]
    pub state: ConversationState,
    #[serde(alias = "last_message")]
    pub preview: String,
    pub audience: Vec<u64>,
    pub last_message_at: DateTime<Utc>,
    pub message_count: u64,
    pub subscribed: bool,
    pub private: bool,
    pub starred: bool,
    pub participants: Vec<ConversationParticipant>,
    pub messages: Vec<ConversationMessage>,
}

impl std::cmp::PartialEq for Conversation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl crate::types::ResponseType for Conversation {}

#[derive(Debug, Deserialize)]
pub struct ConversationMessage {
    pub id: u64,
    pub body: String,
    pub author_id: u64,
    pub generated: bool,
    // pub media_comment: Option<()>,
    pub forwarded_messages: Option<Vec<ConversationMessage>>,
    pub attachments: Option<Vec<ConversationAttachment>>,
}

#[derive(Debug, Deserialize)]
pub struct ConversationAttachment {
    pub id: u64,
    pub display_name: String,
    #[serde(alias = "content-type")]
    pub content_type: String, // MIME type
    pub filename: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ConversationParticipant {
    pub id: u64,
    pub name: String,
    pub full_name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct UnreadCount {
    pub unread_count: u64,
}

impl crate::types::ResponseType for UnreadCount {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversationState {
    Read,
    Unread,
    Archived,
}
