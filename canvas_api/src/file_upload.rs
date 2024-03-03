use reqwest::{Client, Response, Result};
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Serialize)]
pub struct FileNotifyRequest {
    pub name: String,
    pub size: u64,
    pub content_type: Option<String>,
    pub parent_folder_id: Option<String>,
    pub parent_folder_path: Option<String>,
    pub on_duplicate: DuplicateBehavior,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DuplicateBehavior {
    Overwrite,
    Rename,
}

#[derive(Deserialize)]
pub struct FileNotifyResponse {
    pub upload_url: String,
    pub upload_params: serde_json::Map<String, serde_json::Value>,
}

async fn notify_submission<T: std::borrow::Borrow<canvas_cli_config::Config>>(
    client: Client,
    config: T,
    request: FileNotifyRequest,
    course_id: u64,
    assignment_id: u64,
) -> Result<FileNotifyResponse> {
    client
        .post(format!(
        "{}/api/v1/courses/{course_id}/assignments/{assignment_id}/submissions/comments/self/files",
        config.borrow().network.url
    ))
        .json(&request)
        .send()
        .await?
        .json()
        .await
}

async fn upload_as_instructed(
    client: Client,
    server_instructions: FileNotifyResponse,
    file: &mut std::fs::File,
) -> Result<Response> {
    let mut multipart: reqwest::multipart::Form = reqwest::multipart::Form::new();
    for param in server_instructions.upload_params {
        multipart = match param {
            (s, serde_json::Value::String(v)) => multipart.text(s, v),
            (s, serde_json::Value::Number(v)) => multipart.text(s, v.to_string()),
            (s, serde_json::Value::Bool(v)) => multipart.text(s, v.to_string()),
            _ => panic!("Unsupported value type"),
        }
    }

    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).expect("Unable to read file!");
    let file: reqwest::multipart::Part = reqwest::multipart::Part::bytes(bytes);
    multipart = multipart.part("file", file);

    client
        .post(server_instructions.upload_url)
        .multipart(multipart)
        .send()
        .await
}

pub async fn upload_to_assignment<T: std::borrow::Borrow<canvas_cli_config::Config>>(
    client: Client,
    config: T,
    path: std::path::PathBuf,
    course_id: u64,
    assignment_id: u64,
) -> Result<Response> {
    let mut file: std::fs::File = std::fs::File::open(path).unwrap();
    let metadata = file.metadata().expect("Unable to get file metadata!");

    let request = FileNotifyRequest {
        name: String::from(""),
        size: metadata.len(),
        content_type: None,
        parent_folder_id: None,
        parent_folder_path: None,
        on_duplicate: DuplicateBehavior::Rename,
    };

    let instructions = notify_submission(
        client.clone(),
        config.borrow(),
        request,
        course_id,
        assignment_id,
    )
    .await?;

    upload_as_instructed(client.clone(), instructions, &mut file).await
}
