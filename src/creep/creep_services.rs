use std::collections::HashSet;

use js_sys::{JsString, Object, Reflect};
use log::{debug, info};
use screeps::{game, Creep, SharedCreepProperties};
use wasm_bindgen::JsCast;

use crate::{memory::game_memory::GameMemory, state::game::GameState};

use super::creep_ops::CreepOps;

pub struct CreepServices;

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
impl CreepServices {
    pub fn track_creeps(game_state: &GameState) {
        // record creep names to the room they are in (room data)
        // record creep positions to the rooms they are in
        // record creep amounts to the rooms they come from (commune data)

        let creeps = &game_state.creeps;
        for (creep_name, creep) in creeps {

        }
    }

    pub fn run_creeps(game_state: &GameState, memory: &mut GameMemory) {

        let creeps = &game_state.creeps;
        for (creep_name, creep) in creeps {

            debug!("running creep {}", creep.name());

            CreepOps::run_role(&creep, game_state, memory);
    
            if creep.spawning() {
    
                continue;
            }
        }
    }

    pub fn clean_creep_memories(game_state: &GameState, memory: &mut GameMemory) {

        info!("running memory cleanup");
    
        let _ = &memory.creeps.retain(|creep_name, _creep| {

            game_state.creeps.contains_key(creep_name)
        });
    }
}