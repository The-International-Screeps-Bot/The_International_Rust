use screeps::RoomName;

use crate::{constants::creep::CreepRole, memory::game_memory::GameMemory, state::game::GameState};

use super::roles::{scout_ops, source_harvester_ops};

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn try_register_scout_targets(game_state: &mut GameState, memory: &mut GameMemory) {
    let room_names = game_state.rooms.keys().cloned().collect::<Vec<_>>();

    for room_name in room_names {
        let room_state = game_state.room_states.get(&room_name).unwrap();

        for creep_name in room_state.creeps_by_role[CreepRole::Scout].clone() {
            scout_ops::try_register_scout_target(&creep_name, game_state, memory);
        }
    }
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn try_scouts(game_state: &mut GameState, memory: &mut GameMemory) {
    let room_names = game_state.rooms.keys().cloned().collect::<Vec<_>>();
    for room_name in room_names {
        let room_state = game_state.room_states.get(&room_name).unwrap();
        for creep_name in room_state.creeps_by_role[CreepRole::Scout].clone() {
            scout_ops::try_scout(&creep_name, &room_name, game_state, memory);
        }
    }
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn harvest_commune_sources(game_state: &mut GameState, memory: &mut GameMemory) {
    let room_names = game_state.rooms.keys().cloned().collect::<Vec<_>>();
    for room_name in room_names {
        let room_state = game_state.room_states.get(&room_name).unwrap();
        for creep_name in room_state.creeps_by_role[CreepRole::SourceHarvester].clone() {}
    }
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn harvest_remote_sources(game_state: &mut GameState, memory: &mut GameMemory) {
    let room_names = game_state.rooms.keys().cloned().collect::<Vec<_>>();
    for room_name in room_names {
        let room_state = game_state.room_states.get(&room_name).unwrap();
        for creep_name in room_state.creeps_by_role[CreepRole::SourceHarvester].clone() {}
    }
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn register_commune_harvest_strength(game_state: &mut GameState, memory: &mut GameMemory) {
    let room_names = game_state.rooms.keys().cloned().collect::<Vec<_>>();
    for room_name in room_names {
        let room_state = game_state.room_states.get(&room_name).unwrap();
        for creep_name in room_state.creeps_by_role[CreepRole::SourceHarvester].clone() {
            source_harvester_ops::register_harvest_strength(
                creep_name.as_str(),
                &room_name,
                game_state,
                memory,
            )
        }
    }
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn try_harvest_commune_sources(game_state: &mut GameState, memory: &mut GameMemory) {
    let room_names = game_state.rooms.keys().cloned().collect::<Vec<RoomName>>();
    for room_name in room_names {
        let room_state = game_state.room_states.get(&room_name).unwrap();
        for creep_name in room_state.creeps_by_role[CreepRole::SourceHarvester].clone() {
            source_harvester_ops::try_harvest(creep_name.as_str(), game_state, memory);
        }
    }
}
