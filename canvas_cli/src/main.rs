use anyhow::Context;
use clap::Parser;
use std::{fs::File, sync::Mutex};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod cli;
mod selector;
use cli::Cli;
use selector::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse before logging setup to prevent empty logfile generation for --help call
    let cli = Cli::parse();
    setup_logging()?;

    if let Some(_) = cli.config {
        todo!("Passing a config file is not yet supported!");
    }

    let client =
        canvas_api::create_client(auth_token(&cli).await?).context("Creating Client Failed!")?;
    info!("Created request client!");

    match &cli.command {
        cli::Commands::Courses { command: c } => {
            if let None = c {
                let class = course_selector(client).await?;
                println!("{:#?}", class);
            }
        }
        cli::Commands::Todo { command: c } => {
            if let None = c {
                let class = todo_selector(client).await?;
                println!("{:#?}", class);
            }
        }
        // cli::Commands::Inbox { command: c } => {}
        cli::Commands::Profile { command: c } => {
            if let None = c {
                let profile = canvas_api::requests::get_self(client).await?;
                println!("{:#?}", profile);
            }
        }
        _ => {}
    }

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
