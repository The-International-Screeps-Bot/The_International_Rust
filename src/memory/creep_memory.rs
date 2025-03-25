use screeps::RoomName;
use serde::{Deserialize, Serialize};

use crate::constants::creep::CreepRole;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreepMemory {
    pub role: CreepRole,
    pub room_from: RoomName,
    pub source_index: Option<usize>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerCreepMemory {}
