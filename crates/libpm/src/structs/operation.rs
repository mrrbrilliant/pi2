use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Operation {
    pub flags: Option<Vec<String>>,
    pub packages: Option<Vec<String>>,
}
