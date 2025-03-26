use std::collections::HashMap;

use enum_map::{EnumMap, enum_map};
use screeps::{
    ConstructionSite, LocalRoomTerrain, ObjectId, Path, Position, Room, RoomName, Source,
    Structure, StructureContainer, StructureController, StructureFactory, StructureNuker,
    StructureObject, StructurePowerSpawn, StructureProperties, StructureStorage, StructureTerminal,
    StructureType, find, game::map::RoomStatus,
};
use screeps_utils::sparse_cost_matrix::SparseCostMatrix;

use crate::{
    constants::{
        creep::CreepRole,
        general::{GeneralError, GeneralResult},
        room::{NO_VISION_STATE_EXPIRATION, NotMyCreeps},
        structure::{OrganizedStructures, SpawnsByActivity},
    },
    creep::my_creep::MyCreep,
};

use super::game::GameState;

pub type RoomStates = HashMap<RoomName, RoomState>;

#[derive(Debug)]
pub struct RoomState {
    pub name: RoomName,
    pub status: Option<RoomStatus>,
    pub terrain: Option<LocalRoomTerrain>,
    pub sparse_terrain: Option<SparseCostMatrix>,
    pub default_move_ops: Option<SparseCostMatrix>,
    pub enemy_threat_positions: Option<SparseCostMatrix>,
    pub last_seen: u32,
    pub expired: bool,

    // Structures
    pub structures: Option<Vec<StructureObject>>,
    pub structures_by_type: Option<OrganizedStructures>,
    pub storage: Option<StructureStorage>,
    pub terminal: Option<StructureTerminal>,
    pub controller: Option<StructureController>,

    pub my_construction_sites: Option<Vec<ConstructionSite>>,
    pub not_my_construction_sites: Option<NotMyConstructionSites>,
    pub commune_plan: Option<CommunePlan>,

    // Sources
    pub sources: Option<Vec<Source>>,
    pub harvest_positions: Option<Vec<Vec<Position>>>,

    // Creeps
    pub my_creeps: Vec<String>,
    pub creeps_by_role: EnumMap<CreepRole, Vec<String>>,
    pub not_my_creeps: Option<NotMyCreeps>,
}

impl RoomState {
    pub fn new(room_name: RoomName, game_state: &GameState) -> Self {
        Self {
            name: room_name,
            status: None,
            terrain: None,
            sparse_terrain: None,
            default_move_ops: None,
            enemy_threat_positions: None,
            last_seen: game_state.tick,
            expired: false,
            structures: None,
            structures_by_type: None,
            storage: None,
            terminal: None,
            controller: None,
            my_construction_sites: None,
            not_my_construction_sites: None,
            commune_plan: None,
            sources: None,
            harvest_positions: None,
            my_creeps: Vec::new(),
            not_my_creeps: None,
            creeps_by_role: creeps_by_role(),
        }
    }

    /// Track when the room was last seen.
    /// If the room hasn't been seen for awhile, remove it
    /// Otherwise if we do see the room, record the fact (reset timer)
    pub fn track_vision(&mut self, has_vision: bool, tick: u32) {
        if has_vision {
            self.last_seen = tick;
            return;
        }

        // We don't have vision

        // If the room hasn't been seen for awhile, record that it is expired
        if tick - self.last_seen >= NO_VISION_STATE_EXPIRATION {
            self.expired = true;
        }
    }

    pub fn tick_update(&mut self, room_name: &RoomName) {
        self.my_construction_sites = None;
        self.not_my_construction_sites = None;

        self.my_creeps = Vec::new();
        self.creeps_by_role = creeps_by_role();
        self.not_my_creeps = None;

        self.structures = None;
        self.structures_by_type = None;
        self.storage = None;
        self.terminal = None;
        self.controller = None;

        self.enemy_threat_positions = None;
    }

    pub fn interval_update(&mut self, room_name: &RoomName) {
        self.terrain = None;
        self.sparse_terrain = None;
        self.default_move_ops = None;
    }
}

#[derive(Debug)]
pub struct NotMyConstructionSites {
    pub ally: Vec<ConstructionSite>,
    pub enemy: Vec<ConstructionSite>,
}

impl NotMyConstructionSites {
    pub fn new() -> Self {
        Self {
            ally: Vec::new(),
            enemy: Vec::new(),
        }
    }
}

// All of the data associated with a commune's base plan
#[derive(Debug)]
pub struct CommunePlan {
    pub grid_map: SparseCostMatrix,
    pub terrain_map: [u8; 2500],
    pub road_map: SparseCostMatrix,
    pub plan_map: SparseCostMatrix,
    pub plan_attempts: Vec<CommunePlanAttemptSummary>,
    pub current_attempt: CommunePlanAttemptData,
    /// FastFiller
    pub fast_filler_start_positions: Option<Vec<Position>>,
}

impl CommunePlan {
    pub fn new() -> Self {
        Self {
            grid_map: SparseCostMatrix::new(),
            terrain_map: [0; 2500],
            road_map: SparseCostMatrix::new(),
            plan_map: SparseCostMatrix::new(),
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

fn creeps_by_role() -> EnumMap<CreepRole, Vec<String>> {
    enum_map! {
        CreepRole::SourceHarvester => Vec::new(),
        CreepRole::Builder => Vec::new(),
        CreepRole::Upgrader => Vec::new(),
        CreepRole::Scout => Vec::new(),
        CreepRole::Hauler => Vec::new(),
        CreepRole::Repairer => Vec::new(),
        CreepRole::Antifa => Vec::new(),
        CreepRole::Unknown => Vec::new(),
        CreepRole::FastFill => Vec::new(),
        CreepRole::MineralHarvester => Vec::new(),
        CreepRole::RemoteHauler => Vec::new(),
        CreepRole::RemoteMineralHarvester => Vec::new(),
        CreepRole::RemoteBuilder => Vec::new(),
        CreepRole::RemoteSourceHarvester => Vec::new(),
        CreepRole::Downgraders => Vec::new(),
        CreepRole::Claimer => Vec::new(),
        CreepRole::RemoteReserver => Vec::new(),
        CreepRole::Hub => Vec::new(),
        CreepRole::Vanguard => Vec::new(),
    }
}
