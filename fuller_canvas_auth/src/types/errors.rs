use fuller_config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    // #[error("Error with OAuth Setup Process")]
    // OAuthParsing(#[from] oauth2::url::ParseError),
    #[error("Error with OAuth Transaction Process")]
    OAuthTransaction,
    #[error("Error fetching token from Config!")]
    Config(#[from] ConfigError),
    #[error("Null token")]
    NullToken,
}
