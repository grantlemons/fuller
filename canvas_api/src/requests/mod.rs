use canvas_auth::AccessToken;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder, Result,
};
use tracing::{info, instrument};

#[instrument]
async fn get_generic<T: crate::types::ResponseType>(
    client: reqwest::Client,
    path: &str,
    query: Option<&[(&str, &str)]>,
) -> reqwest::Result<T> {
    let address = std::env::var("CANVAS_URL").unwrap() + path;
    info!("Request address is {address}");

    info!("Making request to server...");
    let request = client.get(address);
    let response = match query {
        Some(q) => request.query(q).send().await?,
        None => request.send().await?,
    };

    info!("Parsing response!");
    parse_result(response).await
}

#[instrument]
async fn parse_result<T: crate::types::ResponseType>(
    response: reqwest::Response,
) -> reqwest::Result<T> {
    info!("Getting body from response...");
    let body = response.text().await?;
    let untyped: serde_json::Value = serde_json::from_str(&body).unwrap();
    info!("Parsed into untyped JSON");

    info!("Attempting to parse JSON into structured data type...");
    match serde_json::from_str(&body) {
        Ok(v) => Ok(v),
        Err(e) => {
            tracing::error!("Unable to parse JSON into structured data type! Panicking!");
            tracing::error!("{:#?}", untyped);
            panic!("{:#?}", e);
        }
    }
}

#[instrument]
pub fn create_client(auth_token: AccessToken) -> Result<Client> {
    let pagination = 50;

    info!("Building application reqwest client...");
    info!("Default pagination set to {pagination}");
    info!("Setting auth header...");
    let mut auth_bearer: HeaderValue = ("Bearer ".to_owned() + auth_token.secret())
        .try_into()
        .unwrap();
    auth_bearer.set_sensitive(true);
    info!("Auth header set!");

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", auth_bearer);
    headers.insert("per_page", pagination.into());

    ClientBuilder::new().default_headers(headers).build()
}

pub fn create_test_client(auth_token: AccessToken) -> Result<Client> {
    let pagination = 50;

    info!("Building test reqwest client...");
    info!("Default pagination set to {pagination}");
    info!("Setting auth header...");
    let mut auth_bearer: HeaderValue = ("Bearer ".to_owned() + auth_token.secret())
        .try_into()
        .unwrap();
    auth_bearer.set_sensitive(true);
    info!("Auth header set!");

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", auth_bearer);
    headers.insert("per_page", 100.into());

    ClientBuilder::new().default_headers(headers).build()
}

pub mod assignment;
pub mod course;
pub mod discussion;
pub mod module;
pub mod todo;
pub mod user;

pub use assignment::*;
pub use course::*;
pub use discussion::*;
pub use module::*;
pub use todo::*;
pub use user::*;
