use screeps::{ObjectId, Position, RoomName, SharedCreepProperties};

use crate::{
    constants::creep::CreepPart,
    creep::{any_creep_ops, my_creep_ops},
    memory::game_memory::GameMemory,
    room::room_ops,
    state::game::GameState,
};

pub fn register_source(
    creep_name: &str,
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) {
    let creep_memory = memory.creeps.get(creep_name).unwrap();
    let Some(source_index) = creep_memory.source_index else {
        return;
    };

    // Register harvest strength

    let creep = game_state.creeps.get(creep_name).unwrap();
    let work_parts = my_creep_ops::get_parts_by_type(creep_name, game_state)[CreepPart::Work];

    let commune_state = game_state.commune_states.get_mut(room_name).unwrap();
    commune_state.source_harvest_strengths[source_index] += work_parts;

    // Register source position
    try_register_harvest_pos(creep_name, room_name, source_index, game_state, memory)
}

fn try_register_harvest_pos(
    creep_name: &str,
    room_name: &RoomName,
    source_index: usize,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) {
    let creep_memory = memory.creeps.get(creep_name).unwrap();

    // If we already have an existing harvest position, record it and stop
    if let Some(harvest_pos) = creep_memory.harvest_pos {
        let commune_state = game_state.commune_states.get_mut(room_name).unwrap();
        commune_state.reserved_positions.insert(harvest_pos);

        return;
    };

    // Try to find a new harvest pos

    let harvestable_room_memory = memory.harvestable_rooms.get(room_name).unwrap();

    let room_state = game_state.room_states.get(room_name).unwrap();
    let harvest_positions = room_ops::harvest_positions(room_name, game_state, memory).unwrap();

    let harvest_positions = &harvest_positions[source_index];
    let mut harvest_pos: Option<Position> = None;

    let commune_state = game_state.commune_states.get(room_name).unwrap();

    for pos in harvest_positions.iter() {
        // Skip the pos if it is reserved by a creep
        if commune_state.reserved_positions.contains(pos) {
            continue;
        }

        harvest_pos = Some(*pos);
        break;
    }

    // If we were unable to find a harvest pos, stop
    let Some(harvest_pos) = harvest_pos else {
        log::warn!("Failed to get harvest position for creep {}", creep_name);
        return;
    };

    // Register the harvest pos

    let commune_state = game_state.commune_states.get_mut(room_name).unwrap();
    commune_state.reserved_positions.insert(harvest_pos);

    // Record it in our memory

    let creep_memory = memory.creeps.get_mut(creep_name).unwrap();
    creep_memory.harvest_pos = Some(harvest_pos);
}

pub fn try_harvest(creep_name: &str, game_state: &mut GameState, memory: &mut GameMemory) {
    let creep = game_state.creeps.get(creep_name).unwrap();
    let creep_memory = memory.creeps.get(creep_name).unwrap();

    let room = game_state
        .rooms
        .get(&creep.inner().room().unwrap().name().clone())
        .unwrap();
    let sources = room_ops::get_sources(&room.name(), game_state);

    let Some(source_index) = creep_memory.source_index else {
        return;
    };

    let source = sources.get(source_index).unwrap();

    my_creep_ops::drop_harvest(&creep_name.to_string(), source, game_state, memory);
}
