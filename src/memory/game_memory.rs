use std::collections::HashMap;

use screeps::{ConstructionSite, ObjectId, RoomName};
use serde::{Deserialize, Serialize};

use crate::{settings::Settings, utils::general::GeneralUtils};

use super::{
    creep_memory::{CreepMemory, PowerCreepMemory}, global_requests::{ClaimRequests, WorkRequests}, room_memory::RoomMemory, stat_memory::Stats
};

#[derive(Serialize, Deserialize, Default)]
pub struct GameMemory {
    pub breaking_version: u32,
    pub me: String,
    pub rooms: HashMap<RoomName, RoomMemory>,
    pub creeps: HashMap<String, CreepMemory>,
    pub power_creeps: HashMap<String, PowerCreepMemory>,
    // Consider putting stats in a segment instead
    pub stats: Stats,
    pub work_requests: WorkRequests,
    pub claim_requests: ClaimRequests,
    pub combat_stats: HashMap<String, u32>,
    // Consider putting construction sites in a segment
    pub construction_sites: HashMap<ObjectId<ConstructionSite>, u32>,
}

impl GameMemory {
    pub fn new(settings: &Settings) -> Self {



        GameMemory {
            breaking_version: settings.breaking_version,
            me: GeneralUtils::me().unwrap(),
            ..Default::default()
        }
    }
}
