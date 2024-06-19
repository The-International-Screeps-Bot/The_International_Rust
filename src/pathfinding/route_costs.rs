use std::collections::{HashMap, HashSet};

use screeps::RoomName;

use crate::memory::{game_memory::GameMemory, room_memory::RoomType};

pub fn weight_room_types<'a>(room_type_weights: HashMap<RoomType, u8>, memory: &'a GameMemory) -> impl Fn(&RoomName) -> u8 + 'a {

    let f = move |room_name: &RoomName| -> u8 {
        let Some(room_memory) = memory.rooms.get(room_name) else {
            return u8::MAX
        };

        let Some(weight) = room_type_weights.get(&room_memory.room_type) else {
            return 0
        };

        *weight
    };

    f
}