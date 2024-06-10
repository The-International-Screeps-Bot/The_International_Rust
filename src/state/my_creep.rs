use std::collections::HashMap;

use enum_map::EnumMap;
use screeps::{HasPosition, Position};

use crate::creep::my_creep::MyCreep;

pub type MyCreepStates = HashMap<String, MyCreepState>;

#[derive(Debug)]
/// State for creeps we necessarily own
pub struct MyCreepState {
    /// The next position the creep intends to move to
    pub move_request: Option<Position>,
    /// The position which the creep is registered to move to or stay at
    pub move_target: Option<Position>,
    /// The coord for which the creep intends to action.
    /// A harvester's action_coord is where it wants to harvest at
    /// A invader core attacker's action_coord is the invader core's position
    /// An upgrader's action_coord is the controller's position
    pub action_coord: Option<Position>,
}

impl MyCreepState {
    pub fn new(creep: &MyCreep, name: &str) -> Self {
        Self {
            move_request: None,
            move_target: None,
            action_coord: None,
        }
    }
}

pub struct MyCreepStateOps;

impl MyCreepStateOps {
    pub fn update_state(state: &mut MyCreepState) {
        state.move_request = None;
        state.action_coord = None;
        state.move_target = None;
    }
}
