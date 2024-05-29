use std::{collections::HashMap, default};

use crate::{constants::creep::{ActiveCreepPartsByType, CreepParts, CreepPartsByType}, creep::owned_creep::OwnedCreep};
use screeps::{Creep, Position};

use super::game::GameState;

#[derive(Default, Debug)]
pub struct CreepState {
    pub name: String,
    pub cost: u32,
    pub parts: Option<CreepParts>,
    pub parts_by_type: Option<CreepPartsByType>,
    pub active_parts_by_type: Option<ActiveCreepPartsByType>,
    pub move_request: Option<Position>,
}

impl CreepState {
    pub fn new(creep: &OwnedCreep, name: String) -> Self {
        Self { name, cost: 0, ..Default::default() }
    }
}

pub struct CreepStateOps;

impl CreepStateOps {
    pub fn update_state(state: &mut CreepState) {}
}
