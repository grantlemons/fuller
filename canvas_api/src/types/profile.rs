use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub id: u64,
    pub name: String,
    pub short_name: String,
    pub sortable_name: String,
    pub title: Option<String>,
    pub bio: Option<String>,
    pub primary_email: String,
    pub login_id: String,
    pub avatar_url: String,
    pub time_zone: String,
}

impl crate::types::ResponseType for Profile {}
