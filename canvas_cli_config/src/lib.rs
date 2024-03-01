use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Path is not a file")]
    InvalidPath,
    #[error("Unable to open file")]
    FsError(#[from] std::io::Error),
    #[error("Unable to parse file")]
    ParseError(#[from] toml::de::Error),
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
}

#[derive(Deserialize, Clone, Debug)]
pub struct NetworkConfig {
    pub url: String,
    pub token: Option<AccessToken>,
    pub pagination: u16,
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
