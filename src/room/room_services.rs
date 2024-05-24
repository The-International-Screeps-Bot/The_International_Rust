use screeps::RoomName;

use crate::{
    memory::{game_memory::GameMemory, room_memory},
    state::game::GameState,
};

use super::{commune::spawning::spawn_services::SpawnServices, room_ops::RoomOps};

pub struct RoomServices;

impl RoomServices {
    pub fn move_creeps(game_state: &mut GameState, memory: &mut GameMemory) {

        let room_names: Vec<RoomName> = game_state.rooms.keys().cloned().collect();
        for room_name in &room_names {
            
            RoomOps::move_creeps(room_name, game_state, memory);
        }
    }

    pub fn spawn_creeps(game_state: &mut GameState, memory: &mut GameMemory) {

        let room_names: Vec<RoomName> = game_state.rooms.keys().cloned().collect();
        for room_name in &room_names {
            
            SpawnServices::try_spawn_creeps(room_name, game_state, memory);
        }
    }
}
