use crate::types::Todo;
use reqwest::{Client, Result};

pub async fn get_todo<T: std::borrow::Borrow<canvas_cli_config::Config>>(
    client: Client,
    config: T,
) -> Result<Vec<Todo>> {
    super::get_generic(client, config.borrow(), "/api/v1/users/self/todo", None).await
}

pub async fn ignore_todo<T: std::borrow::Borrow<canvas_cli_config::Config>>(
    client: Client,
    _: T,
    todo: &Todo,
) -> Result<bool> {
    client
        .delete(todo.ignore_url.as_str())
        .send()
        .await
        .map(|res| res.status().is_success())
}
