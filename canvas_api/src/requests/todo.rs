use crate::types::Todo;
use canvas_cli_config::Config;
use reqwest::{Client, Result};
use std::borrow::Borrow;

pub async fn get_todo(client: Client, config: impl Borrow<Config>) -> Result<Vec<Todo>> {
    super::get_generic(client, config.borrow(), "/api/v1/users/self/todo", None).await
}

pub async fn ignore_todo(client: Client, _: impl Borrow<Config>, todo: &Todo) -> Result<()> {
    client.delete(todo.ignore_url.as_str()).send().await?;

    Ok(())
}
