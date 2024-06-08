use std::collections::HashMap;

use enum_map::{Enum, EnumMap};
use screeps::{BodyPart, Creep, Part};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Enum, Debug)]
pub enum CreepRole {
    SourceHarvester,
    Hauler,
    Upgrader,
    Builder,
    Scout,
    Repairer,
    Antifa,
    Unknown,
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

#[derive(Debug, Enum, Copy, Clone)]
pub enum CreepPart {
    Move,
    Work,
    Carry,
    Attack,
    RangedAttack,
    Tough,
    Heal,
    Claim,
}

impl CreepPart {
    pub const fn cost(self) -> u32 {
        match self {
            Self::Move => 50,
            Self::Work => 100,
            Self::Carry => 50,
            Self::Attack => 80,
            Self::RangedAttack => 150,
            Self::Tough => 10,
            Self::Heal => 250,
            Self::Claim => 600,
        }
    }
}

pub type BodypartCounts = EnumMap<CreepPart, u32>;