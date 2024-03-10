use std::collections::HashMap;

use crate::creep::owned_creep::OwnedCreep;
use screeps::Creep;

use super::game::GameState;

#[derive(Debug)]
pub struct CreepState {
    pub name: String,
    pub cost: u32,
}

impl CreepState {
    pub fn new(creep: &OwnedCreep, name: String) -> Self {
        Self { name, cost: 0 }
    }
}

pub struct CreepStateOps;

impl CreepStateOps {
    pub fn update_state(state: &mut CreepState) {}
}
