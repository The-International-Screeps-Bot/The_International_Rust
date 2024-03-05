use std::collections::HashSet;

use screeps::{pathfinder::{self, SearchGoal}, Position, RoomName};

use crate::constants::general::AnyResult;

use super::pathfinder_ops::{PathGoals, PathfinderOps};

pub struct PathfindingOpts {
    pub room_callback: fn(&RoomName) -> u8,
    pub route_callback: fn(&RoomName) -> u8,
    pub avoid_enemy_creeps: bool,
}

pub struct PathfindingServices;

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
impl PathfindingServices {

    pub fn try_find_path(origin: &Position, goals: &PathGoals) -> Result<Vec<Position>, AnyResult> {
        let mut allowed_rooms = PathfindingServices::find_allowed_rooms(origin, goals);

        let path = PathfindingServices::generate_path(origin, goals);
        path
    }

    fn find_allowed_rooms(origin: &Position, goals: &PathGoals) {
        let mut allowed_rooms: HashSet<RoomName> = HashSet::new();
        allowed_rooms.insert(origin.room_name());


    }

    fn generate_path(origin: &Position, goals: &PathGoals) -> Result<Vec<Position>, AnyResult> {

        PathfinderOps::find_path(*origin, goals, None)
    }
}