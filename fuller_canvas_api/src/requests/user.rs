use crate::types::Profile;
use fuller_config::Config;
use reqwest::{Client, Result};
use std::borrow::Borrow;

pub async fn get_profile(
    client: Client,
    config: impl Borrow<Config>,
    user_id: u64,
) -> Result<Profile> {
    super::get_generic(
        client,
        config.borrow(),
        &format!("/api/v1/users/{user_id}/profile"),
        None,
    )
    .await
}

pub async fn get_self(client: Client, config: impl Borrow<Config>) -> Result<Profile> {
    super::get_generic(client, config.borrow(), "/api/v1/users/self/profile", None).await
}
