use std::{collections::HashMap, mem};

use js_sys::JsString;
use log::{error, warn};
use screeps::{raw_memory, ConstructionSite, ObjectId, RoomName};
use serde::{Deserialize, Serialize};

use crate::{constants::general::GeneralResult, international::collective_ops, settings::Settings, state::game::GameState, utils::{self, general::GeneralUtils}, SETTINGS};

use super::{
    creep_memory::{CreepMemory, PowerCreepMemory}, global_requests::{ClaimRequests, WorkRequests}, player::{AllyMemory, EnemyMemory}, room_memory::{CenterRoomMemory, CommuneRoomMemory, HighwayRoomMemory, IntersectionRoomMemory, KeeperRoomMemory, NeutralRoomMemory, RemoteRoomMemory, RoomMemory}, stat_memory::StatsMemory
};

#[derive(Serialize, Deserialize)]
pub struct GameMemory {
    pub breaking_version: Option<u32>,
    pub me: String,
    pub rooms: HashMap<RoomName, RoomMemory>,
    pub remotes: HashMap<RoomName, RemoteRoomMemory>,
    pub communes: HashMap<RoomName, CommuneRoomMemory>,
    pub highway: HashMap<RoomName, HighwayRoomMemory>,
    pub intersection: HashMap<RoomName, IntersectionRoomMemory>,
    pub center: HashMap<RoomName, CenterRoomMemory>,
    pub keeper: HashMap<RoomName, KeeperRoomMemory>,
    pub ally: HashMap<RoomName, AllyMemory>,
    pub enemy: HashMap<RoomName, EnemyMemory>,
    pub neutral: HashMap<RoomName, NeutralRoomMemory>,
    pub creeps: HashMap<String, CreepMemory>,
    pub power_creeps: HashMap<String, PowerCreepMemory>,
    // Consider putting stats in a segment instead
    pub stats: StatsMemory,
    pub work_requests: WorkRequests,
    pub claim_requests: ClaimRequests,
    pub combat_stats: HashMap<String, u32>,
    // Consider putting construction sites in a segment
    pub construction_sites: HashMap<ObjectId<ConstructionSite>, u32>,
    pub allies: HashMap<String, AllyMemory>,
    pub enemies: HashMap<String, EnemyMemory>
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
impl GameMemory {
    pub fn new(breaking_version: Option<u32>) -> Self {

        GameMemory {
            breaking_version,
            me: utils::general::me().unwrap(),
            rooms: HashMap::new(),
            remotes: HashMap::new(),
            communes: HashMap::new(),
            highway: HashMap::new(),
            intersection: HashMap::new(),
            center: HashMap::new(),
            keeper: HashMap::new(),
            ally: HashMap::new(),
            enemy: HashMap::new(),
            neutral: HashMap::new(),
            creeps: HashMap::new(),
            power_creeps: HashMap::new(),
            stats: StatsMemory::new(),
            work_requests: WorkRequests::new(),
            claim_requests: ClaimRequests::new(),
            combat_stats: HashMap::new(),
            construction_sites: HashMap::new(),
            allies: HashMap::new(),
            enemies: HashMap::new(),
        }
    }

    pub fn load_from_memory_or_default() -> GameMemory {
        let stringified_memory = raw_memory::get().as_string().unwrap();

        match serde_json::from_str::<GameMemory>(&stringified_memory) {
            Ok(memory) => memory,
            Err(err) => {
                error!("memory parse error, using default {:?}", err);

                // Would not be surprised if this errored, since SETTINGS is made in the same local_thread!{}
                SETTINGS.with_borrow(|settings| {
                    GameMemory::new(Some(settings.breaking_version))
                })
            }
        }
    }

    pub fn try_apply_settings(&mut self, settings: &Settings) {
        if let Some(breaking_version) = self.breaking_version {
            if (breaking_version == settings.breaking_version) {
                return;
            }
        }

        self.breaking_version = Some(settings.breaking_version);
    }

    pub fn write(&mut self) {
        match serde_json::to_string(self) {
            Ok(v) => raw_memory::set(&JsString::from(v)),
            Err(e) => {
                warn!("Memory write error {:?}", e)
            }
        }
    }

    pub fn try_migrate(
        &mut self,
        game_state: &GameState,
        settings: &Settings,
    ) -> GeneralResult {
        if game_state.init_tick != game_state.tick {
            return GeneralResult::Fail;
        }

        if let Some(breaking_version) = self.breaking_version {
            if (breaking_version == settings.breaking_version) {
                return GeneralResult::Fail;
            }
        }

        // migrate

        self.migrate(game_state, settings)
    }

    fn migrate(&mut self,
        game_state: &GameState,
        settings: &Settings,
    ) -> GeneralResult {
        collective_ops::kill_all_creeps(game_state);
        let _ = mem::replace(self, GameMemory::new(Some(settings.breaking_version)));

        GeneralResult::Success
    }

    /// Set raw memory to equal an empty string
    pub fn clear_memory(memory: &mut GameMemory) {
        raw_memory::set(&JsString::from(""));
    }
}
