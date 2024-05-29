use std::collections::HashMap;

use screeps::{
    find, ObjectId, Path, Position, Room, RoomName, Source, StructureContainer,
    StructureController, StructureFactory, StructureNuker, StructureObject, StructurePowerSpawn,
    StructureProperties, StructureStorage, StructureTerminal, StructureType,
};

use crate::constants::{room::NotMyCreeps, structure::{OrganizedStructures, SpawnsByActivity}};

use super::game::GameState;

#[derive(Debug)]
pub struct RoomState {
    pub name: RoomName,

    // Structures
    pub structures: Option<OrganizedStructures>,
    pub storage: Option<StructureStorage>,
    pub terminal: Option<StructureTerminal>,
    pub power_spawn: Option<StructurePowerSpawn>,
    pub controller: Option<StructureController>,
    pub nuker: Option<StructureNuker>,
    pub factory: Option<StructureFactory>,
    pub commune_plan: Option<CommunePlan>,
    pub spawns_by_activity: Option<SpawnsByActivity>,

    // Sources
    pub sources: Option<Vec<Source>>,
    pub harvest_positions: Option<Vec<Position>>,

    // Creeps
    pub not_my_creeps: Option<NotMyCreeps>,
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
            commune_plan: None,
            sources: None,
            harvest_positions: None,
            not_my_creeps: None,
            spawns_by_activity: None,
        }
    }
}

// All of the data associated with a commune's base plan
#[derive(Debug)]
pub struct CommunePlan {
    pub grid_map: [u8; 2500],
    pub terrain_map: [u8; 2500],
    pub road_map: [u8; 2500],
    pub plan_map: [u8; 2500],
    pub plan_attempts: Vec<CommunePlanAttemptSummary>,
    pub current_attempt: CommunePlanAttemptData,
    /// FastFiller
    pub fast_filler_start_positions: Option<Vec<Position>>,
}

impl CommunePlan {
    pub fn new() -> Self {
        Self {
            grid_map: [0; 2500],
            terrain_map: [0; 2500],
            road_map: [0; 2500],
            plan_map: [0; 2500],
            plan_attempts: Vec::new(),
            current_attempt: CommunePlanAttemptData::new(),
            fast_filler_start_positions: None,
        }
    }
}

/// Data required to complete a plan attempt and inform a summary of it
#[derive(Debug)]
pub struct CommunePlanAttemptData {
    /// Upgrade
    pub center_upgrade_pos: Option<Position>,
    pub upgrade_path: Option<Vec<Position>>,
    /// Lab
    pub input_lab_2_pos: Option<Position>,
    pub output_lab_positions: Option<Vec<Position>>,
    /// Source
    pub source_harvest_positions: Option<Vec<Position>>,
    pub source_paths: Option<Vec<Vec<Position>>>,
    pub source_structure_positions: Option<Vec<Position>>,
    pub commune_sources: Option<Vec<Source>>,
    pub unprotected_sources: Option<u32>,
    /// Mineral
    pub mineral_path: Option<Vec<Position>>,
    pub mineral_harvest_positions: Option<Vec<Position>>,
    /// Controller
    pub is_controller_protected: bool,
    /// Progress
    pub planned_grid_coords: bool,
    pub finished_grid: bool,
    pub general_shielded: bool,
    pub finished_fast_filler_road_prune: bool,
    /// If the planner is in the process of recording a plan attempt
    pub recording: bool,
    pub mark_sources_avoid: bool,
    pub finished_tower_paths: bool,
    pub plan_configed: bool,
}

impl CommunePlanAttemptData {
    pub fn new() -> Self {
        Self {
            center_upgrade_pos: None,
            upgrade_path: None,
            input_lab_2_pos: None,
            output_lab_positions: None,
            source_harvest_positions: None,
            source_paths: None,
            source_structure_positions: None,
            commune_sources: None,
            unprotected_sources: None,
            mineral_path: None,
            mineral_harvest_positions: None,
            is_controller_protected: false,
            planned_grid_coords: false,
            finished_grid: false,
            general_shielded: false,
            finished_fast_filler_road_prune: false,
            recording: false,
            mark_sources_avoid: false,
            finished_tower_paths: false,
            plan_configed: false,
        }
    }
}

/// Derived from a completed plan attempt
#[derive(Debug)]
pub struct CommunePlanAttemptSummary {
    score: u32,
    stamp_anchors: f64,
    base_plans: String,
    rampart_plans: String,
    road_quota: Vec<u32>,
    commune_sources: ObjectId<Source>,
    source_harvest_positions: Vec<Position>,
    source_paths: Vec<Vec<Position>>,
    mineral_harvest_positions: Vec<Position>,
    mineral_path: Vec<Position>,
    center_upgrade_pos: Position,
    upgrade_path: Vec<Position>,
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
