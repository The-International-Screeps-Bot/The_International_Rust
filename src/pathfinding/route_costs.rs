use std::collections::{HashMap, HashSet};

use screeps::RoomName;

use crate::memory::{game_memory::GameMemory, room_memory::StaticRoomType};

/* pub fn weight_room_types<'a>(
    room_type_weights: HashMap<RoomType, u8>,
    memory: &'a GameMemory,
) -> impl for<'a> Fn(&'a RoomName) -> u8 {
    let f: (dyn for<'a> Fn(&'a RoomName) -> u8 + 'static) = |room_name: &RoomName| -> u8 {
        let Some(room_memory) = memory.rooms.get(room_name) else {
            return u8::MAX;
        };

        let Some(weight) = room_type_weights.get(&room_memory.room_type) else {
            return 0;
        };

        *weight
    };

    f
} */

pub fn economy_creep_costs(room_name: &RoomName, memory: &GameMemory) -> u8 {
    let Some(room_memory) = memory.rooms.get(room_name) else {
        return u8::MAX;
    };

    let room_type_weights = HashMap::from([
        (StaticRoomType::Commune, 1),
        (StaticRoomType::Ally, 1),
        (StaticRoomType::Center, 1),
        (StaticRoomType::Highway, 1),
        (StaticRoomType::Keeper, 1),
        (StaticRoomType::Enemy, 2),
        (StaticRoomType::Neutral, 2),
    ]);

    let Some(weight) = room_type_weights.get(&room_memory.room_type) else {
        return 0;
    };

    *weight
}
