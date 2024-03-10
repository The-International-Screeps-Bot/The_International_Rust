use std::{cell::RefCell, collections::HashMap, mem};

use js_sys::JsString;
use log::{error, warn};
use screeps::{
    game::{
        self,
        map::{RoomStatus, RoomStatusResult},
    },
    raw_memory, RoomName,
};
use serde::{Deserialize, Serialize};

use crate::{
    constants::general::GeneralResult,
    international::collective_ops::CollectiveOps,
    settings::Settings, state::game::GameState,
};

use super::{
    creep_memory::{CreepMemory, PowerCreepMemory},
    game_memory::GameMemory,
    room_memory::RoomMemory,
};

pub struct MemoryOps;

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
impl MemoryOps {
    pub fn load_from_memory_or_default() -> GameMemory {
        let stringified_memory = raw_memory::get().as_string().unwrap();

        let memory = match serde_json::from_str::<GameMemory>(&stringified_memory) {
            Ok(memory) => memory,
            Err(err) => {
                error!("memory parse error, using default {:?}", err);
                return GameMemory::default();
            }
        };

        memory
    }

    pub fn try_apply_settings(memory: &mut GameMemory, settings: &Settings) {
        if memory.breaking_version != settings.breaking_version {}

        memory.breaking_version = settings.breaking_version;
    }

    pub fn write(memory: &GameMemory) {
        match serde_json::to_string(memory) {
            Ok(v) => raw_memory::set(&JsString::from(v)),
            Err(e) => {
                warn!("Memory write error {:?}", e)
            }
        }
    }

    pub fn try_migrate(
        game_state: &GameState,
        settings: &Settings,
        memory: &mut GameMemory,
    ) -> GeneralResult {
        if game_state.init_tick != game_state.tick {
            return GeneralResult::Fail;
        }

        if memory.breaking_version == settings.breaking_version {
            return GeneralResult::Fail;
        }

        // migrate

        MemoryOps::migrate(game_state, settings, memory)
    }

    fn migrate(
        game_state: &GameState,
        settings: &Settings,
        memory: &mut GameMemory,
    ) -> GeneralResult {
        CollectiveOps::kill_all_creeps(game_state);
        let _ = mem::replace(memory, GameMemory::new(&settings));

        GeneralResult::Success
    }

    /// Set raw memory to equal an empty string
    pub fn clear_memory(memory: &mut GameMemory) {
        raw_memory::set(&JsString::from(""));
    }
}

thread_local! { pub static MEMORY: RefCell<GameMemory> = RefCell::new(MemoryOps::load_from_memory_or_default()); }
