use std::collections::HashMap;

use screeps::{ObjectId, Structure};

#[derive(Debug, Default)]
pub struct StructuresState {
    // Should add logic to clear this list every 100 ticks or so.
    pub active_statuses: StructureActiveStatuses,
}

impl StructuresState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

pub type StructureActiveStatuses = HashMap<ObjectId<Structure>, bool>;
