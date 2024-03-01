use anyhow::Context;
use canvas_api::requests::ignore_todo;
use canvas_cli_config::{Config, NetworkConfig};
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

    let config = create_config(&cli)?;
    let auth_token = canvas_auth::connect(&config)
        .await
        .context("Fetching Auth Token Failed!")?;
    let request_client =
        canvas_api::create_client(auth_token, &config).context("Creating Client Failed!")?;
    info!("Created request client!");

    match &cli.command {
        // called courses command without any further subcommands
        cli::Commands::Courses { command: None } => {
            match course_selector(request_client, &config).await {
                Err(Error::Input(_)) => warn!("Error getting user input! Ignoring."),
                Ok(choice) => println!("{:#?}", choice),
                Err(e) => Err(e)?,
            }
        }
        // called todo command without any further subcommands
        cli::Commands::Todo { command: None } => match todo_selector(request_client, &config).await
        {
            Err(Error::Input(_)) => warn!("Error getting user input! Ignoring."),
            Ok(choice) => println!("{:#?}", choice),
            Err(e) => Err(e)?,
        },
        // called todo command with the ignore subcommand
        cli::Commands::Todo {
            command: Some(TodoCommands::Ignore),
        } => match todo_multiselector(request_client.clone(), &config).await {
            Err(Error::Input(_)) => warn!("Error getting user input! Ignoring."),
            Ok(choices) => {
                for choice in choices {
                    ignore_todo(request_client.clone(), &config, &choice).await?;
                }
            }
            Err(e) => Err(e)?,
        },
        // called inbox command without any further subcommands
        cli::Commands::Inbox { command: None } => {
            todo!("Inbox not implemented yet!");
        }
        // called profile command without any further subcommands
        cli::Commands::Profile { command: None } => {
            let profile = canvas_api::requests::get_self(request_client, &config).await?;
            println!("{:#?}", profile);
        }
        _ => {}
    }

    warn!("Program complete, terminating!");
    Ok(())
}

fn create_config(cli: &Cli) -> Result<Config, Error> {
    let mut config = match (&cli.no_config, &cli.token, &cli.url, &cli.pagination) {
        // custom config without file if all cli options set
        (true, Some(token), Some(url), Some(pagination)) => Config {
            network: NetworkConfig {
                token: Some(token.to_owned().into()),
                url: url.to_owned(),
                pagination: pagination.to_owned(),
            },
        },
        // otherwise error on no config flag
        (true, _, _, _) => Err(Error::NeedMoreOptions)?,
        // otherwise use config file
        _ => canvas_cli_config::get_config(cli.config.to_owned())?,
    };

    // overwrite config file individually
    if let Some(token) = &cli.token {
        info!("User provided a token: {:?}", token);
        config.network.token = Some(token.to_owned().into());
    }
    if let Some(url) = &cli.url {
        config.network.url = url.to_owned();
    }
    if let Some(pagination) = &cli.pagination {
        config.network.pagination = pagination.to_owned();
    }

    Ok(config)
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
