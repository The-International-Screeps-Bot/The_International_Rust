use screeps::RoomName;
use serde::{Deserialize, Serialize};

use crate::constants::creep::CreepRole;

#[derive(Serialize, Deserialize, Clone)]
pub struct CreepMemory {
    #[serde(rename = "0")]
    pub role: CreepRole,
    pub room_from: RoomName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scout_target: Option<RoomName>,
}

impl CreepMemory {
    pub fn new(role: CreepRole, room_from: RoomName) -> Self {
        Self {
            role,
            room_from,
            source_index: None,
            scout_target: None,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct PowerCreepMemory {

}