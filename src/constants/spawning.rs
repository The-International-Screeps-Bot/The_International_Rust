use super::creep::CreepRole;

pub struct SpawnRequest {
    pub role: CreepRole   
}

pub type SpawnRequests = Vec<SpawnRequest>;