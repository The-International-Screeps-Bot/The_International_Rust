use screeps::Room;

use crate::state::game::GameState;

pub struct CommuneOps;

impl CommuneOps {
    pub fn spawn_energy_capacity(room: &Room, game_state: &mut GameState) -> u32 {
        if let Some(spawn_energy_capacity) = game_state.communes_state.spawn_energy_capacities.get_mut(&room.name()) {
            return *spawn_energy_capacity
        }

        let spawn_energy_capacity = room.energy_capacity_available();
        game_state.communes_state.spawn_energy_capacities.insert(room.name(), spawn_energy_capacity);
        spawn_energy_capacity
    }
}