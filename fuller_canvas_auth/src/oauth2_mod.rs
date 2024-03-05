#![allow(unused)]

use crate::AuthError;
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::async_http_client;
use oauth2::url::{ParseError, Url};
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    EmptyExtraTokenFields, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    StandardTokenResponse, TokenResponse, TokenUrl,
};
use tracing::info;

fn create_client() -> Result<BasicClient, ParseError> {
    let client_id = ClientId::new("".to_owned());
    let client_secret = ClientSecret::new("".to_owned());
    let auth_url = AuthUrl::new("".to_owned())?;
    let token_url = TokenUrl::new("".to_owned())?;
    let redirect_url = RedirectUrl::new("".to_owned())?;

    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url);

    Ok(client)
}

fn generate_pkce() -> (PkceCodeChallenge, PkceCodeVerifier) {
    PkceCodeChallenge::new_random_sha256()
}

fn generate_redirect_url(client: &BasicClient, challenge: PkceCodeChallenge) -> Url {
    let scopes = [
        Scope::new("read".to_owned()),
        Scope::new("write".to_owned()),
    ];

    let (auth_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(scopes)
        .set_pkce_challenge(challenge)
        .url();

    auth_url
}

async fn get_token(
    client: &BasicClient,
    verifier: PkceCodeVerifier,
) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, AuthError> {
    let auth_code = AuthorizationCode::new("".to_owned());

    let res = client
        .exchange_code(auth_code)
        .set_pkce_verifier(verifier)
        .request_async(async_http_client)
        .await;
    match res {
        Ok(res) => Ok(res),
        Err(_) => Err(AuthError::OAuthTransaction),
    }
}

#[tracing::instrument]
pub async fn connect(_: &fuller_config::Config) -> Result<crate::AccessToken, AuthError> {
    let client = create_client()?;
    let (challenge, verifier) = generate_pkce();
    let redirect_url = generate_redirect_url(&client, challenge);
    info!("Issuing redirect url: {}", redirect_url);
    println!("Access redirect here: {}", redirect_url);

    let response = get_token(&client, verifier).await?;

    let access_token = response.access_token();
    let expires_in = response.expires_in();
    let refresh_token = response.refresh_token();

    info!("OAuth2 Auth Proccess Complete!");
    Ok(access_token.secret().to_owned().into())
}
