use std::collections::HashMap;

use screeps::{
    find, Room, RoomName, StructureContainer, StructureController, StructureFactory,
    StructureNuker, StructureObject, StructurePowerSpawn, StructureProperties, StructureStorage,
    StructureTerminal, StructureType,
};

use crate::constants::structure::OrganizedStructures;

use super::game::GameState;

#[derive(Debug)]
pub struct RoomState {
    pub name: RoomName,
    pub structures: Option<OrganizedStructures>,
    pub storage: Option<StructureStorage>,
    pub terminal: Option<StructureTerminal>,
    pub power_spawn: Option<StructurePowerSpawn>,
    pub controller: Option<StructureController>,
    pub nuker: Option<StructureNuker>,
    pub factory: Option<StructureFactory>,
}

impl RoomState {
    pub fn new(room: &Room, room_name: RoomName) -> Self {
        Self {
            name: room_name,
            structures: None,
            storage: None,
            terminal: None,
            power_spawn: None,
            controller: None,
            nuker: None,
            factory: None,
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
        state.structures = None;
        state.storage = None;
        state.terminal = None;
        state.power_spawn = None;
        state.controller = None;
        state.nuker = None;
        state.factory = None;
    }
}
