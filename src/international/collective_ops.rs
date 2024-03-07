use crate::{constants::general::GeneralResult, state::game::GameState};

pub struct CollectiveOps;

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
impl CollectiveOps {
    pub fn funnel_order(game_state: &GameState) {}

    pub fn kill_all_creeps(game_state: &GameState) {
        let creeps = &game_state.creeps;
        for (creep_name, creep) in creeps {
            let _ = creep.inner().suicide();
        }
    }

    /// Generate a new unique creep id and increment the tracker
    pub fn new_creep_id(game_state: &mut GameState) -> Result<u32, GeneralResult> {
        let mut creep_id_index = game_state.creep_id_index;

        // increase the id index until it doesn't match an existing creep's name,
        // make that the new creep id index
        while creep_id_index < u32::MAX {
            creep_id_index += 1;

            if game_state.creeps.contains_key(&creep_id_index.to_string()) {
                continue;
            }

            game_state.creep_id_index = creep_id_index;
            return Ok(creep_id_index);
        }

        Err(GeneralResult::Fail)
    }
}
