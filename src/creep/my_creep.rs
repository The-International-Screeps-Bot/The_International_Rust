use screeps::{Creep, HasPosition, SharedCreepProperties};

use crate::state::{game::GameState, my_creep::MyCreepState};

use super::creep_functions::CreepFunctions;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct MyCreep(Creep);

impl MyCreep {
    pub fn new(creep: &Creep) -> Result<Self, ()> {
        if creep.my() {
            Ok(Self(creep.clone()))
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn inner(&self) -> &Creep {
        &self.0
    }

    #[inline]
    pub fn state<'a>(&self, creep_name: &String, game_state: &'a mut GameState) -> &'a mut MyCreepState {
        game_state.my_creep_states.get_mut(creep_name).unwrap()
    }
}
