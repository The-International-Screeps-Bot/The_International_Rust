use std::collections::HashSet;

use log::LevelFilter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Settings {
    pub breaking_version: u32,
    pub compressed_memory: bool,
    pub allies: HashSet<String>,
    pub log_filter: LevelFilter,
}

impl Settings {
    pub fn new() -> Self {
        // These are default settings. Do not modify them for personal use

        let mut allies = HashSet::new();
        allies.insert("MarvinTMB".to_string());

        Settings {
            allies,
            compressed_memory: true,
            breaking_version: 4,
            log_filter: LevelFilter::Trace,
        }
    }
}
