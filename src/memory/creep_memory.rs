use serde::{Deserialize, Serialize};

use crate::constants::creep::CreepRole;

#[derive(Serialize, Deserialize, Default)]
pub struct CreepMemory {
    #[serde(rename = "0")]
    pub role: CreepRole
}

#[derive(Serialize, Deserialize, Default)]
pub struct PowerCreepMemory {

}