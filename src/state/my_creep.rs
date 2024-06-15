use std::collections::HashMap;

use enum_map::EnumMap;
use screeps::{HasPosition, Part, Position};

use crate::{constants::creep::{ActiveCreepPartsByType, CreepParts, CreepPartsByType}, creep::my_creep::MyCreep};

use super::game::GameState;

pub type MyCreepStates = HashMap<String, MyCreepState>;

#[derive(Debug)]
/// State for creeps we necessarily own
pub struct MyCreepState {
    pub cost: Option<u32>,
    /// The next position the creep intends to move to
    pub move_request: Option<Position>,
    /// The position which the creep is registered to move to or stay at
    pub move_target: Option<Position>,
    /// The coord for which the creep intends to action.
    /// A harvester's action_coord is where it wants to harvest at
    /// A invader core attacker's action_coord is the invader core's position
    /// An upgrader's action_coord is the controller's position
    pub action_pos: Option<Position>,
    pub parts: Option<Vec<Part>>,
    pub parts_by_type: Option<CreepPartsByType>,
    pub active_parts_by_type: Option<ActiveCreepPartsByType>,
}

impl MyCreepState {
    pub fn new(name: &str) -> Self {
        Self {
            cost: None,
            move_request: None,
            move_target: None,
            action_pos: None,
            parts: None,
            parts_by_type: None,
            active_parts_by_type: None,
        }
    }
}

pub struct MyCreepStateOps;

impl MyCreepStateOps {
    pub fn update_state(state: &mut MyCreepState) {
        state.move_request = None;
        state.action_pos = None;
        state.move_target = None;
    }
}
