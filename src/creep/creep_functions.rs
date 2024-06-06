use screeps::{Creep, SharedCreepProperties};

use crate::state::{creep::CreepState, game::GameState};

pub trait CreepFunctions: SharedCreepProperties {
    /* fn state<'a>(&self, creep_name: &String, game_state: &'a mut GameState) -> &'a mut CreepState {} */
}

impl CreepFunctions for Creep {
    /* fn state<'a>(&self, creep_name: &String, game_state: &'a mut GameState) -> &'a mut CreepState {
        if let Some(creep_state) = game_state.creep_states.get_mut(creep_name) {
            return creep_state;
        }

        let mut creep_state = CreepState::new(self, creep_name);

        game_state
            .creep_states
            .insert(creep_name.clone(), creep_state)
            .unwrap();

        &mut creep_state
    } */
}
