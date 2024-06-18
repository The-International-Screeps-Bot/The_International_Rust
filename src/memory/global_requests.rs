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

    pub fn is_abandoned(&self) -> bool {
        match self.abandon {
            None => true,
            Some(abandon) => {
                if abandon > 0 {
                    return true;
                }
    
                false
            }
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

    pub fn is_abandoned(&self) -> bool {
        match self.abandon {
            None => true,
            Some(abandon) => {
                if abandon > 0 {
                    return true;
                }
    
                false
            }
        }
    }
}

pub type ClaimRequests = HashMap<RoomName, ClaimRequest>;

#[derive(Serialize, Deserialize, Default)]
pub struct AttackRequest {
    pub responders: Option<RoomName>,
    pub abandon: Option<u32>,
}

impl AttackRequest {
    pub fn new() -> Self {
        Self {
            responders: None,
            abandon: None,
        }
    }

    pub fn is_abandoned(&self) -> bool {
        match self.abandon {
            None => true,
            Some(abandon) => {
                if abandon > 0 {
                    return true;
                }
    
                false
            }
        }
    }
}

pub type AttackRequests = HashMap<RoomName, AttackRequest>;

#[derive(Serialize, Deserialize, Default)]
pub struct DefenseRequest {
    pub responders: Option<RoomName>,
    pub abandon: Option<u32>,
}

impl DefenseRequest {
    pub fn new() -> Self {
        Self {
            responders: None,
            abandon: None,
        }
    }

    pub fn is_abandoned(&self) -> bool {
        match self.abandon {
            None => true,
            Some(abandon) => {
                if abandon > 0 {
                    return true;
                }
    
                false
            }
        }
    }
}

pub type DefenseRequests = HashMap<RoomName, DefenseRequest>;