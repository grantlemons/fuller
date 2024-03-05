pub mod oauth2_mod;
pub mod token;
pub mod types {
    pub mod access_token;
    pub mod errors;

    pub use access_token::AccessToken;
    pub use errors::AuthError;
}
use tracing::info;
pub use types::*;

pub async fn connect(config: &fuller_config::Config) -> Result<AccessToken, AuthError> {
    if config.network.token.is_some() {
        crate::token::connect(config).await
    } else {
        info!("Token not configured: Attempting OAuth Authorization Process");
        crate::oauth2_mod::connect(config).await
    }
}
