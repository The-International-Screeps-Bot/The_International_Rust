use std::collections::HashMap;

use screeps::{Room, RoomName};

use super::game::GameState;

#[derive(Debug)]
pub struct CommuneState {
    pub name: RoomName,
    pub spawn_energy_capacity: u32,
    pub min_energy: u32,
}

impl CommuneState {
    pub fn new(room: &Room, name: RoomName) -> Self {
        Self {
            name,
            spawn_energy_capacity: room.energy_capacity_available(),
            min_energy: 0,
        }
    }
}

pub struct CommuneStateOps;

impl CommuneStateOps {

    pub fn update_state(state: &mut CommuneState) {

        
    }
}