use std::collections::HashMap;

use screeps::RoomName;

use crate::constants::structure::OrganizedStructures;

#[derive(Default)]
pub struct RoomsState {
    pub structures: HashMap<RoomName, OrganizedStructures>
}

impl RoomsState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}