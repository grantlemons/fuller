use crate::cli::Cli;
use crate::selector::*;
use crate::Error;
use canvas_api::requests::*;
use canvas_cli_config::Config;
use canvas_cli_config::ConfigIgnore;
use reqwest::Client;
use tracing::info;
use tracing::warn;

pub async fn handle_list_courses(request_client: Client, config: &Config) -> anyhow::Result<()> {
    match prompt_selector(
        get_courses(request_client, config)
            .await?
            .into_iter()
            .filter(|c| !config.ignore.courses.contains(&(c.id as i64)))
            .collect(),
    )
    .await
    {
        Err(Error::Input(_)) => warn!("Error getting user input! Ignoring."),
        Ok(choice) => println!("{:#?}", choice),
        Err(e) => Err(e)?,
    }

    Ok(())
}

pub async fn ignore_courses(
    request_client: Client,
    cli: Cli,
    config: &Config,
) -> anyhow::Result<()> {
    match prompt_multiselector(
        get_courses(request_client, config)
            .await?
            .into_iter()
            .filter(|c| !config.ignore.courses.contains(&(c.id as i64)))
            .collect(),
    )
    .await
    {
        Err(Error::Input(_)) => warn!("Error getting user input! Ignoring."),
        Ok(choices) => {
            for choice in choices {
                info!("User ignored course {}", choice);
                canvas_cli_config::ignore_id(
                    cli.config.to_owned(),
                    ConfigIgnore::Course(choice.id as i64),
                )?;
            }
        }
        Err(e) => Err(e)?,
    }

    Ok(())
}

pub async fn handle_list_todo(request_client: Client, config: &Config) -> anyhow::Result<()> {
    match prompt_selector(get_todo(request_client, config).await?).await {
        Err(Error::Input(_)) => warn!("Error getting user input! Ignoring."),
        Ok(choice) => println!("{:#?}", choice),
        Err(e) => Err(e)?,
    }

    Ok(())
}

pub async fn handle_ignore_todo(request_client: Client, config: &Config) -> anyhow::Result<()> {
    match prompt_multiselector(get_todo(request_client.to_owned(), config).await?).await {
        Err(Error::Input(_)) => warn!("Error getting user input! Ignoring."),
        Ok(choices) => {
            for choice in choices {
                ignore_todo(request_client.to_owned(), config, &choice).await?;
            }
        }
        Err(e) => Err(e)?,
    }

    Ok(())
}

pub async fn handle_show_profile(request_client: Client, config: &Config) -> anyhow::Result<()> {
    let profile = get_self(request_client, config).await?;
    println!("{:#?}", profile);

    Ok(())
}
