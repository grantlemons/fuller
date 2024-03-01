use canvas_cli_config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Error with OAuth Setup Process")]
    OAuthParsingError(#[from] oauth2::url::ParseError),
    #[error("Error with OAuth Transaction Process")]
    OAuthTransactionError,
    #[error("Error fetching token from Config!")]
    ConfigError(#[from] ConfigError),
    #[error("Null token")]
    NullToken,
}
