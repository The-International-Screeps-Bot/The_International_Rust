use std::collections::HashMap;

use enum_map::EnumMap;
use screeps::RoomName;
use serde::{Deserialize, Serialize};

use crate::constants::creep::CreepRole;

#[derive(Serialize, Deserialize, Default)]
pub struct StatsMemory {
    pub total_creeps: u32,
    pub alive_power_creeps: u32,
    pub power_creep_count: u32,
    pub combined_rcl: u32,
    pub gcl_progress: u64,
    pub gcl_total: u64,
    pub gpl_progress: u64,
    pub gpl_total: u64,
    pub remotes: HashMap<RoomName, RemoteStatsMemory>,
    pub communes: HashMap<RoomName, CommuneStatsMemory>,
    pub cpu_used: u32,
    pub game_time: u32,
}

impl StatsMemory {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct RemoteStatsMemory {

}

impl RemoteStatsMemory {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct CommuneStatsMemory {
    pub creeps: u32,
    pub creeps_from_room: u32,
    pub power_creeps: u32,
    pub stored_energy: u32,
    pub energy_out_upgrade: u32,
    pub energy_out_build: u32,
    pub energy_out_repair_economy: u32,
    pub energy_out_repair_barricades: u32,
    pub energy_out_spawn: u32,
    pub energy_out_renew: u32,
    pub energy_out_terminal_domestic: u32,
    pub energy_out_terminal_foreign: u32,
    pub energy_out_terminal_transaction_costs: u32,
    pub controller_level: u32,
    pub remote_count: u32,
    pub minerals_harvested: u32,
    pub min_hauler_cost: u32,
    pub spawn_usage_percent: u32,
}

impl CommuneStatsMemory {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}