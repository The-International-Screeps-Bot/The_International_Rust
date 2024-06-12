use std::collections::{HashMap, HashSet};

use screeps::RoomName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct WorkRequest {
    /// May want to have multiple responders in the future
    pub responders: Option<HashSet<RoomName>>,
    /// how many persisent work parts are requested for the job
    pub work_strength: Option<u32>,
    /// How many ticks to abandon the request for
    pub abandon: Option<u32>,
}

impl WorkRequest {
    pub fn new() -> Self {
        Self {
            responders: None,
            work_strength: None,
            abandon: None,
        }
    }
}

pub type WorkRequests = HashMap<RoomName, WorkRequest>;

#[derive(Serialize, Deserialize, Default)]
pub struct ClaimRequest {
    pub responder: Option<RoomName>,
    /// How many ticks to abandon the request for
    pub abandon: Option<u32>,
    pub claimer: Option<bool>,
}

impl ClaimRequest {
    pub fn new() -> Self {
        Self {
            responder: None,
            abandon: None,
            claimer: None,
        }
    }
}

pub type ClaimRequests = HashMap<RoomName, ClaimRequest>;

#[derive(Serialize, Deserialize, Default)]
pub struct CombatRequest {
    pub responders: Option<RoomName>,
    pub abandon: Option<u32>,
}

impl CombatRequest {
    pub fn new() -> Self {
        Self {
            responders: None,
            abandon: None,
        }
    }
}

pub type CombatRequests = HashMap<RoomName, CombatRequest>;