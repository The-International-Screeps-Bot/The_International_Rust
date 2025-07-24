use std::collections::HashSet;

use screeps::{HasPosition, Position, RoomName};
use screeps_utils::sparse_cost_matrix::SparseCostMatrix;

use crate::{creep::any_creep_ops, memory::{enemy, game_memory::GameMemory}, room::room_ops, state::game::GameState};

pub fn try_active_safe_mode(room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) {

    if !can_safe_mode(room_name, game_state, memory) {
        return
    }

    if !should_safe_mode_for_base(room_name, game_state, memory) {
        return
    }

    let controller = room_ops::controller(room_name, game_state);
    let Some(controller) = controller else {
        return
    };

    controller.activate_safe_mode();
}

fn can_safe_mode(room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) -> bool {
    let controller = room_ops::controller(room_name, game_state);
    let Some(controller) = controller else {
        return false
    };
       
    let Some(safe_mode_cooldown) = controller.safe_mode_cooldown() else {
        // Consider recording the tick in room_state for when the safemode cooldown will be off
        return false
    };

    return true
}

fn should_safe_mode_for_base(room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) -> bool {
    let base_positions: HashSet<Position> = HashSet::new();

    let enemy_creeps = room_ops::not_my_creeps(room_name, game_state, memory).enemy;

    for enemy_creep in enemy_creeps {
        // Check if the creep has combat capabilities

        if !base_positions.contains(&enemy_creep.pos()) {
            continue;
        };

        // Otherwise, the creep is standing on a base position

        return true
    }

    return false
}

fn should_safe_mode_for_controller(room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) -> bool {
    return true
}