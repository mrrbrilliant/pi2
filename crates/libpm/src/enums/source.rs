use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct SourceType {
    pub version: Option<String>,
    pub path: Option<String>,
    pub extract: Option<bool>,
    pub save_as: Option<String>,
    pub git: Option<String>,
    pub web: Option<String>,
}
