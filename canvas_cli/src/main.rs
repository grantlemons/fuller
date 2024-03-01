use anyhow::Context;
use canvas_api::requests::ignore_todo;
use canvas_cli_config::Config;
use clap::Parser;
use std::{fs::File, sync::Mutex};
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod cli;
mod error;
mod selector;
use cli::*;
use error::Error;
use selector::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse before logging setup to prevent empty logfile generation for --help call
    let cli = Cli::parse();
    setup_logging()?;
    let config = canvas_cli_config::get_config(cli.config.to_owned())?;

    let client = canvas_api::create_client(auth_token(&cli, &config).await?, &config)
        .context("Creating Client Failed!")?;
    info!("Created request client!");

    match &cli.command {
        cli::Commands::Courses { command: None } => match course_selector(client, &config).await {
            Err(Error::InputError(_)) => warn!("Error getting user input! Ignoring."),
            Ok(choice) => println!("{:#?}", choice),
            Err(e) => Err(e)?,
        },
        cli::Commands::Todo { command: None } => match todo_selector(client, &config).await {
            Err(Error::InputError(_)) => warn!("Error getting user input! Ignoring."),
            Ok(choice) => println!("{:#?}", choice),
            Err(e) => Err(e)?,
        },
        cli::Commands::Todo {
            command: Some(TodoCommands::Ignore),
        } => match todo_multiselector(client.clone(), &config).await {
            Err(Error::InputError(_)) => warn!("Error getting user input! Ignoring."),
            Ok(choices) => {
                for choice in choices {
                    ignore_todo(client.clone(), &config, &choice).await?;
                }
            }
            Err(e) => Err(e)?,
        },
        cli::Commands::Inbox { command: None } => {
            todo!("Inbox not implemented yet!");
        }
        cli::Commands::Profile { command: None } => {
            let profile = canvas_api::requests::get_self(client, &config).await?;
            println!("{:#?}", profile);
        }
        _ => {}
    }

    warn!("Program complete, terminating!");
    Ok(())
}

async fn auth_token(cli: &Cli, config: &Config) -> anyhow::Result<canvas_auth::AccessToken> {
    if let Some(token) = &cli.token {
        let auth_token = token.clone();
        info!("User provided a token: {:?}", auth_token);

        Ok(auth_token)
    } else {
        canvas_auth::connect(config)
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
