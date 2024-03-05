use crate::error::Error;
use canvas_api::{requests::*, types::*};
use canvas_cli_config::Config;
use reqwest::Client;
use tracing::info;

pub async fn prompt_selector<T: std::fmt::Display + std::fmt::Debug>(
    options: Vec<T>,
) -> Result<T, crate::Error> {
    let selection = inquire::Select::new("", options)
        .with_vim_mode(true)
        .with_page_size(15)
        .prompt()?;
    info!("User made selection: {}", selection);

    Ok(selection)
}

pub async fn prompt_multiselector<T: std::fmt::Display + std::fmt::Debug>(
    options: Vec<T>,
) -> Result<Vec<T>, crate::Error> {
    let selection = inquire::MultiSelect::new("", options)
        .with_vim_mode(true)
        .with_page_size(15)
        .prompt()?;
    info!("User made selections: {:?}", selection);

    Ok(selection)
}

pub fn text_entry(message: &str) -> Result<String, inquire::InquireError> {
    inquire::Editor::new(message).prompt()
}

pub async fn select_course(
    request_client: Client,
    config: &Config,
    course_id: Option<u64>,
) -> Result<Course, Error> {
    if let Some(course_id) = course_id {
        Ok(get_course(request_client, config, course_id).await?)
    } else {
        prompt_selector(
            get_courses(request_client, config)
                .await?
                .into_iter()
                .filter(|c| !config.ignore.courses.contains(&(c.id as i64)))
                .collect(),
        )
        .await
    }
}

pub async fn select_courses(
    request_client: Client,
    config: &Config,
    course_ids: Option<Vec<u64>>,
) -> Result<Vec<Course>, Error> {
    let courses = get_courses(request_client, config)
        .await?
        .into_iter()
        .filter(|c| !config.ignore.courses.contains(&(c.id as i64)));

    if let Some(course_ids) = course_ids {
        Ok(courses.filter(|c| course_ids.contains(&c.id)).collect())
    } else {
        prompt_multiselector(courses.collect()).await
    }
}

pub async fn select_assignment(
    request_client: Client,
    config: &Config,
    course_id: Option<u64>,
    assignment_id: Option<u64>,
) -> Result<Assignment, Error> {
    let course_id = select_course(request_client.clone(), config, course_id)
        .await?
        .id;

    if let Some(assignment_id) = assignment_id {
        Ok(get_assignment(request_client, config, course_id, assignment_id).await?)
    } else {
        prompt_selector(
            list_course_assignments(request_client, config, course_id)
                .await?
                .into_iter()
                .filter(|a| !config.ignore.assignments.contains(&(a.id as i64)))
                .collect(),
        )
        .await
    }
}

pub async fn select_todo(request_client: Client, config: &Config) -> Result<Todo, Error> {
    prompt_selector(get_todo(request_client, config).await?).await
}
