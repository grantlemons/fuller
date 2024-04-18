use crate::cli::Cli;
use crate::error::Error;
use crate::selector::*;
use fuller_canvas_api::requests::*;
use fuller_canvas_api::types::*;
use fuller_canvas_api::upload_to_assignment;
use fuller_canvas_api::Client;
use fuller_config::associate_submission_file;
use fuller_config::disassociate_submission_files;
use fuller_config::Config;
use fuller_config::ConfigIgnore;
use std::path::{Path, PathBuf};
use tracing::info;

pub async fn ignore_courses(
    cli: &Cli,
    request_client: Client,
    config: &Config,
    course_ids: Option<Vec<u64>>,
) -> Result<(), Error> {
    let course_ids = if let Some(course_ids) = course_ids {
        course_ids
    } else {
        select_courses(request_client, config, None)
            .await?
            .into_iter()
            .map(|c| c.id)
            .collect()
    };

    for course_id in course_ids {
        info!("User ignored course {}", course_id);
        fuller_config::ignore_id(
            cli.config.to_owned(),
            ConfigIgnore::Course(course_id as i64),
        )?;
    }

    Ok(())
}

fn uploadable_filter(
    choice: &Assignment,
    submission_type: AllowedSubmissionType,
    path: Option<&Path>,
) -> bool {
    !choice.locked_for_user
        && choice.submission_types.contains(&submission_type)
        && if let Some(path) = path {
            if let (Some(extension), Some(allowed)) =
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
        } else {
            true
        }
}

pub async fn handle_submit(
    cli: &Cli,
    request_client: Client,
    config: &Config,
    course_id: Option<u64>,
    assignment_id: Option<u64>,
) -> Result<(), Error> {
    let course_id = if let Some(course_id) = course_id {
        course_id
    } else {
        select_course(request_client.clone(), config, None)
            .await?
            .id
    };

    let assignment = if let Some(assignment_id) = assignment_id {
        get_assignment(request_client.clone(), config, course_id, assignment_id).await?
    } else {
        prompt_selector(
            list_course_assignments(request_client.clone(), config, course_id)
                .await?
                .into_iter()
                .filter(|c| !config.ignore.assignments.contains(&(c.id as i64)))
                .filter(|c| {
                    uploadable_filter(c, AllowedSubmissionType::OnlineTextEntry, None)
                        || uploadable_filter(c, AllowedSubmissionType::OnlineUrl, None)
                        || (uploadable_filter(c, AllowedSubmissionType::OnlineUpload, None)
                            && config
                                .associations
                                .submission_files
                                .contains_key(&c.id.to_string()))
                })
                .collect(),
        )
        .await?
    };

    let submission_type = prompt_selector(
        assignment
            .submission_types
            .into_iter()
            .filter(|c| {
                c == &AllowedSubmissionType::OnlineTextEntry
                    || c == &AllowedSubmissionType::OnlineUrl
                    || c == &AllowedSubmissionType::OnlineUpload
            })
            .collect(),
    )
    .await?;

    let submission_type = match submission_type {
        AllowedSubmissionType::OnlineTextEntry => {
            SubmissionRequestType::OnlineTextEntry(text_entry("")?)
        }
        AllowedSubmissionType::OnlineUrl => {
            SubmissionRequestType::OnlineUrl(inquire::Text::new("Enter URL").prompt()?)
        }
        AllowedSubmissionType::OnlineUpload => {
            let res = SubmissionRequestType::OnlineUpload(
                config
                    .associations
                    .submission_files
                    .get(&assignment.id.to_string())
                    .expect("Expected config to contain key. Has config changed?")
                    .iter()
                    .copied()
                    .map(|v| v as u64)
                    .collect(),
            );

            info!("Disassociating submission files!");
            disassociate_submission_files(cli.config.to_owned(), assignment.id)?;

            res
        }
        _ => panic!("Unexpected value matched"),
    };

    let comment = inquire::Text::new("Comment?")
        .prompt_skippable()
        .unwrap_or_default();
    info!("Comment: {:?}", comment);
    let submission_request = SubmissionRequest::new(submission_type, comment);

    submit_assignment(
        request_client,
        config,
        assignment.course_id,
        assignment.id,
        submission_request,
    )
    .await?;

    Ok(())
}

pub async fn handle_upload_file(
    cli: &Cli,
    request_client: Client,
    config: &Config,
    path: &PathBuf,
    course_id: Option<u64>,
    assignment_id: Option<u64>,
) -> Result<(), Error> {
    let course_id = if let Some(course_id) = course_id {
        course_id
    } else {
        select_course(request_client.clone(), config, None)
            .await?
            .id
    };

    let assignment = if let Some(assignment_id) = assignment_id {
        get_assignment(request_client.clone(), config, course_id, assignment_id).await?
    } else {
        prompt_selector(
            list_course_assignments(request_client.clone(), config, course_id)
                .await?
                .into_iter()
                .filter(|c| !config.ignore.assignments.contains(&(c.id as i64)))
                .filter(|c| uploadable_filter(c, AllowedSubmissionType::OnlineUpload, Some(path)))
                .collect(),
        )
        .await?
    };

    let name = inquire::Text::new("Enter name for upload (include extension)").prompt()?;
    let res = upload_to_assignment(
        request_client,
        config,
        name,
        path,
        assignment.course_id,
        assignment.id,
    )
    .await?;

    info!("{:#?}", res);
    associate_submission_file(cli.config.to_owned(), assignment.id, res.id)?;

    Ok(())
}

pub async fn handle_ignore_todo(request_client: Client, config: &Config) -> Result<(), Error> {
    let choices = prompt_multiselector(get_todo(request_client.to_owned(), config).await?).await?;

    for choice in choices {
        ignore_todo(request_client.to_owned(), config, &choice).await?;
    }

    Ok(())
}
