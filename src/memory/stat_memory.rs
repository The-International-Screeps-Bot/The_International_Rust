use std::collections::HashMap;

use screeps::RoomName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Stats {
    pub creep_count: u32,
    pub power_creep_count: u32,
    pub combined_rcl: u32,
    pub gcl_progress: u64,
    pub gcl_total: u64,
    pub gpl_progress: u64,
    pub gpl_total: u64,
    pub rooms: HashMap<RoomName, RoomStats>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct RoomStats {
    creep_count: u32,
    stored_energy: u32,
}