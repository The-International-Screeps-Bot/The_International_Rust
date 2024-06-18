use std::{collections::HashMap, mem};

use js_sys::JsString;
use log::{error, info, warn};
use screeps::{raw_memory, ConstructionSite, ObjectId, RoomName};
use serde::{Deserialize, Serialize};

use crate::{constants::general::GeneralResult, international::collective_ops, settings::Settings, state::game::GameState, utils::{self, general::GeneralUtils}, SETTINGS};

use super::{
    ally::AllyMemory, creep_memory::{CreepMemory, PowerCreepMemory}, enemy::EnemyMemory, global_requests::{ClaimRequests, WorkRequests}, room_memory::{AllyRoomMemory, CenterRoomMemory, CommuneRoomMemory, EnemyRoomMemory, HighwayRoomMemory, IntersectionRoomMemory, KeeperRoomMemory, NeutralRoomMemory, RemoteRoomMemory, RoomMemory},
};

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
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
    pub ally: HashMap<RoomName, AllyRoomMemory>,
    pub enemy: HashMap<RoomName, EnemyRoomMemory>,
    pub neutral: HashMap<RoomName, NeutralRoomMemory>,
    pub creeps: HashMap<String, CreepMemory>,
    pub power_creeps: HashMap<String, PowerCreepMemory>,
    pub work_requests: WorkRequests,
    pub claim_requests: ClaimRequests,
    pub combat_stats: HashMap<String, u32>,
    // Consider putting construction sites in a segment
    pub construction_sites: HashMap<ObjectId<ConstructionSite>, u32>,
    pub allies: HashMap<String, AllyMemory>,
    pub enemies: HashMap<String, EnemyMemory>
}

impl GameMemory {
    pub fn new(breaking_version: Option<u32>) -> Self {

        info!("constructing new GameMemory");

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
                error!("memory parse error on initial read {:?}", err);

                // Would not be surprised if this errored, since SETTINGS is made in the same local_thread!{}
                SETTINGS.with_borrow(|settings| {
                    GameMemory::new(Some(settings.breaking_version))
                })
            }
        }
    }

    pub fn write(&mut self) {
        match serde_json::to_string(self) {
            Ok(v) => raw_memory::set(&JsString::from(v)),
            Err(e) => {
                warn!("Memory write error {:?}", e)
            }
        }
    }

    pub fn tick_update(&mut self, game_state: &mut GameState, settings: &Settings) {
        
        self.try_migrate(game_state, settings);
        self.tick_update_commune_memory(game_state);
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
        mem::swap(self, &mut GameMemory::new(Some(settings.breaking_version)));

        GeneralResult::Success
    }

    /// Set raw memory to equal an empty string
    pub fn clear_memory(memory: &mut GameMemory) {
        raw_memory::set(&JsString::from(""));
    }

    pub fn tick_update_commune_memory(&mut self, game_state: &mut GameState) {

        let commune_names = game_state.communes.clone();
        for room_name in commune_names {

            self.communes.entry(room_name).or_insert_with(|| CommuneRoomMemory::new(&room_name, game_state));
        }
    }
}