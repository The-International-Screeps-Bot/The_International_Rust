use std::collections::HashMap;

use screeps::{source, Position, Room, RoomName};

use crate::{constants::structure::SpawnsByActivity, memory::game_memory::GameMemory};

use super::game::GameState;

#[derive(Debug)]
pub struct CommuneState {
    pub name: RoomName,
    pub spawn_energy_capacity: u32,
    pub min_energy: Option<u32>,
    pub spawns_by_activity: Option<SpawnsByActivity>,
    pub upgrade_strength: u32,
    pub build_strength: u32,
    pub repair_strength: u32,
    pub haul_strength: u32,
    pub mineral_harvest_strength: u32,
    pub used_mineral_positions: Vec<Position>,
    pub source_harvest_strengths: Vec<u32>,
    pub used_source_harvest_positions: Vec<Vec<Position>>,
}

impl CommuneState {
    pub fn new(room: &Room, room_name: RoomName, memory: &GameMemory) -> Self {

        let commune_memory = memory.communes.get(&room_name).unwrap();

        // source harvest positions are found by the commune planner
        // mineral harvest positions are found by the commune planner

        let mut source_harvest_strengths: Vec<u32> = Vec::new();

        for i in 0..commune_memory.source_positions.len() {
            source_harvest_strengths.push(0);
        }

        Self {
            name: room_name,
            spawn_energy_capacity: room.energy_capacity_available(),
            min_energy: Some(0),
            spawns_by_activity: None,
            upgrade_strength: 0,
            build_strength: 0,
            repair_strength: 0,
            haul_strength: 0,
            mineral_harvest_strength: 0,
            used_mineral_positions: Vec::new(),
            source_harvest_strengths,
            used_source_harvest_positions: Vec::new(),
        }
    }

    pub fn tick_update(&mut self, room_name: &RoomName) {

        self.min_energy = None;
        self.spawns_by_activity = None;
    }
}