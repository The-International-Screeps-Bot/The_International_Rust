use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::constants::segments::Command;

#[derive(Debug)]
pub struct SimpleAlliesSegment {
    pub my_segment_data: SegmentData,
    pub current_ally: Option<String>,
    pub ally_econ_info: Option<EconInfo>,
    pub ally_requests: Option<Requests>,
    pub last_updated: Option<u32>,
}

impl SimpleAlliesSegment {
    pub fn new() -> Self {
        Self {
            current_ally: None,
            ally_econ_info: None,
            ally_requests: None,
            last_updated: None,
            my_segment_data: SegmentData::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Requests {

}

impl Requests {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EconInfo {

}

impl EconInfo {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentData {
    pub requests: Requests,
    pub econ_info: Option<EconInfo>,
    pub commands: Option<HashMap<String, Command>>,
}

impl SegmentData {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
            econ_info: None,
            commands: None,
        }
    }
}