use screeps::{Position, RoomName};
use screeps_utils::sparse_cost_matrix::SparseCostMatrix;

use crate::{memory::game_memory::GameMemory, room::room_ops, state::game::GameState};

pub fn economy_room_costs(room_name: &RoomName, game_state: &mut GameState, memory: &GameMemory) -> SparseCostMatrix {
    // // Temporary solution for when we don't have vision
    // if !game_state.rooms.contains_key(room_name) {
    //     return SparseCostMatrix::new();
    // }

    let mut costs = room_ops::default_move_costs(room_name, game_state, memory);
    costs
}