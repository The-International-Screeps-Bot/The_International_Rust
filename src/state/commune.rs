use std::collections::{HashMap, HashSet};

use screeps::{source, Position, Room, RoomName};
use screeps_utils::sparse_cost_matrix::SparseCostMatrix;

use crate::{constants::structure::SpawnsByActivity, memory::game_memory::GameMemory};

use super::game::GameState;

#[derive(Debug)]
pub struct CommuneState {
    pub name: RoomName,
    pub spawn_energy_capacity: u32,
    pub rcl: u8,
    /// Number of carry parts worth of hauling the commune wants
    pub haul_need: u32,
    pub min_energy: Option<u32>,
    pub spawns_by_activity: Option<SpawnsByActivity>,
    pub upgrade_strength: u32,
    pub build_strength: u32,
    pub repair_strength: u32,
    /// Number of carry parts worth of hauling the commune appears to have
    pub haul_strength: u32,
    pub mineral_harvest_strength: u32,
    pub source_harvest_strengths: Vec<u32>,
    pub source_harvest_creeps: Vec<u8>,
    pub structure_plans: SparseCostMatrix,
    pub rampart_plans: SparseCostMatrix,
    pub planning_completed: bool,
    pub reserved_positions: HashSet<Position>,
}

impl CommuneState {
    pub fn new(room_name: RoomName, game_state: &GameState, memory: &GameMemory) -> Self {
        // source harvest positions are found by the commune planner
        // mineral harvest positions are found by the commune planner

        let mut source_harvest_strengths: Vec<u32> = Vec::new();
        let mut source_harvest_creeps: Vec<u8> = Vec::new();

        if let Some(harvestable_room_memory) = memory.harvestable_rooms.get(&room_name) {
            for i in 0..harvestable_room_memory.source_positions.len() {
                source_harvest_strengths.push(0);
                source_harvest_creeps.push(0);
            }
        }

        let room = game_state.rooms.get(&room_name).unwrap();

        let controller = room.controller().unwrap();
        let rcl = controller.level();

        Self {
            name: room_name,
            spawn_energy_capacity: room.energy_capacity_available(),
            rcl,
            haul_need: 0,
            min_energy: Some(0),
            spawns_by_activity: None,
            upgrade_strength: 0,
            build_strength: 0,
            repair_strength: 0,
            haul_strength: 0,
            mineral_harvest_strength: 0,
            source_harvest_strengths,
            source_harvest_creeps,
            // Derive from pre-existing plans if they exist
            structure_plans: SparseCostMatrix::new(),
            rampart_plans: SparseCostMatrix::new(),
            planning_completed: false,
            reserved_positions: HashSet::new(),
        }
    }
    
    pub fn tick_update(&mut self, room_name: &RoomName) {
        self.spawns_by_activity = None;
    }

    pub fn interval_update(&mut self, room_name: &RoomName) {
        self.min_energy = None;
    }
}
