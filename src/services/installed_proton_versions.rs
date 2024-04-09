use crate::traits::proton_versions::{FetchError, ProtonVersion, ProtonVersions};

pub struct InstalledProtonVersions {
    config_path: String,
}

impl ProtonVersions for InstalledProtonVersions {
    fn get_proton_versions(&self) -> Result<Vec<ProtonVersion>, FetchError> {
        unimplemented!()
    }
}
