use std::collections::HashMap;

use screeps::{StructureObject, StructureType};

pub type OrganizedStructures = HashMap<StructureType, Vec<StructureObject>>;