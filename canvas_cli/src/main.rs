use anyhow::Context;
use clap::Parser;
use std::{fs::File, sync::Mutex};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod cli;
use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logging()?;
    let cli = Cli::parse();

    if let Some(_) = cli.config {
        todo!("Passing a config file is not yet supported!");
    }

    let _client =
        canvas_api::create_client(auth_token(&cli).await?).context("Creating Client Failed!")?;
    info!("Created request client!");

    Ok(())
}

async fn auth_token(cli: &Cli) -> anyhow::Result<canvas_auth::AccessToken> {
    if let Some(token) = &cli.token {
        let auth_token;
        auth_token = token.clone().into();
        info!("User provided a token: {:?}", auth_token);

        Ok(auth_token)
    } else {
        canvas_auth::connect()
            .await
            .context("Fetching Auth Token Failed!")
    }
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
