use screeps::RoomName;
use serde::{Deserialize, Serialize};

use crate::constants::creep::CreepRole;

#[derive(Serialize, Deserialize, Clone)]
pub struct CreepMemory {
    #[serde(rename = "0")]
    pub role: CreepRole,
    pub room_from: RoomName,
    pub source_index: Option<usize>,
    pub scout_target: Option<RoomName>,
}

impl CreepMemory {
    pub fn new(room_name: &RoomName) -> Self {
        Self {
            role: CreepRole::Unknown,
            room_from: room_name.clone(),
            source_index: None,
            scout_target: None,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct PowerCreepMemory {

}