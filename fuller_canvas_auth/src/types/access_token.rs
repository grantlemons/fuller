#[derive(serde::Serialize, serde::Deserialize, Clone)]
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

impl From<fuller_config::AccessToken> for AccessToken {
    fn from(value: fuller_config::AccessToken) -> Self {
        Self(value.secret().to_owned())
    }
}

impl From<AccessToken> for fuller_config::AccessToken {
    fn from(value: AccessToken) -> Self {
        Self::new(value.secret().to_owned())
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
