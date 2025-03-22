use screeps::{Position, RoomName};
use screeps_utils::sparse_cost_matrix::SparseCostMatrix;

use crate::{room::room_ops::sparse_terrain, state::game::GameState};

pub fn economy_room_costs(room_name: &RoomName, game_state: &mut GameState) -> SparseCostMatrix {
    let mut costs = sparse_terrain(room_name, game_state);

    costs
}