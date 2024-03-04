use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Conversation {
    pub id: u64,
    pub subject: String,
    pub last_message: String,
    pub subscribed: bool,
    pub private: bool,
    pub starred: bool,
    pub tags: Option<Vec<String>>,
    pub message: Vec<ConversationMessage>
}

#[derive(Debug, Deserialize)]
pub struct ConversationMessage {
    pub id: u64,
    pub body: String,
    pub author_id: String,
    pub generated: bool,
    pub media_comment: String,
    pub forwarded_messages: Option<ConversationMessage>,
    pub attachments: Option<Vec<ConversationAttachment>>
    
}

#[derive(Debug, Deserialize)]
pub struct ConversationAttachment {
    pub id: u64,
    pub display_name: String,
    pub uuid: String,
}
