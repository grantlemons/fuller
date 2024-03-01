use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Network request error")]
    Request(#[from] reqwest::Error),
    #[error("User input error")]
    Input(#[from] inquire::error::InquireError),
    #[error(" with Config")]
    Config(#[from] canvas_cli_config::ConfigError),
    #[error("More options required for no config flag")]
    NeedMoreOptions,
}
