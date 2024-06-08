use std::collections::HashMap;

use screeps::{Room, RoomName};

use crate::constants::structure::SpawnsByActivity;

use super::game::GameState;

#[derive(Debug)]
pub struct CommuneState {
    pub name: RoomName,
    pub spawn_energy_capacity: u32,
    pub min_energy: Option<u32>,
    pub spawns_by_activity: Option<SpawnsByActivity>,
}

impl CommuneState {
    pub fn new(room: &Room, name: RoomName) -> Self {
        Self {
            name,
            spawn_energy_capacity: room.energy_capacity_available(),
            min_energy: Some(0),
            spawns_by_activity: None,
        }
    }

    pub fn tick_update(&mut self, room_name: &RoomName) {

        self.min_energy = None;
        self.spawns_by_activity = None;
    }
}