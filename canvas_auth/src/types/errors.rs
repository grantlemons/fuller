use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Error with OAuth Transaction Process")]
    OAuthError,
    #[error("Error fetching token from Environment! Try setting the CANVAS_ACCESS_TOKEN environment variable.")]
    EnvError,
}

impl From<std::env::VarError> for AuthError {
    fn from(_: std::env::VarError) -> Self {
        Self::EnvError
    }
}

impl From<oauth2::url::ParseError> for AuthError {
    fn from(_: oauth2::url::ParseError) -> Self {
        Self::OAuthError
    }
}
