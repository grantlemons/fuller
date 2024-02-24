use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder, Result,
};

const BASE_URL: &str = "https://elearning.mines.edu";

async fn get_generic<T: crate::types::ResponseType>(
    client: reqwest::Client,
    path: &str,
    query: Option<&[(&str, &str)]>,
) -> reqwest::Result<T> {
    let request = client.get(BASE_URL.to_owned() + path);
    let response = match query {
        Some(q) => request.query(q).send().await?,
        None => request.send().await?,
    };
    parse_result(response).await
}

async fn parse_result<T: crate::types::ResponseType>(
    response: reqwest::Response,
) -> reqwest::Result<T> {
    let body = response.text().await?;
    let untyped: serde_json::Value = serde_json::from_str(&body).unwrap();

    match serde_json::from_str(&body) {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("{:#?}", untyped);
            panic!("{:#?}", e);
        }
    }
    // response.json::<T>().await
}

pub fn create_client(auth_token: &str) -> Result<Client> {
    let mut auth_bearer: HeaderValue = ("Bearer ".to_owned() + auth_token).try_into().unwrap();
    auth_bearer.set_sensitive(true);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", auth_bearer);
    headers.insert("per_page", 50.into());

    ClientBuilder::new().default_headers(headers).build()
}

pub fn create_test_client(auth_token: &str) -> Result<Client> {
    let mut auth_bearer: HeaderValue = ("Bearer ".to_owned() + auth_token).try_into().unwrap();
    auth_bearer.set_sensitive(true);

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
