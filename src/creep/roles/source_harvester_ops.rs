use screeps::{ObjectId, RoomName, SharedCreepProperties};

use crate::{
    constants::creep::CreepPart, creep::{any_creep_ops, my_creep_ops}, memory::game_memory::GameMemory, room::room_ops, state::game::GameState
};

pub fn register_harvest_strength(creep_name: &str, room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) {

    let creep_memory = memory.creeps.get(creep_name).unwrap();
    let Some(source_index) = creep_memory.source_index else {
        return
    };

    // let creep_state = game_state.my_creep_states.get_mut(creep_name).unwrap();

    let creep = game_state.creeps.get(creep_name).unwrap();
    let work_parts = my_creep_ops::get_parts_by_type(creep_name, game_state)[CreepPart::Work];

    let commune_state = game_state.commune_states.get_mut(room_name).unwrap();

    commune_state.source_harvest_strengths[source_index] += work_parts;
}

pub fn try_harvest(creep_name: &str, game_state: &mut GameState, memory: &mut GameMemory) {

    let creep = game_state.creeps.get(creep_name).unwrap();
    let creep_memory = memory.creeps.get(creep_name).unwrap();

    let room = game_state.rooms.get(&creep.inner().room().unwrap().name().clone()).unwrap();
    let sources = room_ops::get_sources(&room.name(), game_state);

    let Some(source_index) = creep_memory.source_index else {
        return;
    };

    let source = sources.get(source_index).unwrap();

    my_creep_ops::drop_harvest(&creep_name.to_string(), source, game_state, memory);
}
