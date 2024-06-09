use std::{
    default,
    fmt::{self, Debug},
};

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
    pub priority: f32,
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
    pub priority: f32,
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
    pub priority: f32,
    pub max_creeps: Option<u32>,
    pub threshold: Option<f32>,
    pub spawn_target: Option<Position>,
}

pub enum SpawnRequestArgs {
    IndividualUniform(IndividualUniformSpawnRequestArgs),
    GroupUniform(GroupUniformSpawnRequestArgs),
    GroupDiverse(GroupDiverseSpawnRequestArgs),
}

impl Debug for SpawnRequestArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let role = match self {
            SpawnRequestArgs::IndividualUniform(args) => args.role,
            SpawnRequestArgs::GroupUniform(args) => args.role,
            SpawnRequestArgs::GroupDiverse(args) => args.role,
        };

        write!(
            f,
            "{:?}",
            role,
        )
    }
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
    pub priority: f32,
    pub tier: u32,
    pub cost: u32,
    pub memory: CreepMemory,
    pub body_part_counts: EnumMap<CreepPart, u32>,
    pub spawn_target: Option<Position>,
}

impl Debug for SpawnRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let role = self.role;

        write!(
            f,
            "{:?}",
            role,
        )
    }
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

pub mod spawn_priority_bounds {
    pub const SOURCE_HARVESTER: (f32, f32) = (2.0, 1000.0);
    pub const HAULER: (f32, f32) = (1.0, 1000.0);
    // 0-1 as a percent of repair need fulfilled. Perhaps repair need increases as ramparts get closer to being super low
    pub const URGENT_REPAIRER: (f32, f32) = (3.0, 4.0);
    // 0-1 as a percent of damage need fulfilled
    pub const DEFENDER: (f32, f32) = (3.0, 4.0);

    // Remote room
    // Each remote room adds + 1 priority
    pub const REMOTE_RESERVER: (f32, f32) = (10.0, 1000.0);
    pub const REMOTE_BUILDER: (f32, f32) = (10.2, 1000.0);

    // Remote source
    // Each remote source adds +1 priority

    pub const REMOTE_SOURCE_HARVESTER: (f32, f32) = (10.1, 1000.0);
    // Source harvester + .1
    pub const REMOTE_HAULER: (f32, f32) = (10.3, 1000.0);

    // Still need to figure these out. Ideally we spawn them in between efficiency peaks. As in, when we can spawn very efficient creeps don't spawn these guys, otherwise spawn them
    // Determine how much repair, upgrade, build combat and scout spawn time we need
    // Compare to predictions of spawn time for remoting and other needs
    // Cut down on remotes until we have sufficient spawn time for these creeps
    // Then, when we would have spent spawn time on the less efficient remotes, we instead spawn these guys

    pub const NORMAL_REPAIRER: (f32, f32) = (0.0, 1000.0);
    pub const UPGRADER: (f32, f32) = (0.0, 1000.0);
    pub const BUILDER: (f32, f32) = (0.0, 1000.0);
    // Give more spawn time the more RCL weighted hate we have
    pub const ANTIFA: (f32, f32) = (0.0, 1000.0);
    pub const SCOUT: (f32, f32) = (0.0, 1000.0);
}
