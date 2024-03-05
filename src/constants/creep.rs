use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub enum CreepRole {
    #[default]
    SourceHarvester,
    Hauler,
    Upgrader,
    Builder,
    Scout,
    Repairer,
    Antifa,
}

pub enum CreepOperationResult {
    Success,
    Fail,
    InProgress,
    Exception,
}
