use crate::AccessToken;
use crate::AuthError;
use dotenv::dotenv;
use tracing::info;

#[tracing::instrument]
pub async fn connect() -> Result<AccessToken, AuthError> {
    dotenv().ok();
    let env_token = std::env::var("CANVAS_ACCESS_TOKEN")?.to_owned();
    let access_token = AccessToken::new(env_token);

    info!("Environment Token Auth Proccess Complete!");
    Ok(access_token)
}
