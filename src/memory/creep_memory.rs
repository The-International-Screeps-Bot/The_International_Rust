use screeps::RoomName;
use serde::{Deserialize, Serialize};

use crate::constants::creep::CreepRole;

#[derive(Serialize, Deserialize, Clone)]
pub struct CreepMemory {
    #[serde(rename = "0")]
    pub role: CreepRole,
    pub source_index: Option<usize>,
    pub scout_target: Option<RoomName>,
}

impl Default for CreepMemory {
    fn default() -> Self {
        Self {
            role: CreepRole::Unknown,
            source_index: None,
            scout_target: None,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct PowerCreepMemory {

}