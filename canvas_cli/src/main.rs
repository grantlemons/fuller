use anyhow::Context;
use clap::Parser;
use std::{fs::File, sync::Mutex};
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod cli;
mod error;
mod selector;
use cli::Cli;
use error::Error;
use selector::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse before logging setup to prevent empty logfile generation for --help call
    let cli = Cli::parse();
    setup_logging()?;

    if cli.config.is_some() {
        todo!("Passing a config file is not yet supported!");
    }

    let client =
        canvas_api::create_client(auth_token(&cli).await?).context("Creating Client Failed!")?;
    info!("Created request client!");

    match &cli.command {
        cli::Commands::Courses { command: None } => match course_selector(client).await {
            Err(Error::InputError(_)) => warn!("Error getting user input! Ignoring."),
            choice => println!("{:#?}", choice?),
        },
        cli::Commands::Todo { command: None } => match todo_selector(client).await {
            Err(Error::InputError(_)) => warn!("Error getting user input! Ignoring."),
            choice => println!("{:#?}", choice?),
        },
        cli::Commands::Inbox { command: None } => {
            todo!("Inbox implemented yet!");
        }
        cli::Commands::Profile { command: None } => {
            let profile = canvas_api::requests::get_self(client).await?;
            println!("{:#?}", profile);
        }
        _ => {}
    }

    Ok(())
}

async fn auth_token(cli: &Cli) -> anyhow::Result<canvas_auth::AccessToken> {
    if let Some(token) = &cli.token {
        let auth_token = token.clone().into();
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
