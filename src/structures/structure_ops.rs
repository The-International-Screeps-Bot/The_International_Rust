use screeps::{HasId, Structure};

use crate::state::game::GameState;

pub fn is_active(structure: &Structure, game_state: &mut GameState) -> bool {
    if let Some(is_active) = game_state
        .structures_state
        .active_statuses
        .get(&structure.id())
    {
        return *is_active;
    }

    let is_active = structure.is_active();

    game_state
        .structures_state
        .active_statuses
        .insert(structure.id(), is_active);
    is_active
}
