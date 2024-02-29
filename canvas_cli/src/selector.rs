use canvas_api::types::*;
use reqwest::Client;
use tracing::info;

pub(crate) async fn course_selector(client: Client) -> Result<Course, crate::error::Error> {
    let courses = canvas_api::requests::get_courses(client).await?;
    info!("Fetched course list, prompting user.");

    Ok(prompt_selector(courses).await?)
}

pub(crate) async fn todo_selector(client: Client) -> Result<Todo, crate::error::Error> {
    let todo = canvas_api::requests::get_todo(client).await?;
    info!("Fetched todo list, prompting user.");

    Ok(prompt_selector(todo).await?)
}

async fn prompt_selector<T: std::fmt::Display>(
    options: Vec<T>,
) -> inquire::error::InquireResult<T> {
    use inquire::formatter::OptionFormatter;
    use inquire::Select;

    let formatter: OptionFormatter<T> = &|o| format!("[{}] {}", o.index, o.value);
    let selection = Select::new("Select Course to View", options)
        .with_formatter(formatter)
        .with_vim_mode(true)
        .with_page_size(15)
        .prompt()?;
    info!("User made selection: {}", selection);

    Ok(selection)
}
