use screeps::{Position, RoomName};
use serde::{Deserialize, Serialize};

use crate::constants::creep::CreepRole;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreepMemory {
    pub role: CreepRole,
    pub room_from: RoomName,
    /// A position that the creep intends to stand on without unreasonable disruption
    pub harvest_pos: Option<Position>,
    pub source_index: Option<usize>,
    pub scout_target: Option<RoomName>,
    pub rampart_only_shoving: Option<bool>,
    pub move_goal_pos: Option<Position>,
    pub move_path: Option<Vec<Position>>,
}

impl CreepMemory {
    pub fn new(role: CreepRole, room_from: RoomName) -> Self {
        Self {
            role,
            room_from,
            harvest_pos: None,
            source_index: None,
            scout_target: None,
            rampart_only_shoving: None,
            move_goal_pos: None,
            move_path: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerCreepMemory {}
