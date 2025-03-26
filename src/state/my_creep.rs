use std::collections::HashMap;

use enum_map::EnumMap;
use screeps::{Creep, HasPosition, Part, Position, Spawning};

use crate::{
    constants::creep::{ActiveCreepPartsByType, CreepParts, CreepPartsByType},
    creep::my_creep::MyCreep,
};

use super::game::GameState;

pub type MyCreepStates = HashMap<String, MyCreepState>;

#[derive(Debug)]
/// State for creeps we necessarily own
pub struct MyCreepState {
    pub cost: Option<u32>,
    pub spawning: bool,
    pub fatigue: u32,
    pub pos: Position,
    /// The next position the creep intends to move to
    pub move_request: Option<Position>,
    pub move_options: Option<Vec<Position>>,
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
    pub fn new(name: &str, creep: &Creep) -> Self {
        Self {
            cost: None,
            spawning: creep.spawning(),
            fatigue: creep.fatigue(),
            pos: creep.pos(),
            move_request: None,
            move_options: None,
            action_pos: None,
            parts: None,
            parts_by_type: None,
            active_parts_by_type: None,
        }
    }

    pub fn tick_update(&mut self, creep: &MyCreep) {
        self.spawning = creep.inner().spawning();
        self.fatigue = creep.inner().fatigue();
        self.pos = creep.inner().pos();

        self.move_request = None;
        self.move_options = None;

        self.active_parts_by_type = None;
    }

    pub fn interval_update(&mut self) {

        self.parts = None;
        self.parts_by_type = None;
    }
}
