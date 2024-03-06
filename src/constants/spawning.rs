use screeps::{BodyPart, SpawnOptions};

use crate::memory::creep_memory::CreepMemory;

use super::creep::CreepRole;

pub struct SpawnRequestArgs {
    pub constructorType: SpawnRequestTypes,
    pub role: CreepRole,
    pub defaultParts: Vec<BodyPart>,
    pub extraParts: Vec<BodyPart>, 
    pub partsQuota: u32,
    pub minCostPerCreep: u32,
    pub memory_additions: CreepMemory,
    pub priority: u32,
    pub max_creeps: u32,
    pub max_cost_per_creep: u32,
}

pub enum SpawnRequestTypes {
    IndividualUniform,
    GroupDiverse,
    GroupUniform,
}

pub struct SpawnRequest {
    pub role: CreepRole,   
    pub priority: u32,
    pub tier: u32,
    pub cost: u32,
    pub extraOpts: SpawnOptions,
}

pub type SpawnRequests = Vec<SpawnRequest>;