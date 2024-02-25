use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Discussion {
    pub id: u64,
    pub title: String,
    pub message: String, // possibly nullable?
    pub html_url: String,
    pub posted_at: Option<DateTime<Utc>>,
    pub last_reply_at: Option<DateTime<Utc>>,
    pub require_initial_post: Option<bool>,
    pub user_can_see_posts: bool,
    pub discussion_subentry_count: u64,
    pub read_state: ReadState,
    pub unread_count: u64,
    pub subscribed: bool,
    pub assignment_id: Option<u64>,
    pub lock_at: Option<DateTime<Utc>>,
    pub locked: bool,
    pub pinned: bool,
    pub locked_for_user: bool,
    pub user_name: Option<String>,
    pub discussion_type: DiscussionType,
    // pub attachments: Vec<Attachment>,
}

impl crate::types::ResponseType for Discussion {}

impl std::cmp::PartialEq for Discussion {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Deserialize)]
pub struct DiscussionEntry {
    pub id: u64,
    pub user_id: Option<u64>,
    pub user_name: Option<String>,
    pub message: Option<String>,
    pub read_state: ReadState,
    pub create_at: DateTime<Utc>,
    /// The user_id, user_name, and message will not be returned for deleted entries.
    pub deleted: bool,
}

impl crate::types::ResponseType for DiscussionEntry {}

impl std::cmp::PartialEq for DiscussionEntry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadState {
    Read,
    Unread,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscussionType {
    SideComment,
    Threaded,
}
