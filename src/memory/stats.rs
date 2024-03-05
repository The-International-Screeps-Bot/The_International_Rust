use std::collections::HashMap;

use screeps::RoomName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Stats {
    pub creep_count: u32,
    pub power_creep_count: u32,
    pub rooms: HashMap<RoomName, RoomStats>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct RoomStats {
    creep_count: u32,
    stored_energy: u32,
}