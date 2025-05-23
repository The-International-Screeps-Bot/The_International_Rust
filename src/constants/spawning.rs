use std::default;

use enum_map::{Enum, EnumMap};
use screeps::{constants::creep::Part, BodyPart, Position, SpawnOptions};

use crate::memory::creep_memory::CreepMemory;

use super::creep::{CreepPart, CreepRole};

pub struct IndividualUniformSpawnRequestArgs {
    pub role: CreepRole,
    pub default_parts: Vec<CreepPart>,
    pub extra_parts: Vec<CreepPart>,
    pub parts_quota: u32,
    pub min_cost_per_creep: u32,
    pub max_cost_per_creep: u32,
    pub memory_additions: CreepMemory,
    pub priority: u32,
    pub creeps_quota: u32,
    pub spawn_target: Option<Position>,
}

pub struct GroupUniformSpawnRequestArgs {
    pub role: CreepRole,
    pub default_parts: Vec<CreepPart>,
    pub extra_parts: Vec<CreepPart>,
    pub parts_quota: u32,
    pub min_cost_per_creep: u32,
    pub max_cost_per_creep: Option<u32>,
    pub memory_additions: CreepMemory,
    pub priority: u32,
    pub max_creeps: Option<u32>,
    pub threshold: Option<f32>,
    pub spawn_target: Option<Position>,
}

pub struct GroupDiverseSpawnRequestArgs {
    pub role: CreepRole,
    pub default_parts: Vec<CreepPart>,
    pub extra_parts: Vec<CreepPart>,
    pub parts_quota: u32,
    pub min_cost_per_creep: u32,
    pub max_cost_per_creep: Option<u32>,
    pub memory_additions: CreepMemory,
    pub priority: u32,
    pub max_creeps: Option<u32>,
    pub threshold: Option<f32>,
    pub spawn_target: Option<Position>,
}

pub enum SpawnRequestArgs {
    IndividualUniform(IndividualUniformSpawnRequestArgs),
    GroupUniform(GroupUniformSpawnRequestArgs),
    GroupDiverse(GroupDiverseSpawnRequestArgs),
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
    pub memory: CreepMemory,
    pub body_part_counts: EnumMap<CreepPart, u32>,
    pub spawn_target: Option<Position>,
}

#[derive(Default)]
pub struct BodypartsByPriority {
    pub tough: u32,
    pub claim: u32,
    pub attack: u32,
    pub ranged_attack: u32,
    pub secondary_tough: u32,
    pub work: u32,
    pub carry: u32,
    pub move_part: u32,
    pub secondary_attack: u32,
    pub heal: u32,
}

impl BodypartsByPriority {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}