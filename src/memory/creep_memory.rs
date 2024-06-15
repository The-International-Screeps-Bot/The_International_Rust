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
    pub rampart_only_shoving: Option<bool>,
}

impl CreepMemory {
    pub fn new(role: CreepRole, room_from: RoomName) -> Self {
        Self {
            role,
            room_from,
            source_index: None,
            scout_target: None,
            rampart_only_shoving: None,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct PowerCreepMemory {

}