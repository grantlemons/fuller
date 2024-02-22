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
    response.json::<T>().await
}

pub fn create_client(auth_token: &str) -> Result<Client> {
    let mut auth_bearer: HeaderValue = ("Bearer ".to_owned() + auth_token).try_into().unwrap();
    auth_bearer.set_sensitive(true);

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", auth_bearer);

    ClientBuilder::new().default_headers(headers).build()
}

mod assignment;
mod course;
mod todo;
mod user;

pub use assignment::*;
pub use course::*;
pub use todo::*;
pub use user::*;
