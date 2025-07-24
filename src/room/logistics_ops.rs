use screeps::{spawn, Resource, ResourceType, RoomName, StructureExtension};

use crate::{constants::structure::SpawningStructure, state::game::GameState};

use super::room_ops;

pub fn create_spawning_structure_logistics_requests(room_name: &RoomName, game_state: &mut GameState) {
    let spawning_structures: Vec<SpawningStructure> = Vec::new();

    for structure in spawning_structures {


    }
}

pub fn create_power_spawn_logistics_requests(room_name: &RoomName, game_state: &mut GameState) {
    let room_state = game_state.room_states.get(room_name).unwrap();

    let power_spawns = &room_ops::structures_by_type(room_name, game_state).power_spawn;
    for power_spawn in power_spawns {
        let store = power_spawn.store();

        if store.get_used_capacity(Some(ResourceType::Energy)) == store.get_capacity(Some(ResourceType::Energy)) {

        }

        if store.get_used_capacity(Some(ResourceType::Power)) == store.get_capacity(Some(ResourceType::Power)) {

        }
    }
}