use chrono::{DateTime, Utc};
use reqwest::{Client, Result};
use serde::{Deserialize, Serialize};
use std::io::Read;
use tracing::info;

#[derive(Debug, Serialize)]
pub struct FileNotifyRequest {
    pub name: String,
    pub size: u64,
    pub content_type: Option<String>,
    pub parent_folder_id: Option<String>,
    pub parent_folder_path: Option<String>,
    pub on_duplicate: DuplicateBehavior,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DuplicateBehavior {
    Overwrite,
    Rename,
}

#[derive(Debug, Deserialize)]
pub struct FileNotifyResponse {
    pub upload_url: String,
    pub upload_params: serde_json::Map<String, serde_json::Value>,
}

async fn notify_submission<T: std::borrow::Borrow<fuller_config::Config>>(
    client: Client,
    config: T,
    request: FileNotifyRequest,
    course_id: u64,
    assignment_id: u64,
) -> Result<FileNotifyResponse> {
    let url = format!(
        "{}/api/v1/courses/{course_id}/assignments/{assignment_id}/submissions/self/files",
        config.borrow().network.url
    );

    info!("Requesting instructions!");
    info!("Url: {url}");
    client.post(url).form(&request).send().await?.json().await
}

async fn upload_as_instructed(
    client: Client,
    server_instructions: FileNotifyResponse,
    file: &mut std::fs::File,
) -> Result<FileUploadResponse> {
    let mut multipart: reqwest::multipart::Form = reqwest::multipart::Form::new();
    for param in server_instructions.upload_params {
        info!("Server upload param: ({}, {})", param.0, param.1);
        multipart = match param {
            (s, serde_json::Value::String(v)) => multipart.text(s, v),
            (s, serde_json::Value::Number(v)) => multipart.text(s, v.to_string()),
            (s, serde_json::Value::Bool(v)) => multipart.text(s, v.to_string()),
            _ => panic!("Unsupported value type"),
        }
    }

    info!("Reading submitted file to bytes!");
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).expect("Unable to read file!");
    let file: reqwest::multipart::Part = reqwest::multipart::Part::bytes(bytes);
    multipart = multipart.part("file", file);

    info!("Uploading file as instructed!");
    client
        .post(server_instructions.upload_url)
        .multipart(multipart)
        .header(reqwest::header::AUTHORIZATION, "")
        .send()
        .await?
        .json()
        .await
}

pub async fn upload_to_assignment<T: std::borrow::Borrow<fuller_config::Config>>(
    client: Client,
    config: T,
    name: String,
    path: &std::path::PathBuf,
    course_id: u64,
    assignment_id: u64,
) -> Result<FileUploadResponse> {
    info!("Opening provided file for upload!");
    let mut file: std::fs::File = std::fs::File::open(path).unwrap();
    let metadata = file.metadata().expect("Unable to get file metadata!");

    let request = FileNotifyRequest {
        name,
        size: metadata.len(),
        content_type: None,
        parent_folder_id: None,
        parent_folder_path: None,
        on_duplicate: DuplicateBehavior::Rename,
    };
    info!("Request constructed: {:#?}", request);

    info!("Attempting to get instructions from server!");
    let instructions = notify_submission(
        client.clone(),
        config.borrow(),
        request,
        course_id,
        assignment_id,
    )
    .await?;

    info!("Attempting to upload file as instructed!");
    let res = upload_as_instructed(client.clone(), instructions, &mut file).await?;
    client.get(&res.location).send().await?; // get location to verify per docs

    Ok(res)
}

#[derive(Debug, Deserialize)]
pub struct FileUploadResponse {
    pub location: String,
    pub id: u64,
    pub uuid: String,
    pub folder_id: u64,
    pub display_name: String,
    pub filename: String,
    pub upload_status: UploadStatus,
    #[serde(alias = "content-type")]
    pub content_type: String,
    pub url: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub locked: bool,
    pub hidden: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UploadStatus {
    Success,
    // ...
}
