use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Write};
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
    pub fn new(value: impl ToString) -> Self {
        Self(value.to_string())
    }

    pub fn secret(&self) -> &str {
        &self.0
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub formatting: FormattingConfig,
    pub network: NetworkConfig,
    pub ignore: IgnoreConfig,
    pub associations: AssociationsConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct FormattingConfig {
    pub date: String,
    pub time: String,
    pub max_width: usize,
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

#[derive(Deserialize, Clone, Debug)]
pub struct AssociationsConfig {
    pub submission_files: std::collections::BTreeMap<String, Vec<i64>>,
}

#[derive(Debug)]
pub enum ConfigIgnore {
    Course(i64),
    Inbox(i64),
    Discussion(i64),
    Grade(i64),
    Assignment(i64),
    Module(i64),
}

pub fn config_path(path: Option<PathBuf>) -> PathBuf {
    if let Some(path) = path {
        path
    } else {
        let mut path = dirs::config_dir().unwrap_or(PathBuf::from("."));
        path.push("fuller");
        path.push("config.toml");

        if !path.exists() {
            panic!(
                "Create config file at {:?} or specify one with a flag.",
                path
            )
        }

        path
    }
}

pub fn associate_submission_file(
    path: Option<PathBuf>,
    assignment_id: u64,
    file_id: u64,
) -> Result<Config, ConfigError> {
    use toml_edit::{Item, Value};

    let path = config_path(path);
    if !path.is_file() {
        tracing::error!(
            "Unable to find config file path in filesystem. {:?} is not a file!",
            path
        );
        return Err(ConfigError::InvalidPath);
    }

    let mut file = File::open(&path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let mut doc = toml_edit::Document::from_str(&file_contents)?;

    let table = doc["associations"]["submission_files"]
        .as_table_mut()
        .expect("associations.submission_files does not exist");

    let str_rep = assignment_id.to_string();
    match table.get_mut(&str_rep) {
        Some(Item::Value(Value::Array(v))) => v.push(file_id as i64),
        None => {
            let mut arr = toml_edit::Array::new();
            arr.push(file_id as i64);
            table.insert(&str_rep, Item::Value(Value::Array(arr)));
        }
        _ => {}
    };

    let bytes = doc.to_string();
    File::create(&path)?.write_all(bytes.as_bytes())?;
    Ok(toml_edit::de::from_document::<Config>(doc)?)
}

pub fn disassociate_submission_files(
    path: Option<PathBuf>,
    assignment_id: u64,
) -> Result<Config, ConfigError> {
    let path = config_path(path);
    if !path.is_file() {
        tracing::error!(
            "Unable to find config file path in filesystem. {:?} is not a file!",
            path
        );
        return Err(ConfigError::InvalidPath);
    }

    let mut file = File::open(&path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let mut doc = toml_edit::Document::from_str(&file_contents)?;

    doc["associations"]["submission_files"]
        .as_table_mut()
        .expect("associations.submission_files does not exist")
        .remove_entry(&assignment_id.to_string())
        .expect("Unable to remove assignment id from config associations table!");

    let bytes = doc.to_string();
    File::create(&path)?.write_all(bytes.as_bytes())?;
    Ok(toml_edit::de::from_document::<Config>(doc)?)
}

pub fn ignore_id(path: Option<PathBuf>, change: ConfigIgnore) -> Result<Config, ConfigError> {
    let path = config_path(path);
    if !path.is_file() {
        tracing::error!(
            "Unable to find config file path in filesystem. {:?} is not a file!",
            path
        );
        return Err(ConfigError::InvalidPath);
    }

    let mut file = File::open(&path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let mut doc = toml_edit::Document::from_str(&file_contents)?;

    match change {
        ConfigIgnore::Course(v) => doc["ignore"]["courses"]
            .as_array_mut()
            .expect("ignore.courses does not exist")
            .push(v),
        ConfigIgnore::Inbox(v) => doc["ignore"]["inbox"]
            .as_array_mut()
            .expect("ignore.inbox does not exist")
            .push(v),
        ConfigIgnore::Discussion(v) => doc["ignore"]["discussions"]
            .as_array_mut()
            .expect("ignore.discussions does not exist")
            .push(v),
        ConfigIgnore::Grade(v) => doc["ignore"]["grades"]
            .as_array_mut()
            .expect("ignore.grades does not exist")
            .push(v),
        ConfigIgnore::Assignment(v) => doc["ignore"]["assignments"]
            .as_array_mut()
            .expect("ignore.assignments does not exist")
            .push(v),
        ConfigIgnore::Module(v) => doc["ignore"]["modules"]
            .as_array_mut()
            .expect("ignore.modules does not exist")
            .push(v),
    };

    let bytes = doc.to_string();
    File::create(&path)?.write_all(bytes.as_bytes())?;
    Ok(toml_edit::de::from_document::<Config>(doc)?)
}

pub fn get_config(path: Option<PathBuf>) -> Result<Config, ConfigError> {
    let path = config_path(path);
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
