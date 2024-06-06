use std::{collections::HashMap, default};

use crate::{
    constants::creep::{ActiveCreepPartsByType, CreepParts, CreepPartsByType},
    creep::my_creep::MyCreep,
};
use screeps::{Creep, HasPosition, Position, SharedCreepProperties};

use super::game::GameState;

#[derive(Debug)]
/// State for creeps we either incedentally own or do not own
pub struct CreepState {
    pub name: String,
    pub pos: Option<Position>,
    pub cost: Option<u32>,
    /// Cached part's of the creep, not accounting for health state
    pub parts: Option<CreepParts>,
    pub parts_by_type: Option<CreepPartsByType>,
    pub active_parts_by_type: Option<ActiveCreepPartsByType>,
}

impl CreepState {
    pub fn new(creep: &Creep, name: &str) -> Self {
        Self {
            name: name.to_string(),
            pos: None,
            cost: None,
            parts: None,
            parts_by_type: None,
            active_parts_by_type: None,
        }
    }
}

pub struct CreepStateOps;

impl CreepStateOps {
    pub fn update_state(state: &mut CreepState) {
        state.pos = None;
        state.active_parts_by_type = None;
    }
}
