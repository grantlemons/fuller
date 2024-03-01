use crate::AccessToken;
use crate::AuthError;
use canvas_cli_config::Config;
use tracing::info;

#[tracing::instrument]
pub async fn connect(config: &Config) -> Result<AccessToken, AuthError> {
    let access_token = AccessToken::from(config.network.token.to_owned());

    info!("Token Auth Proccess Complete!");
    Ok(access_token)
}
