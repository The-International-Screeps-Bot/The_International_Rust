use std::collections::HashMap;

use screeps::RoomName;

use crate::constants::structure::OrganizedStructures;

#[derive(Default)]
pub struct RoomState {
    pub structures: Option<OrganizedStructures>,
}

impl RoomState {
    pub fn new() -> Self {
        RoomState { ..Default::default() }
    }
}

pub type RoomStates = HashMap<RoomName, RoomState>;