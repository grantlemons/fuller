use crate::types::Profile;
use reqwest::{Client, Result};

pub async fn get_profile<T: std::borrow::Borrow<canvas_cli_config::Config>>(
    client: Client,
    config: T,
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

pub async fn get_self<T: std::borrow::Borrow<canvas_cli_config::Config>>(
    client: Client,
    config: T,
) -> Result<Profile> {
    super::get_generic(client, config.borrow(), "/api/v1/users/self/profile", None).await
}
