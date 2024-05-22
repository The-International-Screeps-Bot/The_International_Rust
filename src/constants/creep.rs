use std::collections::HashMap;

use screeps::{BodyPart, Creep, Part};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub enum CreepRole {
    #[default]
    SourceHarvester,
    Hauler,
    Upgrader,
    Builder,
    Scout,
    Repairer,
    Antifa,
}

pub enum CreepOperationResult {
    Success,
    Fail,
    InProgress,
    Exception,
}

pub type CreepParts = Vec<Part>;
pub type CreepPartsByType = HashMap<Part, u32>;
pub type ActiveCreepPartsByType = HashMap<Part, u32>;