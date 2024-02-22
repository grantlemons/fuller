use crate::types::Profile;
use reqwest::{Client, Result};

pub async fn get_profile(client: Client, user_id: u64) -> Result<Profile> {
    super::get_generic(client, &format!("/api/v1/users/{user_id}/profile"), None).await
}

pub async fn get_self(client: Client) -> Result<Profile> {
    super::get_generic(client, &format!("/api/v1/users/self/profile"), None).await
}
