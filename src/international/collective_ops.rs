use crate::{constants::general::{GeneralResult, NON_COMMUNE_SIGNS}, memory::game_memory::GameMemory, state::game::GameState};

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub fn funnel_order(game_state: &GameState) {}

pub fn kill_all_creeps(game_state: &GameState) {
    let creeps = &game_state.creeps;
    for (creep_name, creep) in creeps {
        let _ = creep.inner().suicide();
    }
}

/// Generate a new unique creep id and increment the tracker
pub fn new_creep_id(game_state: &mut GameState, memory: &mut GameMemory) -> Result<u32, GeneralResult> {

    // increase the id index until it doesn't match an existing creep's name,
    // make that the new creep id index
    while game_state.creep_id_index < u32::MAX {
        game_state.creep_id_index += 1;

        if memory.creeps.contains_key(&game_state.creep_id_index.to_string()) {
            continue;
        }

        return Ok(game_state.creep_id_index);
    }

    Err(GeneralResult::Fail)
}

pub fn commune_sign(game_state: &mut GameState, memory: &mut GameMemory) -> String {
    format!("A commune of the working creeps: No Capitalists allowed! Collectivized v3{:?}", memory.breaking_version)
}

pub fn non_commune_sign(game_state: &mut GameState, memory: &mut GameMemory) -> String {
    fastrand::choice(NON_COMMUNE_SIGNS).unwrap().to_string()
}