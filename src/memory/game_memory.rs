use std::{collections::HashMap, mem};

use js_sys::JsString;
use log::{error, info, warn};
use screeps::{ConstructionSite, ObjectId, RoomName, raw_memory};
use serde::{Deserialize, Serialize};

use crate::{
    constants::general::{GeneralError, GeneralResult}, international::collective_ops, memory::global_requests::DefenseRequests, room::room_ops::try_scout_room, settings::Settings, state::game::GameState, utils::{self, general::GeneralUtils}, SETTINGS
};

use super::{
    ally::AllyMemory,
    creep_memory::{CreepMemory, PowerCreepMemory},
    enemy::EnemyMemory,
    global_requests::{AttackRequests, ClaimRequests, WorkRequests},
    room_memory::{
        AllyRoomMemory, CommuneRoomMemory, EnemyRoomMemory, HarvestableRoomMemory,
        HighwayRoomMemory, PortalRoomMemory, RemoteRoomMemory, RoomMemory,
    },
    static_room_memory::{ClaimableRoomMemory, KeeperRoomMemory},
};

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub struct GameMemory {
    #[serde(rename = "0")]
    pub breaking_version: u32,
    #[serde(rename = "1")]
    pub me: String,
    #[serde(rename = "3")]
    pub compressed_memory: bool,
    #[serde(rename = "2")]
    pub rooms: HashMap<RoomName, RoomMemory>,
    pub remotes: HashMap<RoomName, RemoteRoomMemory>,
    pub communes: HashMap<RoomName, CommuneRoomMemory>,
    pub claimable_rooms: HashMap<RoomName, ClaimableRoomMemory>,
    pub highway: HashMap<RoomName, HighwayRoomMemory>,
    pub portal_rooms: HashMap<RoomName, PortalRoomMemory>,
    pub harvestable_rooms: HashMap<RoomName, HarvestableRoomMemory>,
    pub keeper: HashMap<RoomName, KeeperRoomMemory>,
    pub ally: HashMap<RoomName, AllyRoomMemory>,
    pub enemy: HashMap<RoomName, EnemyRoomMemory>,
    pub creeps: HashMap<String, CreepMemory>,
    pub power_creeps: HashMap<String, PowerCreepMemory>,
    pub work_requests: WorkRequests,
    pub claim_requests: ClaimRequests,
    pub attack_requests: AttackRequests,
    pub defense_requests: DefenseRequests,
    pub combat_stats: HashMap<String, u32>,
    // Consider putting construction sites in a segment
    pub construction_sites: HashMap<ObjectId<ConstructionSite>, u32>,
    pub allies: HashMap<String, AllyMemory>,
    pub enemies: HashMap<String, EnemyMemory>,
}

impl GameMemory {
    pub fn new(settings: &Settings) -> Self {
        info!("constructing new GameMemory");

        GameMemory {
            breaking_version: settings.breaking_version,
            compressed_memory: settings.compressed_memory,
            me: utils::general::me().unwrap(),
            rooms: HashMap::new(),
            remotes: HashMap::new(),
            communes: HashMap::new(),
            claimable_rooms: HashMap::new(),
            highway: HashMap::new(),
            portal_rooms: HashMap::new(),
            harvestable_rooms: HashMap::new(),
            keeper: HashMap::new(),
            ally: HashMap::new(),
            enemy: HashMap::new(),
            creeps: HashMap::new(),
            power_creeps: HashMap::new(),
            work_requests: WorkRequests::new(),
            claim_requests: ClaimRequests::new(),
            attack_requests: AttackRequests::new(),
            defense_requests: DefenseRequests::new(),
            combat_stats: HashMap::new(),
            construction_sites: HashMap::new(),
            allies: HashMap::new(),
            enemies: HashMap::new(),
        }
    }

    pub fn load_from_memory_or_default() -> GameMemory {
        SETTINGS.with_borrow(|settings| {
            let memory: Result<GameMemory, GeneralError> = match settings.compressed_memory {
                true => GameMemory::read_base32768_bitcode_or_default(),
                false => GameMemory::read_or_default(),
            };
            
            if let Ok(memory) = memory {
                return memory;
            }
            
            // We were not able to create memory from the game's cache
            
            GameMemory::new(settings)
        })
        
        // let stringified_memory = raw_memory::get().as_string().unwrap();
        // info!("TRYING TO LOAD MEMORY");
        // match serde_json::from_str::<GameMemory>(&stringified_memory) {
        //     Ok(memory) => memory,
        //     Err(err) => {
        //         error!("memory parse error on initial read {:?}", err);

        //         // Would not be surprised if this errored, since SETTINGS is made in the same local_thread!{}
        //         // Doesn't seem to panic so, keep it ig?
        //         SETTINGS.with_borrow(|settings| GameMemory::new(&settings))
        //     }
        // }
    }
    
