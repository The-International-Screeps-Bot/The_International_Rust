use std::collections::HashMap;

use enum_map::Enum;
use log::error;
use screeps::{
    HasStore, StructureContainer, StructureController, StructureExtension, StructureExtractor,
    StructureFactory, StructureInvaderCore, StructureKeeperLair, StructureLab, StructureLink,
    StructureNuker, StructureObject, StructureObserver, StructurePortal, StructurePowerBank,
    StructurePowerSpawn, StructureRampart, StructureRoad, StructureSpawn, StructureStorage,
    StructureTerminal, StructureTower, StructureType, StructureWall,
};

pub type OldOrganizedStructures = HashMap<StructureType, Vec<StructureObject>>;

pub const IMPASSIBLE_STRUCTURES: [StructureType; 8] = [
    StructureType::Spawn,
    StructureType::Extension,
    StructureType::Storage,
    StructureType::Terminal,
    StructureType::Factory,
    StructureType::PowerSpawn,
    StructureType::Nuker,
    StructureType::Link,
    // more
];

#[derive(Enum, Debug)]
pub enum CustomStructureType {
    Spawn,
    Extension,
    Road,
    Wall,
    Rampart,
    KeeperLair,
    Portal,
    Controller,
    Link,
    Storage,
    Tower,
    Observer,
    PowerBank,
    PowerSpawn,
    Extractor,
    Lab,
    Terminal,
    Container,
    Nuker,
    Factory,
    InvaderCore,
}

impl CustomStructureType {
    pub fn as_structure_type(&self) -> StructureType {
        match self {
            Self::Spawn => StructureType::Spawn,
            Self::Extension => StructureType::Extension,
            Self::Road => StructureType::Road,
            Self::Wall => StructureType::Wall,
            Self::Rampart => StructureType::Rampart,
            Self::KeeperLair => StructureType::KeeperLair,
            Self::Portal => StructureType::Portal,
            Self::Controller => StructureType::Controller,
            Self::Link => StructureType::Link,
            Self::Storage => StructureType::Storage,
            Self::Tower => StructureType::Tower,
            Self::Observer => StructureType::Observer,
            Self::PowerBank => StructureType::PowerBank,
            Self::PowerSpawn => StructureType::PowerSpawn,
            Self::Extractor => StructureType::Extractor,
            Self::Lab => StructureType::Lab,
            Self::Terminal => StructureType::Terminal,
            Self::Container => StructureType::Container,
            Self::Nuker => StructureType::Nuker,
            Self::Factory => StructureType::Factory,
            Self::InvaderCore => StructureType::InvaderCore,
        }
    }

    pub fn from_structure_type(structure_type: StructureType) -> Self {
        match structure_type {
            StructureType::Spawn => Self::Spawn,
            StructureType::Extension => Self::Extension,
            StructureType::Road => Self::Road,
            StructureType::Wall => Self::Wall,
            StructureType::Rampart => Self::Rampart,
            StructureType::KeeperLair => Self::KeeperLair,
            StructureType::Portal => Self::Portal,
            StructureType::Controller => Self::Controller,
            StructureType::Link => Self::Link,
            StructureType::Storage => Self::Storage,
            StructureType::Tower => Self::Tower,
            StructureType::Observer => Self::Observer,
            StructureType::PowerBank => Self::PowerBank,
            StructureType::PowerSpawn => Self::PowerSpawn,
            StructureType::Extractor => Self::Extractor,
            StructureType::Lab => Self::Lab,
            StructureType::Terminal => Self::Terminal,
            StructureType::Container => Self::Container,
            StructureType::Nuker => Self::Nuker,
            StructureType::Factory => Self::Factory,
            StructureType::InvaderCore => Self::InvaderCore,
            _ => panic!("Unknown structure type: {:?}", structure_type),
        }
    }
}

#[derive(Debug, Default)]
pub struct OrganizedStructures {
    pub spawn: Vec<StructureSpawn>,
    pub extension: Vec<StructureExtension>,
    pub road: Vec<StructureRoad>,
    pub wall: Vec<StructureWall>,
    pub rampart: Vec<StructureRampart>,
    pub keeper_lair: Vec<StructureKeeperLair>,
    pub portal: Vec<StructurePortal>,
    pub link: Vec<StructureLink>,
    pub tower: Vec<StructureTower>,
    pub observer: Vec<StructureObserver>,
    pub power_bank: Vec<StructurePowerBank>,
    pub lab: Vec<StructureLab>,
    pub container: Vec<StructureContainer>,
    pub invader_core: Vec<StructureInvaderCore>,
    pub power_spawn: Vec<StructurePowerSpawn>,
    pub factory: Vec<StructureFactory>,
    pub nuker: Vec<StructureNuker>,
}

#[derive(Debug, Clone)]
pub struct SpawnsByActivity {
    pub active: Vec<StructureSpawn>,
    pub inactive: Vec<StructureSpawn>,
}

impl SpawnsByActivity {
    pub fn new() -> Self {
        Self {
            active: Vec::new(),
            inactive: Vec::new(),
        }
    }
}

pub enum SpawningStructure {
    StructureSpawn,
    StructureExtension,
}
