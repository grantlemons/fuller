use crate::types::Todo;
use reqwest::{Client, Result};

pub async fn get_todo(client: Client) -> Result<Vec<Todo>> {
    super::get_generic(client, "/api/v1/users/self/todo", None).await
}

pub async fn ignore_todo(client: Client, todo: &Todo) -> Result<bool> {
    client
        .delete(todo.ignore_url.as_str())
        .send()
        .await
        .map(|res| res.status().is_success())
}
