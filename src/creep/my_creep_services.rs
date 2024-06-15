use std::collections::{HashMap, HashSet};

use log::{debug, info};
use screeps::{Creep, Position, RoomName, SharedCreepProperties};

use super::{
    creep_move_ops::{self, assign_move_target_as_pos},
    my_creep_ops,
};
use crate::{
    constants::creep::MoveTargets, memory::game_memory::GameMemory, state::game::GameState,
};

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn track_creeps(game_state: &GameState) {
    // record creep names to the room they are in (room data)
    // record creep positions to the rooms they are in
    // record creep amounts to the rooms they come from (commune data)

    let creeps = &game_state.creeps;
    for (creep_name, creep) in creeps {}
}

// Not part of design philosphy
// pub fn run_creeps(game_state: &mut GameState, memory: &mut GameMemory) {
//     let creep_names: Vec<String> = game_state.creeps.keys().cloned().collect();
//     for creep_name in &creep_names {

//         let creep = game_state.creeps.get(creep_name).unwrap();

//         if creep.inner().spawning() {
//             continue;
//         }

//         my_creep_ops::run_role(creep_name, game_state, memory);
//     }
// }

pub fn clean_creep_memories(game_state: &GameState, memory: &mut GameMemory) {
    info!("running memory cleanup");

    let _ = &memory
        .creeps
        .retain(|creep_name, _creep| game_state.creeps.contains_key(creep_name));
}

pub fn move_creeps(game_state: &mut GameState, memory: &mut GameMemory) {
    let room_names = game_state.rooms.keys().cloned().collect::<Vec<_>>();
    for room_name in room_names {
        let mut move_targets: HashMap<Position, String> = HashMap::new();

        register_move_targets(game_state, &room_name, &mut move_targets);

        run_move_requests(game_state, memory, &room_name, &mut move_targets);
    }
}

fn register_move_targets(
    game_state: &mut GameState,
    room_name: &RoomName,
    move_targets: &mut MoveTargets,
) {
    let room_state = game_state.room_states.get(&room_name).unwrap();
    let creep_names: Vec<String> = room_state.my_creeps.clone();

    for creep_name in creep_names {
        assign_move_target_as_pos(creep_name.as_str(), game_state, move_targets)
    }
}

fn run_move_requests(
    game_state: &mut GameState,
    memory: &GameMemory,
    room_name: &RoomName,
    move_targets: &mut MoveTargets,
) {
    let room_state = game_state.room_states.get(&room_name).unwrap();
    let creep_names: Vec<String> = room_state.my_creeps.clone();

    for creep_name in creep_names {
        creep_move_ops::try_run_move_request(
            creep_name.as_str(),
            room_name,
            game_state,
            memory,
            move_targets,
        );
    }
}
