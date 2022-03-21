use crate::error;

use error::PiError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Pkg {
    // Metadata
    pub pkgname: String,
    pub version: String,
    pub release: u32,
    pub description: String,
    pub architectures: Option<Vec<String>>,
    pub licenses: Option<Vec<String>>,
    pub project_url: Option<Vec<String>>,
    // Resources
    pub sources: Option<Vec<String>>,
    pub provides: Option<Vec<String>>,
    pub conflict: Option<Vec<String>>,
    pub replace: Option<Vec<String>>,
    // Dependencies
    pub dependencies: Option<Vec<String>>,
    pub build_dependencies: Option<Vec<String>>,
    pub optional_dependencies: Option<Vec<String>>,
    pub builder: Option<String>,
    pub deploy: Option<String>,
    pub env_vars: Option<Vec<String>>,
}

impl Pkg {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self) {
        // pull source
        // extract source
        // build
    }
}
