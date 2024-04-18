use crate::AccessToken;
use crate::AuthError;
use fuller_config::Config;
use tracing::info;

#[tracing::instrument]
pub async fn connect(config: &Config) -> Result<AccessToken, AuthError> {
    let token = match &config.network.token {
        Some(t) if !t.secret().is_empty() => t.to_owned(),
        _ => return Err(AuthError::NullToken),
    };
    let access_token = AccessToken::from(token);

    info!("Token Auth Process Complete!");
    Ok(access_token)
}
