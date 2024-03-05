use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Api error")]
    Api(#[from] fuller_canvas_api::ApiError),
    #[error("User input error")]
    Input(#[from] inquire::error::InquireError),
    #[error("Configuration Error")]
    Config(#[from] fuller_config::ConfigError),
    #[error("Authentication Error")]
    Auth(#[from] fuller_canvas_auth::AuthError),
    #[error("Invalid config setting")]
    InvalidConfig(&'static str),
    #[error("More options required for no config flag")]
    NeedMoreOptions,
}
