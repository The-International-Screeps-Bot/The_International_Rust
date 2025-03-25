use room_costs::economy_room_costs;
use route_costs::economy_creep_costs;
use screeps::RoomName;
use screeps_utils::sparse_cost_matrix::SparseCostMatrix;

use crate::{memory::game_memory::GameMemory, state::game::GameState};

pub mod portal_router;
pub mod pathfinding_services_multi;
pub mod pathfinding_services_single;
pub mod room_pather_multi;
pub mod room_pather_single;
pub mod route_costs;
pub mod room_costs;

pub struct PathfindingOpts {
    pub room_pathfinder_opts: RoomPathfinderOpts,
    pub route_callback: RouteCallback,
    pub avoid_enemy_creeps: Option<bool>,
}

impl PathfindingOpts {
    pub fn new() -> Self {
        Self {
            room_pathfinder_opts: RoomPathfinderOpts::new(),
            route_callback: economy_creep_costs,
            avoid_enemy_creeps: None,
        }
    }
}

pub type RoomCallback = fn(&RoomName) -> u8;
pub type RouteCallback = fn(&RoomName, &GameMemory) -> u8;

pub struct RoomPathfinderOpts {
    pub cost_callback: fn(&RoomName, &mut GameState, &GameMemory) -> SparseCostMatrix,
    pub allow_outside_origin_room: bool,
    pub avoid_enemy_attackers: bool,
}

impl RoomPathfinderOpts {
    pub fn new() -> Self {
        Self {
            cost_callback: economy_room_costs,
            allow_outside_origin_room: true,
            avoid_enemy_attackers: false,
        }
    }
}