    fn read_or_default() -> Result<GameMemory, GeneralError> {
        let stringified_memory = raw_memory::get().as_string().unwrap();
        
        match serde_json::from_str::<GameMemory>(&stringified_memory) {
            Ok(memory) => Ok(memory),
            Err(err) => {
                error!("memory parse error on initial read {:?}", err);
                
                Err(GeneralError::Fail)
            }
        }
    }
    
    fn read_base32768_bitcode_or_default() -> Result<GameMemory, GeneralError> {
        let stringified_memory = raw_memory::get().as_string().unwrap();
        
        let mut bits = Vec::new();
        // Try to decode memory to bitcode
        let Ok(res) = base32768::decode(&stringified_memory, &mut bits) else {
            error!("Failed to decode base32768 memory");
            
            return Err(GeneralError::Fail)
        };
        
        // Try to decode bitcode to memory
        let Ok(memory) = bitcode::deserialize::<GameMemory>(&bits) else {
            error!("Failed to decode bitcode memory");
            
            return Err(GeneralError::Fail)
        };
        
        Ok(memory)
    }

    pub fn write(&self) {
        match self.compressed_memory {
            true => self.write_bitcode_base32768(),
            false => self.write_json(),
        }
    }

    /// Write to memory using JSON (ew!)
    pub fn write_json(&self) {
        match serde_json::to_string(self) {
            Ok(v) => raw_memory::set(&JsString::from(v)),
            Err(e) => {
                warn!("Memory write error {:?}", e)
            }
        }
    }

    /// Write to memory using bitcode encoding + base32768
    pub fn write_bitcode_base32768(&self) {
        let Ok(bits) = bitcode::serialize(self) else {
            warn!("Bitcode serialization error");
            return;
        };

        let Ok(base) = base32768::encode(&bits) else {
            warn!("Base32768 encoding error");
            return;
        };

        raw_memory::set(&JsString::from(base));
    }

    pub fn tick_update(&mut self, game_state: &mut GameState, settings: &Settings) {
        self.try_migrate(game_state, settings);
        self.scout_visible_rooms(game_state);
        self.tick_update_commune_memory(game_state);
    }

    pub fn try_migrate(&mut self, game_state: &GameState, settings: &Settings) -> GeneralResult {
        if game_state.init_tick != game_state.tick {
            return GeneralResult::Fail;
        }

        if (self.breaking_version == settings.breaking_version) {
            return GeneralResult::Fail;
        }

        // migrate

        self.migrate(game_state, settings)
    }

    fn migrate(&mut self, game_state: &GameState, settings: &Settings) -> GeneralResult {
        collective_ops::kill_all_creeps(game_state);
        mem::swap(self, &mut GameMemory::new(settings));

        GeneralResult::Success
    }

    /// Set raw memory to equal an empty string
    pub fn clear_memory(memory: &mut GameMemory) {
        raw_memory::set(&JsString::from(""));
    }

    pub fn tick_update_commune_memory(&mut self, game_state: &mut GameState) {
        let commune_names = game_state.communes.clone();
        for room_name in commune_names {
            if self.communes.contains_key(&room_name) {
                continue;
            }

            if let Ok(commune_memory) = CommuneRoomMemory::new(&room_name, game_state) {
                self.communes.insert(room_name, commune_memory);
            }
        }
    }

    pub fn update_claimable_room_memory(&mut self, game_state: &mut GameState) {
        let room_names: Vec<RoomName> = game_state.rooms.keys().cloned().collect();
        for room_name in room_names {
            if self.claimable_rooms.contains_key(&room_name) {
                continue;
            };

            if let Ok(claimable_memory) = ClaimableRoomMemory::new(&room_name, game_state, self) {
                self.claimable_rooms.insert(room_name, claimable_memory);
            }
        }
    }

    pub fn scout_visible_rooms(&mut self, game_state: &mut GameState) {
        let room_names: Vec<RoomName> = game_state.rooms.keys().cloned().collect();

        for room_name in room_names {
            try_scout_room(&room_name, game_state, self);
        }
    }
}

#[cfg(test)]
mod tests {
    use screeps::RoomName;
    use wasm_bindgen_test::*;

    use crate::{constants::general::GeneralResult, memory::{game_memory::GameMemory, room_memory::{self, RoomMemory}}, settings::Settings, state::game::GameState};

    #[wasm_bindgen_test]
    fn test_memory_compressed() {
        let mut memory = GameMemory::new(&Settings::new());
        let mut game_state = GameState::new();
        
        let room_name = RoomName::new("W1N1").unwrap();
        let room_memory = RoomMemory::new(&room_name, &mut game_state, &mut memory).ok().unwrap();
        memory.rooms.insert(room_name, room_memory);

        memory.write();
        let read_memory = GameMemory::read_or_default();

        // eprintln!("read memory {:?}", read_memory);
        
        assert!(read_memory.is_ok());
    }
    
    #[wasm_bindgen_test]
    fn pass() {
        assert_eq!(1, 1);
    }
}