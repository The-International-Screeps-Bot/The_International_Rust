use std::collections::HashMap;

use screeps::RoomName;

use crate::constants::structure::OrganizedStructures;

#[derive(Debug, Default)]
pub struct RoomsState {
    pub structures: HashMap<RoomName, OrganizedStructures>,
}

impl RoomsState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

pub struct CommunePlanner {
    grid_map: [u8; 2500],
    terrain_map: [u8; 2500],
    road_map: [u8; 2500],
    plan_map: [u8; 2500],
}

pub struct RemotePlanner {

}