use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Failed to fetch proton versions: {0}")]
    Fetch(String),
}

pub struct ProtonVersion {
    pub version: String,
    pub path: String,
    pub display_name: String,
}

pub trait ProtonVersions {
    fn get_proton_versions(&self) -> Result<Vec<ProtonVersion>, FetchError>;
}
