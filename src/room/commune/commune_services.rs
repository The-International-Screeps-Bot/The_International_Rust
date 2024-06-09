use log::info;
use screeps::RoomName;

use crate::{memory::game_memory::GameMemory, state::game::GameState, structures::tower_services};

use super::spawning::spawn_services;

pub fn run_towers(game_state: &mut GameState, memory: &mut GameMemory) {
    let room_names = game_state.communes.clone();

    for room_name in room_names {
        tower_services::run_towers(&room_name, game_state, memory);
    }
}

pub fn run_spawning(game_state: &mut GameState, memory: &mut GameMemory) {
    let room_names = game_state.communes.clone();

    for room_name in room_names {
        spawn_services::try_spawn_creeps(&room_name, game_state, memory);
    }
}