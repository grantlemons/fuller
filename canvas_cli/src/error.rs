use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Network request error")]
    RequestError(#[from] reqwest::Error),
    #[error("User input error")]
    InputError(#[from] inquire::error::InquireError),
    #[error("Error with Config")]
    ConfigError(#[from] canvas_cli_config::ConfigError),
    #[error("More options required for no config flag")]
    NeedMoreOptionsError,
}
