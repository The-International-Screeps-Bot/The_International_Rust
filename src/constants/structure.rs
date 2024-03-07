use std::collections::HashMap;

use screeps::{StructureObject, StructureType};

pub type OrganizedStructures = HashMap<StructureType, Vec<StructureObject>>;

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