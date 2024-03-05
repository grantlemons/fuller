use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Error making network request")]
    RequestError(#[from] reqwest::Error),
}
