use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let auth_token = canvas_auth::connect().await.unwrap();
    let _client =
        canvas_api::create_client(auth_token.secret()).context("Creating Client Failed!")?;

    Ok(())
}
