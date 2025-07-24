use std::collections::{HashMap, HashSet};

use screeps::RoomName;

use crate::{memory::{game_memory::GameMemory, room_memory::StaticRoomType}, settings::Settings};

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

    // Room owner
    if let Some(claimable_memory) = memory.claimable_rooms.get(room_name) {
        // If we own the room, return 0 cost
        if claimable_memory.my_claim.is_some() {
            return 0;
        }

        // If someone else owns the room 
        if let Some(non_me_owner) = &claimable_memory.non_me_owner {
            // If it's an ally we're happy to pass through
            if memory.allies.contains_key(non_me_owner) {
                return 0;
            }
            // Otherwise the room is owner by an enemy
            else {
                return u8::MAX;

            }
        }
    }

    // If there is danger, avoid the room
    if let Some(danger) = room_memory.danger {
        if danger > 0 {
            return u8::MAX;
        }
    }

    let room_type_weights = HashMap::from([
        (StaticRoomType::Claimable, 1),
        (StaticRoomType::Center, 1),
        (StaticRoomType::CardinalHighway, 1),
        (StaticRoomType::Intersection, 1),
        (StaticRoomType::Keeper, 1),
    ]);

    let Some(weight) = room_type_weights.get(&room_memory.room_type) else {
        return 0;
    };

    *weight
}
