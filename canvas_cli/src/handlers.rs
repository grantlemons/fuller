use crate::cli::Cli;
use crate::selector::*;
use crate::Error;
use anyhow::Result;
use canvas_api::requests::*;
use canvas_api::types::*;
use canvas_api::upload_to_assignment;
use canvas_cli_config::associate_submission_file;
use canvas_cli_config::Config;
use canvas_cli_config::ConfigIgnore;
use reqwest::Client;
use std::path::{Path, PathBuf};
use tracing::info;
use tracing::warn;

pub async fn select_course(request_client: Client, config: &Config) -> Result<Option<Course>> {
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
        Ok(choice) => return Ok(Some(choice)),
        Err(e) => Err(e)?,
    }

    Ok(None)
}

pub async fn ignore_courses(request_client: Client, cli: Cli, config: &Config) -> Result<()> {
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

pub async fn select_assignment(
    request_client: Client,
    config: &Config,
) -> Result<Option<Assignment>> {
    let course_id = match select_course(request_client.clone(), config).await? {
        Some(choice) => choice.id,
        None => return Ok(None),
    };
    match prompt_selector(
        list_course_assignments(request_client, config, course_id)
            .await?
            .into_iter()
            .filter(|c| !config.ignore.assignments.contains(&(c.id as i64)))
            .collect(),
    )
    .await
    {
        Err(Error::Input(_)) => warn!("Error getting user input! Ignoring."),
        Ok(choice) => return Ok(Some(choice)),
        Err(e) => Err(e)?,
    }

    Ok(None)
}

fn uploadable_filter(choice: &Assignment, path: &Path) -> bool {
    !choice.locked_for_user
        && choice
            .submission_types
            .contains(&AllowedSubmissionType::OnlineUpload)
        && if let (Some(extension), Some(allowed)) =
            (path.extension(), choice.allowed_extensions.clone())
        {
            let extension_string = extension
                .to_owned()
                .into_string()
                .expect("File extension is not valid Unicode!");
            allowed.contains(&extension_string)
        } else {
            true
        }
}

pub async fn handle_upload_file(
    cli: &Cli,
    request_client: Client,
    config: &Config,
    path: &PathBuf,
) -> Result<()> {
    let course_id = match select_course(request_client.clone(), config).await? {
        Some(choice) => choice.id,
        None => return Ok(()),
    };

    let choice: Assignment = match prompt_selector(
        list_course_assignments(request_client.clone(), config, course_id)
            .await?
            .into_iter()
            .filter(|c| !config.ignore.assignments.contains(&(c.id as i64)))
            .filter(|c| uploadable_filter(c, path))
            .collect(),
    )
    .await
    {
        Err(Error::Input(_)) => {
            warn!("Error getting user input! Ignoring.");
            return Ok(());
        }
        Ok(c) => c,
        Err(e) => Err(e)?,
    };

    let name = inquire::Text::new("Enter name for upload (include extension)").prompt()?;
    let res = upload_to_assignment(
        request_client,
        config,
        name,
        path,
        choice.course_id,
        choice.id,
    )
    .await?;

    info!("{:#?}", res);
    associate_submission_file(cli.config.to_owned(), choice.id, res.id)?;

    Ok(())
}

pub async fn select_todo(request_client: Client, config: &Config) -> Result<Option<Todo>> {
    match prompt_selector(get_todo(request_client, config).await?).await {
        Err(Error::Input(_)) => warn!("Error getting user input! Ignoring."),
        Ok(choice) => return Ok(Some(choice)),
        Err(e) => Err(e)?,
    }

    Ok(None)
}

pub async fn handle_ignore_todo(request_client: Client, config: &Config) -> Result<()> {
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

pub async fn handle_show_profile(request_client: Client, config: &Config) -> Result<()> {
    get_self(request_client, config).await?;

    Ok(())
}
