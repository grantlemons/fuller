use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Network request error")]
    RequestError(#[from] reqwest::Error),

    #[error("User input error")]
    InputError(#[from] inquire::error::InquireError),
}
