use crate::types::Profile;
use reqwest::{Client, Result};

pub async fn get_profile(
    client: Client,
    config: &canvas_cli_config::Config,
    user_id: u64,
) -> Result<Profile> {
    super::get_generic(
        client,
        config,
        &format!("/api/v1/users/{user_id}/profile"),
        None,
    )
    .await
}

pub async fn get_self(client: Client, config: &canvas_cli_config::Config) -> Result<Profile> {
    super::get_generic(client, config, "/api/v1/users/self/profile", None).await
}
