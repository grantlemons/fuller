use anyhow::Context;
use canvas_cli_config::Config;
use clap::Parser;
use std::{fs::File, sync::Mutex};
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod cli;
mod error;
mod handlers;
mod selector;

use cli::*;
use error::Error;
use handlers::*;

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
        cli::Commands::Courses { command: None } => {
            let choice = select_course(request_client, &config).await?;
            println!("{:#?}", choice);
        }

        cli::Commands::Courses {
            command: Some(CoursesCommands::Ignore),
        } => ignore_courses(request_client, cli, &config).await?,

        cli::Commands::Courses {
            command: Some(CoursesCommands::Assignments),
        } => {
            let choice = select_assignment(request_client, &config).await?;
            println!("{:#?}", choice);
        }

        cli::Commands::Courses {
            command: Some(CoursesCommands::Upload { path }),
        } => handle_upload_file(&cli, request_client, &config, path).await?,

        cli::Commands::Todo { command: None } => {
            let choice = select_todo(request_client, &config).await?;
            println!("{:#?}", choice);
        }

        cli::Commands::Todo {
            command: Some(TodoCommands::Ignore),
        } => handle_ignore_todo(request_client, &config).await?,

        cli::Commands::Inbox { command: None } => todo!("Inbox not implemented yet!"),

        cli::Commands::Inbox {
            command: Some(InboxCommands::Ignore),
        } => todo!("Inbox not implemented yet!"),

        cli::Commands::Profile { command: None } => {
            handle_show_profile(request_client, &config).await?
        }

        _ => {}
    };

    warn!("Program complete, terminating!");
    Ok(())
}

fn create_config(cli: &Cli) -> Result<Config, Error> {
    let mut config = match (&cli.no_config, &cli.token, &cli.url, &cli.pagination) {
        // custom config without file if all cli options set
        // (true, Some(token), Some(url), Some(pagination)) => Config {
        //     network: NetworkConfig {
        //         token: Some(token.to_owned().into()),
        //         url: url.to_owned(),
        //         pagination: pagination.to_owned(),
        //     },
        // },
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
        .with_max_level(Level::INFO)
        .with_writer(Mutex::new(log_file))
        .pretty()
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("Setting default subscriber failed")?;

    Ok(())
}
