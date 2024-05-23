use screeps::{Room, RoomName, StructureSpawn, StructureType};

use crate::{
    memory::{game_memory::GameMemory, room_memory::RoomMemory}, room::room_ops::RoomOps, state::{commune::CommuneState, game::GameState, room::RoomState}
};

use super::spawn_request_arg_services::SpawnRequestArgServices;

pub struct SpawnServices;

impl SpawnServices {
    pub fn try_spawn_creeps(room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) {
        let structures = RoomOps::structures(room_name, game_state);

        let mut active_spawns: Vec<&StructureSpawn> = Vec::new();
        let mut inactive_spawns: Vec<&StructureSpawn> = Vec::new();

        for spawn in &structures.spawn {
            match spawn.spawning() {
                Some(spawning) => active_spawns.push(spawn),
                _ => inactive_spawns.push(spawn),
            }
        }

        Self::try_use_inactive_spawns(room_name, game_state, memory);
    }

    fn try_use_inactive_spawns(room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) {

        let spawn_request_args = SpawnRequestArgServices::create_spawn_request_args(room_name, game_state, memory);

        let mut inactive_spawns: Vec<&StructureSpawn> = Vec::new();
        for spawn in inactive_spawns {

        }
    }
}
