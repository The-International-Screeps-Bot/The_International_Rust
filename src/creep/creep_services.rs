use std::collections::HashSet;

use log::{debug, info};
use screeps::{Creep, SharedCreepProperties};

use super::{creep_move_ops::CreepMoveOps, owned_creep_ops::OwnedCreepOps};
use crate::{memory::game_memory::GameMemory, state::game::GameState};

pub struct CreepServices;

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
impl CreepServices {
    pub fn track_creeps(game_state: &GameState) {
        // record creep names to the room they are in (room data)
        // record creep positions to the rooms they are in
        // record creep amounts to the rooms they come from (commune data)

        let creeps = &game_state.creeps;
        for (creep_name, creep) in creeps {}
    }

    pub fn run_creeps(game_state: &mut GameState, memory: &mut GameMemory) {
        
        let creep_names: Vec<String> = game_state.creeps.keys().cloned().collect();
        for creep_name in &creep_names {
            debug!("running creep {}", creep_name);

            let creep = game_state.creeps.get(creep_name).unwrap();

            if creep.inner().spawning() {
                continue;
            }

            OwnedCreepOps::run_role(creep_name, game_state, memory);
        }
    }

    pub fn clean_creep_memories(game_state: &GameState, memory: &mut GameMemory) {
        info!("running memory cleanup");

        let _ = &memory
            .creeps
            .retain(|creep_name, _creep| game_state.creeps.contains_key(creep_name));
    }

    pub fn move_creeps(game_state: &GameState) {

        let creep_names: Vec<String> = game_state.creeps.keys().cloned().collect();
        for creep_name in &creep_names {
            CreepMoveOps::try_run_move_request(creep_name, game_state, &mut HashSet::new());
        }

        // for creep_name in game_state.creeps.keys().into_iter().collect::<Vec<String>>() {

        // }

        let creeps = game_state.creeps.values();
        for creep/* (creep_name, creep) */ in creeps {

            CreepMoveOps::try_run_move_request(&creep.inner().name(), game_state, &mut HashSet::new());
        }
    }
}
