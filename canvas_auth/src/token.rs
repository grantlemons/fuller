use dotenv::dotenv;
use oauth2::AccessToken;

pub async fn connect() -> Result<AccessToken, ()> {
    // let client = create_client().unwrap();
    // let (challenge, verifier) = generate_pkce();
    // let redirect_url = generate_redirect_url(&client, challenge);
    // println!("Access redirect here: {}", redirect_url);
    //
    // let response = get_token(&client, verifier).await;
    //
    // let access_token = response.access_token();
    // let expires_in = response.expires_in();
    // let refresh_token = response.refresh_token();

    dotenv().ok();
    let env_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap().to_owned();
    let access_token = AccessToken::new(env_token);

    Ok(access_token)
}
