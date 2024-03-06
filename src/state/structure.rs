use std::collections::HashMap;

use screeps::{ObjectId, Structure};

#[derive(Default)]
pub struct StructureState {
    // Should add logic to clear this list every 100 ticks or so.
    pub active_statuses: StructureActiveStatuses,
}

pub type StructureActiveStatuses = HashMap<ObjectId<Structure>, bool>;