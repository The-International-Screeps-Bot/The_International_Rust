use screeps::{Room, StructureSpawn, StructureType};

use crate::{
    room::room_ops::RoomOps,
    state::{game::GameState, room::RoomState},
};

use super::spawn_request_arg_services::SpawnRequestArgServices;

pub struct SpawnServices;

impl SpawnServices {
    pub fn try_spawn_creeps(room: &Room, room_state: &mut RoomState, game_state: &GameState) {
        let structures = RoomOps::structures(room, room_state);
        let Some(spawns) = structures.get(&StructureType::Spawn) else {
            return;
        };

        let active_spawns: Vec<&StructureSpawn> = Vec::new();
        let inactive_spawns: Vec<&StructureSpawn> = Vec::new();

        for spawn in spawns {
            // if spawn.spawning() {
            //     active_spawns.push(spawn);
            //     continue;
            // }
            // inactive_spawns.push(spawn);
        }

        Self::try_use_inactive_spawns(room, room_state, game_state, inactive_spawns);
    }

    fn try_use_inactive_spawns(room: &Room, room_state: &mut RoomState, game_state: &GameState, inactive_spawns: Vec<&StructureSpawn>) {

        let spawn_request_args = SpawnRequestArgServices::create_spawn_request_args(room, room_state, game_state);

        for spawn in inactive_spawns {

        }
    }
}
