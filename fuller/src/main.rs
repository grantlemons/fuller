use fuller_canvas_api::Client;
use fuller_canvas_api::Viewable;
use fuller_cli_options::*;
use fuller_config::Config;
use selector::*;
use std::{fs::File, sync::Mutex};
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod error;
mod handlers;
mod selector;

pub use error::Error;
use handlers::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse before logging setup to prevent empty logfile generation for --help call
    let cli = Cli::parse();
    // setup_logging();

    let config = create_config(&cli)?;
    let auth_token = fuller_canvas_auth::connect(&config).await?;
    let request_client = fuller_canvas_api::create_client(auth_token, &config)?;
    info!("Created request client!");

    match run_handlers(cli, request_client, &config).await {
        Ok(_) => {}
        Err(Error::Input(_)) => {}
        Err(e) => Err(e)?,
    }

    warn!("Program complete, terminating!");
    Ok(())
}

async fn run_handlers(cli: Cli, request_client: Client, config: &Config) -> Result<(), Error> {
    match cli.command.to_owned() {
        Commands::Courses { course_id, command } => match command {
            None => println!(
                "{}",
                select_course(request_client, config, course_id)
                    .await?
                    .view(config)
            ),
            Some(CoursesCommands::Assignments { assignment_id }) => println!(
                "{}",
                select_assignment(request_client, config, course_id, assignment_id)
                    .await?
                    .view(config)
            ),
            Some(CoursesCommands::Ignore { course_ids }) => {
                ignore_courses(&cli, request_client, config, course_ids).await?
            }
            Some(CoursesCommands::Upload {
                assignment_id,
                path,
            }) => {
                handle_upload_file(
                    &cli,
                    request_client,
                    config,
                    &path,
                    course_id,
                    assignment_id,
                )
                .await?
            }
            Some(CoursesCommands::Submit { assignment_id }) => {
                handle_submit(&cli, request_client, config, course_id, assignment_id).await?
            }
        },

        Commands::Todo { command } => match command {
            None => println!(
                "{}",
                select_todo(request_client, config).await?.view(config)
            ),
            Some(TodoCommands::Ignore) => handle_ignore_todo(request_client, config).await?,
        },

        Commands::Inbox { inbox_id, command } => match command {
            None => println!(
                "{:?}",
                select_conversation(request_client, config, inbox_id).await?
            ),
            Some(_) => todo!("Inbox is not yet supported!"),
        },

        Commands::Profile { command } => match command {
            None => println!(
                "{}",
                fuller_canvas_api::requests::get_self(request_client, config)
                    .await?
                    .view(config)
            ),
            Some(_) => todo!("Searching other users is not yet supported!"),
        },
    };

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
        _ => fuller_config::get_config(cli.config.to_owned())?,
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

    if config.network.url.is_empty() {
        return Err(Error::InvalidConfig("URL not configured!"));
    }

    Ok(config)
}

#[allow(unused)]
fn setup_logging() {
    let log_file = File::create("most-recent.log").unwrap();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_writer(Mutex::new(log_file))
        .pretty()
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
