pub mod oauth2_mod;
pub mod token;
pub mod types {
    pub mod access_token;
    pub mod errors;

    pub use access_token::AccessToken;
    pub use errors::AuthError;
}
pub use types::*;

pub async fn connect() -> Result<AccessToken, AuthError> {
    crate::token::connect().await
}
