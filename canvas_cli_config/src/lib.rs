use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Path is not a file")]
    InvalidPath,
    #[error("Unable to open file")]
    Fs(#[from] std::io::Error),
    #[error("Unable to parse file")]
    Parse(#[from] toml::de::Error),
    #[error("Unable to parse file to edit")]
    EditParse(#[from] toml_edit::TomlError),
    #[error("Unable to parse edited document to config")]
    EditFailed(#[from] toml_edit::de::Error),
}

#[derive(serde::Serialize, Deserialize, Clone)]
pub struct AccessToken(String);

impl std::fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{{Access Token}}")
    }
}

impl From<String> for AccessToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AccessToken {
    pub fn new<T: ToString>(value: T) -> Self {
        Self(value.to_string())
    }

    pub fn secret(&self) -> &str {
        &self.0
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub network: NetworkConfig,
    pub ignore: IgnoreConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct NetworkConfig {
    pub url: String,
    pub token: Option<AccessToken>,
    pub pagination: u16,
}

#[derive(Deserialize, Clone, Debug)]
pub struct IgnoreConfig {
    pub courses: Vec<i64>,
    pub inbox: Vec<i64>,
    pub discussions: Vec<i64>,
    pub grades: Vec<i64>,
    pub assignments: Vec<i64>,
    pub modules: Vec<i64>,
}

#[derive(Debug)]
pub enum IgnoreCategory {
    Courses(i64),
    Inbox(i64),
    Discussions(i64),
    Grades(i64),
    Assignments(i64),
    Modules(i64),
}

pub fn ignore_id(path: Option<PathBuf>, change: IgnoreCategory) -> Result<Config, ConfigError> {
    let path = match path {
        Some(p) => p,
        None => PathBuf::from("./config.toml"),
    };
    if !path.is_file() {
        tracing::error!(
            "Unable to find config file path in filesystem. {:?} is not a file!",
            path
        );
        return Err(ConfigError::InvalidPath);
    }

    let mut file = File::open(path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let mut doc = toml_edit::Document::from_str(&file_contents)?;

    match change {
        IgnoreCategory::Courses(v) => doc["ignore"]["courses"]
            .as_array_mut()
            .expect("ignore.courses does not exist")
            .push(v),
        IgnoreCategory::Inbox(v) => doc["ignore"]["inbox"]
            .as_array_mut()
            .expect("ignore.inbox does not exist")
            .push(v),
        IgnoreCategory::Discussions(v) => doc["ignore"]["discussions"]
            .as_array_mut()
            .expect("ignore.discussions does not exist")
            .push(v),
        IgnoreCategory::Grades(v) => doc["ignore"]["grades"]
            .as_array_mut()
            .expect("ignore.grades does not exist")
            .push(v),
        IgnoreCategory::Assignments(v) => doc["ignore"]["assignments"]
            .as_array_mut()
            .expect("ignore.assignments does not exist")
            .push(v),
        IgnoreCategory::Modules(v) => doc["ignore"]["modules"]
            .as_array_mut()
            .expect("ignore.modules does not exist")
            .push(v),
    };

    Ok(toml_edit::de::from_document::<Config>(doc)?)
}

pub fn get_config(path: Option<PathBuf>) -> Result<Config, ConfigError> {
    let path = match path {
        Some(p) => p,
        None => PathBuf::from("./config.toml"),
    };
    if !path.is_file() {
        tracing::error!(
            "Unable to find config file path in filesystem. {:?} is not a file!",
            path
        );
        return Err(ConfigError::InvalidPath);
    }

    let mut file = File::open(path)?;
    extract_config(&mut file)
}

pub fn extract_config(file: &mut File) -> Result<Config, ConfigError> {
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    Ok(toml::from_str::<Config>(&file_contents)?)
}
