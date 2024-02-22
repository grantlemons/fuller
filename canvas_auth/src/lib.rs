use oauth2::AccessToken;

pub mod oauth2_mod;
pub mod token;

pub async fn connect() -> Result<AccessToken, ()> {
    crate::token::connect().await
}
