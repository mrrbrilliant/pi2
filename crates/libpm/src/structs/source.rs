use crate::enums::SourceType;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Source {
    pub git: Option<HashMap<String, String>>,
    pub web: Option<HashMap<String, String>>,
    pub file: Option<HashMap<String, String>>,
}
