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

impl std::cmp::PartialEq for Profile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
