use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub struct Settings {
    #[serde(rename = "0")]
    pub breaking_version: u32,
    #[serde(rename = "1")]
    pub allies: HashSet<String>,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            allies: HashSet::new(),
            breaking_version: 0,
        }
    }
}
