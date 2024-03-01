use crate::AccessToken;
use crate::AuthError;
use canvas_cli_config::Config;
use tracing::info;

#[tracing::instrument]
pub async fn connect(config: &Config) -> Result<AccessToken, AuthError> {
    let token = match &config.network.token {
        Some(t) => t.to_owned(),
        None => return Err(AuthError::NullToken),
    };
    let access_token = AccessToken::from(token);

    info!("Token Auth Proccess Complete!");
    Ok(access_token)
}
