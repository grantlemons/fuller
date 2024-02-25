#[derive(serde::Serialize)]
pub struct AccessToken(String);

impl std::fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{{Access Token}}")
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
