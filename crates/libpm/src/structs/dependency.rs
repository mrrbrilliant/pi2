use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Dependency {
    pub build: Option<HashMap<String, String>>,
    pub runtime: Option<HashMap<String, String>>,
    pub optional: Option<HashMap<String, String>>,
    pub test: Option<HashMap<String, String>>,
}
