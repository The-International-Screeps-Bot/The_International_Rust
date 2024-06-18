use log::info;
use screeps::RoomName;

use crate::{creep::my_creep_services, memory::game_memory::GameMemory, state::game::GameState, structures::tower_services};

use super::{defense_ops, spawning::spawn_services};

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn run_towers(game_state: &mut GameState, memory: &mut GameMemory) {
    let room_names = game_state.communes.clone();

    for room_name in room_names {
        tower_services::run_towers(&room_name, game_state, memory);
    }
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn run_spawning(game_state: &mut GameState, memory: &mut GameMemory) {
    let room_names = game_state.communes.clone();

    for room_name in room_names {
        spawn_services::try_spawn_creeps(&room_name, game_state, memory);
    }
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn move_creeps(game_state: &mut GameState, memory: &mut GameMemory) {

    let room_names = game_state.communes.clone();

    for room_name in room_names {
        my_creep_services::move_creeps(game_state, memory);
    }
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn try_active_safe_mode(game_state: &mut GameState, memory: &mut GameMemory) {
    let room_names = game_state.communes.clone();

    for room_name in room_names {
        defense_ops::try_active_safe_mode(&room_name, game_state, memory);
    }
}