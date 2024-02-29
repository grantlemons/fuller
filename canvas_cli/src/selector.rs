use anyhow::Context;
use canvas_api::types::*;
use reqwest::Client;
use tracing::info;

pub(crate) async fn course_selector(client: Client) -> anyhow::Result<Course> {
    let courses = canvas_api::requests::get_courses(client)
        .await
        .context("Unable to fetch course list!")?;
    info!("Fetched course list, prompting user.");
    prompt_selector(courses).await
}

pub(crate) async fn todo_selector(client: Client) -> anyhow::Result<Todo> {
    let todo = canvas_api::requests::get_todo(client)
        .await
        .context("Unable to fetch todo list!")?;
    info!("Fetched todo list, prompting user.");
    prompt_selector(todo).await
}

async fn prompt_selector<T: std::fmt::Display>(options: Vec<T>) -> anyhow::Result<T> {
    use inquire::formatter::OptionFormatter;
    use inquire::Select;

    let formatter: OptionFormatter<T> = &|o| format!("[{}], {}", o.index, o.value);
    let selection = Select::new("Select Course to View", options)
        .with_formatter(formatter)
        .with_vim_mode(true)
        .with_page_size(15)
        .prompt()
        .unwrap();
    info!("User made selection: {}", selection);

    Ok(selection)
}
