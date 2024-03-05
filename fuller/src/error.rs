use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Network request error")]
    Request(#[from] reqwest::Error),
    #[error("User input error")]
    Input(#[from] inquire::error::InquireError),
    #[error("Configuration Error")]
    Config(#[from] fuller_config::ConfigError),
    #[error("Invalid config setting")]
    InvalidConfig(&'static str),
    #[error("More options required for no config flag")]
    NeedMoreOptions,
}
