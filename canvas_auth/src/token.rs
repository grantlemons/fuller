use crate::AccessToken;
use crate::AuthError;
use dotenv::dotenv;

pub async fn connect() -> Result<AccessToken, AuthError> {
    dotenv().ok();
    let env_token = std::env::var("CANVAS_ACCESS_TOKEN")?.to_owned();
    let access_token = AccessToken::new(env_token);

    Ok(access_token)
}
