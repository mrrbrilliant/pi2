use std::collections::HashMap;

pub struct Config {
    pub repos: Option<HashMap<String, Vec<String>>>,
}
