use core::panic;
use std::collections::HashMap;

use enum_map::{enum_map, Enum, EnumMap};
use screeps::{BodyPart, Creep, Part, Position};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Enum, Debug)]
pub enum CreepRole {
    // Commune
    #[serde(rename = "0")]
    SourceHarvester,
    #[serde(rename = "1")]
    Hauler,
    #[serde(rename = "2")]
    Upgrader,
    #[serde(rename = "3")]
    Builder,
    #[serde(rename = "4")]
    MineralHarvester,
    #[serde(rename = "5")]
    Repairer,
    #[serde(rename = "6")]
    FastFill,
    #[serde(rename = "7")]
    Hub,
    // Remote
    #[serde(rename = "8")]
    RemoteHauler,
    #[serde(rename = "9")]
    RemoteSourceHarvester,
    #[serde(rename = "10")]
    RemoteMineralHarvester,
    #[serde(rename = "11")]
    RemoteReserver,
    #[serde(rename = "12")]
    RemoteBuilder,
    // Global
    #[serde(rename = "13")]
    Scout,
    #[serde(rename = "14")]
    Claimer,
    #[serde(rename = "15")]
    Vanguard,
    // Includes dismantlers, melee attackers, healers, ranged attackers (and defenders)
    #[serde(rename = "16")]
    Antifa,
    #[serde(rename = "17")]
    Downgraders,
    // Other
    #[serde(rename = "18")]
    Unknown,
}

pub enum CreepOperationResult {
    Success,
    Fail,
    InProgress,
    Exception,
}

pub type CreepParts = Vec<CreepPart>;
pub type CreepPartsByType = EnumMap<CreepPart, u32>;

thread_local! {
    pub static CREEP_PARTS_BY_TYPE: CreepPartsByType = enum_map! {
        CreepPart::Move => 0,
        CreepPart::Work => 0,
        CreepPart::Carry => 0,
        CreepPart::Attack => 0,
        CreepPart::RangedAttack => 0,
        CreepPart::Tough => 0,
        CreepPart::Heal => 0,
        CreepPart::Claim => 0,
    };
}

pub type ActiveCreepPartsByType = HashMap<CreepPart, u32>;

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

    pub const fn to_part(&self) -> Part {
        match self {
            Self::Move => Part::Move,
            Self::Work => Part::Work,
            Self::Carry => Part::Carry,
            Self::Attack => Part::Attack,
            Self::RangedAttack => Part::RangedAttack,
            Self::Tough => Part::Tough,
            Self::Heal => Part::Heal,
            Self::Claim => Part::Claim,
        }
    }

    pub fn from_part(part: &Part) -> CreepPart {
        match part {
            Part::Move => CreepPart::Move,
            Part::Work => CreepPart::Work,
            Part::Carry => CreepPart::Carry,
            Part::Attack => CreepPart::Attack,
            Part::RangedAttack => CreepPart::RangedAttack,
            Part::Tough => CreepPart::Tough,
            Part::Heal => CreepPart::Heal,
            Part::Claim => CreepPart::Claim,
            _ => panic!("Unknown part: {:?}", part),
        }
    }
}

pub type BodypartCounts = EnumMap<CreepPart, u32>;

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq)]
pub enum PriorityCreepPart {
    Tough,
    Claim,
    Attack,
    RangedAttack,
    SecondaryTough,
    Work,
    Carry,
    Move,
    SecondaryAttack,
    Heal,
}

pub const PARTS_BY_PRIORITY: [PriorityCreepPart; 10] = [
    PriorityCreepPart::Tough,
    PriorityCreepPart::Claim,
    PriorityCreepPart::Attack,
    PriorityCreepPart::RangedAttack,
    PriorityCreepPart::SecondaryTough,
    PriorityCreepPart::Work,
    PriorityCreepPart::Carry,
    PriorityCreepPart::Move,
    PriorityCreepPart::SecondaryAttack,
    PriorityCreepPart::Heal,
];

thread_local! {
    pub static PARTS_BY_PRIORITY_PART: EnumMap<PriorityCreepPart, CreepPart> = enum_map! {
        PriorityCreepPart::Tough => CreepPart::Tough,
        PriorityCreepPart::Claim => CreepPart::Claim,
        PriorityCreepPart::Attack => CreepPart::Attack,
        PriorityCreepPart::RangedAttack => CreepPart::RangedAttack,
        PriorityCreepPart::SecondaryTough => CreepPart::Tough,
        PriorityCreepPart::Work => CreepPart::Work,
        PriorityCreepPart::Carry => CreepPart::Carry,
        PriorityCreepPart::Move => CreepPart::Move,
        PriorityCreepPart::SecondaryAttack => CreepPart::Attack,
        PriorityCreepPart::Heal => CreepPart::Heal,
    };
}

pub type MoveTargets = HashMap<Position, String>;