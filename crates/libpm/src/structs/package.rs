use crate::enums::{Architecture, License};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub release: usize,
    pub description: Option<String>,
    pub architecture: Vec<Architecture>,
    pub license: Vec<License>,
    pub project_url: Option<String>,
    pub project_owner: Option<String>,
    pub maintainer: Option<Vec<String>>,
}
