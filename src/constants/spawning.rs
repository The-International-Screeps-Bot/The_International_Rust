use std::default;

use screeps::{BodyPart, SpawnOptions};

use crate::memory::creep_memory::CreepMemory;

use super::creep::CreepRole;

pub struct SpawnRequestArgs {
    pub constructor_type: SpawnRequestTypes,
    pub role: CreepRole,
    pub default_parts: Vec<BodyPart>,
    pub extra_parts: Vec<BodyPart>, 
    pub parts_quota: u32,
    pub min_cost_per_creep: u32,
    pub memory_additions: CreepMemory,
    pub priority: u32,
    pub max_creeps: u32,
    pub max_cost_per_creep: u32,
}

#[derive(Debug, Default)]
pub enum SpawnRequestTypes {
    #[default]
    IndividualUniform,
    GroupDiverse,
    GroupUniform,
}

pub struct SpawnRequest {
    pub role: CreepRole,   
    pub priority: u32,
    pub tier: u32,
    pub cost: u32,
    pub extra_opts: SpawnOptions,
}

pub type SpawnRequests = Vec<SpawnRequest>;