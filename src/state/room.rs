use std::collections::HashMap;

use screeps::{find, Room, RoomName, StructureObject, StructureProperties, StructureType};

use crate::constants::structure::OrganizedStructures;

use super::game::GameState;

#[derive(Debug)]
pub struct RoomState {
    pub name: RoomName,
    pub structures: Option<OrganizedStructures>,
}

impl RoomState {
    pub fn new(room: &Room, room_name: RoomName) -> Self {
        Self {
            name: room_name,
            structures: Some(OrganizedStructures::default()),
        }
    }
}

pub struct CommunePlanner {
    grid_map: [u8; 2500],
    terrain_map: [u8; 2500],
    road_map: [u8; 2500],
    plan_map: [u8; 2500],
}

pub struct RemotePlanner {}

pub struct RoomStateOps;

impl RoomStateOps {

    pub fn update_state(state: &mut RoomState) {
        if let Some(organized_structures) = state.structures.as_mut() {
            organized_structures.clear();
        }
    }
}
