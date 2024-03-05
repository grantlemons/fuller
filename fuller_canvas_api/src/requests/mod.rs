use fuller_canvas_auth::AccessToken;
use fuller_config::Config;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder, Result,
};
use tracing::{info, instrument};

#[instrument]
async fn get_generic<T: crate::types::ResponseType>(
    client: reqwest::Client,
    config: &Config,
    path: &str,
    query: Option<&[(&str, &str)]>,
) -> reqwest::Result<T> {
    let address = config.network.url.to_owned() + path;
    info!("Request address is {address}");

    info!("Making request to server...");
    let request = client.get(address);

    let mut query = query.unwrap_or_default().to_vec();
    let pagination = config.network.pagination.to_string();
    query.push(("per_page", &pagination));

    let response = request.query(&query).send().await?;

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
pub fn create_client(auth_token: AccessToken, config: &Config) -> Result<Client> {
    info!("Building application reqwest client...");
    info!("Default pagination set to {}", config.network.pagination);
    info!("Setting auth header...");
    let mut auth_bearer: HeaderValue = ("Bearer ".to_owned() + auth_token.secret())
        .try_into()
        .unwrap();
    auth_bearer.set_sensitive(true);
    info!("Auth header set!");

    let mut headers = HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, auth_bearer);
    headers.insert("per_page", config.network.pagination.into());

    ClientBuilder::new().default_headers(headers).build()
}

pub mod assignment;
pub mod conversation;
pub mod course;
pub mod discussion;
pub mod module;
pub mod todo;
pub mod user;

pub use assignment::*;
pub use conversation::*;
pub use course::*;
pub use discussion::*;
pub use module::*;
pub use todo::*;
pub use user::*;
