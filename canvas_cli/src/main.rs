use anyhow::Context;
use std::{fs::File, sync::Mutex};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logging()?;

    let auth_token = canvas_auth::connect()
        .await
        .context("Fetching Auth Token Failed!")?;
    let _client = canvas_api::create_client(auth_token).context("Creating Client Failed!")?;

    Ok(())
}

fn setup_logging() -> anyhow::Result<()> {
    let log_file = File::create("most-recent.log")?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_writer(Mutex::new(log_file))
        .pretty()
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("Setting default subscriber failed")?;

    Ok(())
}
