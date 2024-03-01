use canvas_api::types::*;
use canvas_cli_config::Config;
use reqwest::Client;
use tracing::info;

pub(crate) async fn course_selector(
    client: Client,
    config: &Config,
) -> Result<Course, crate::error::Error> {
    let courses = canvas_api::requests::get_courses(client, config).await?;
    info!("Fetched course list, prompting user.");

    Ok(prompt_selector(courses).await?)
}

pub(crate) async fn todo_selector(
    client: Client,
    config: &Config,
) -> Result<Todo, crate::error::Error> {
    let todo = canvas_api::requests::get_todo(client, config).await?;
    info!("Fetched todo list, prompting user.");

    Ok(prompt_selector(todo).await?)
}

pub(crate) async fn todo_multiselector(
    client: Client,
    config: &Config,
) -> Result<Vec<Todo>, crate::error::Error> {
    let todo = canvas_api::requests::get_todo(client, config).await?;
    info!("Fetched todo list, prompting user.");

    Ok(prompt_multiselector(todo).await?)
}

async fn prompt_selector<T: std::fmt::Display + std::fmt::Debug>(
    options: Vec<T>,
) -> inquire::error::InquireResult<T> {
    use inquire::Select;

    let selection = Select::new("Select to View", options)
        .with_vim_mode(true)
        .with_page_size(15)
        .prompt()?;
    info!("User made selection: {}", selection);

    Ok(selection)
}

async fn prompt_multiselector<T: std::fmt::Display + std::fmt::Debug>(
    options: Vec<T>,
) -> inquire::error::InquireResult<Vec<T>> {
    use inquire::MultiSelect;

    let selection = MultiSelect::new("Select to Ignore", options)
        .with_vim_mode(true)
        .with_page_size(15)
        .prompt()?;
    info!("User made selection: {:?}", selection);

    Ok(selection)
}
