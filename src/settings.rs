use std::collections::HashSet;

use log::LevelFilter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    #[serde(rename = "0")]
    pub breaking_version: u32,
    #[serde(rename = "1")]
    pub allies: HashSet<String>,
    pub log_filter: LevelFilter,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            allies: HashSet::new(),
            breaking_version: 0,
            log_filter: LevelFilter::Trace,
        }
    }
}
