use screeps::RoomName;

use crate::{
    memory::{game_memory::GameMemory, room_memory},
    state::game::GameState,
};

use super::{room_ops::RoomOps};

pub struct RoomServices;

impl RoomServices {
    pub fn move_creeps(game_state: &mut GameState, memory: &mut GameMemory) {
        let room_names: Vec<RoomName> = game_state.rooms.keys().cloned().collect();
        for room_name in &room_names {
            RoomOps::move_creeps(room_name, game_state, memory);
        }
    }

    /* pub fn test_state(game_state: &mut GameState, memory: &mut GameMemory) {
        for (room_name, room) in &game_state.rooms {

            let room_state = game_state.room_states.get_mut(room_name).unwrap();

            // RoomOps::test(room, room_state, game_state, memory);
        }
    }

    pub fn test_state_cell(game_state: &mut GameState, memory: &mut GameMemory) {
        for (room_name, room_state) in &game_state.room_states {

            RoomOps::test_state_name(room_name, &mut room_state.borrow_mut(), game_state, memory)
        }
    }

    pub fn test_state_cell_alt(game_state: &mut GameState, memory: &mut GameMemory) {
        for (room_name, room) in &mut game_state.rooms {

            let room_state = game_state.room_states.get_mut(room_name).unwrap();

            RoomOps::test_state(&room.borrow(), &mut room_state.borrow_mut(), game_state, memory)
        }
    }

    pub fn test_name(game_state: &mut GameState, memory: &mut GameMemory) {
        for (room_name, room) in game_state.rooms {

            RoomOps::test_name(&room_name, &room.borrow(), game_state, memory);
        }
    }

    pub fn test_name_alt(game_state: &mut GameState, memory: &mut GameMemory) {
        for (room_name, room) in &game_state.rooms {

            RoomOps::test_name(room_name, &room.borrow(), game_state, memory);
        }
    } */
}
