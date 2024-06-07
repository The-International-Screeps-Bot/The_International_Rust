use js_sys::global;
use screeps::{game, HasPosition};

use crate::{
    memory::game_memory::GameMemory, state::game::GameState, utils::general::GeneralUtils,
};

use super::global_request_ops;

/// Updates and assigns inter-room requests
/// assigning work, combat, etc. to communes

pub fn manage_requests(game_state: &mut GameState, memory: &mut GameMemory) {
    update_requests(game_state, memory);

    if GeneralUtils::is_tick_interval(100) {
        try_assign_requests(game_state, memory);
    }
}

fn update_requests(game_state: &GameState, memory: &mut GameMemory) {
    update_claim_requests(game_state, memory);
    update_work_requests(game_state, memory);
    update_combat_requests(game_state, memory);
}

fn update_claim_requests(game_state: &GameState, memory: &mut GameMemory) {
    let requests = &mut memory.claim_requests;

    for (room_name, request) in requests {
        if let Some(mut abandon) = request.abandon {
            if abandon > 0 {
                request.abandon = Some(abandon - 1);
            }
        }
    }
}

fn update_work_requests(game_state: &GameState, memory: &mut GameMemory) {
    let requests = &mut memory.work_requests;

    for (room_name, request) in requests {
        if let Some(mut abandon) = request.abandon {
            if abandon > 0 {
                request.abandon = Some(abandon - 1);
            }
        }
    }
}

fn update_combat_requests(game_state: &GameState, memory: &mut GameMemory) {
    // TODO: implement
}

fn try_assign_requests(game_state: &GameState, memory: &mut GameMemory) {
    try_assign_claim_requests(game_state, memory);
    try_assign_work_requests(game_state, memory);
    try_assign_combat_requests(game_state, memory);
}

fn try_assign_claim_requests(game_state: &GameState, memory: &mut GameMemory) {
    let gcl_level = game::gcl::level();
    // If our capacity to claim is more or equal to our claim count, stop
    if gcl_level >= game_state.communes.len() as u32 {
        return;
    }

    let claim_requests = &memory.claim_requests;
    for (room_name, request) in claim_requests {
        // Find a commune in range
    }
}

fn try_assign_work_requests(game_state: &GameState, memory: &mut GameMemory) {
    let work_requests = &memory.work_requests;

    for (room_name, request) in work_requests {
        if global_request_ops::is_abandoned(request) {
            continue;
        }

        // Find a commune in range
    }
}

fn try_assign_combat_requests(game_state: &GameState, memory: &mut GameMemory) {
    // TODO: implement
}
