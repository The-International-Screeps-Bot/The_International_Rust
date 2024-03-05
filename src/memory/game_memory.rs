use std::collections::HashMap;

use screeps::RoomName;
use serde::{Deserialize, Serialize};

use crate::settings::Settings;

use super::{
    creep_memory::{CreepMemory, PowerCreepMemory},
    room_memory::RoomMemory, stat_memory::Stats,
};

#[derive(Serialize, Deserialize, Default)]
pub struct GameMemory {
    pub rooms: HashMap<RoomName, RoomMemory>,
    pub creeps: HashMap<String, CreepMemory>,
    pub power_creeps: HashMap<String, PowerCreepMemory>,
    pub settings: Settings,
    // Consider putting stats in a segment instead
    pub stats: Stats,
}

impl GameMemory {
    pub fn new(settings: &Settings) -> Self {
        GameMemory {
            settings: settings.clone(),
            ..Default::default()
        }
    }
}
