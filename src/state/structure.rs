use std::collections::HashMap;

use screeps::{ObjectId, Position, Structure};

#[derive(Debug)]
pub struct StructuresState {
    // Should add logic to clear this list every 100 ticks or so.
    pub active_statuses: StructureActiveStatuses,
}

impl StructuresState {
    pub fn new() -> Self {
        Self {
            active_statuses: HashMap::new(),
        }
    }
}

pub type StructureActiveStatuses = HashMap<ObjectId<Structure>, bool>;