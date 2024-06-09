use std::{collections::HashSet, default};

use screeps::{find, game::{self, map::RoomStatus}, FindConstant, HasPosition, ObjectId, Position, Room, RoomName, Source};
use serde::{Deserialize, Serialize};

use crate::{room::room_ops, state::game::GameState};

#[derive(Serialize, Deserialize)]
pub struct RoomMemory {
    pub room_type: RoomType,
    pub danger: u32,
    pub last_scout: u32,
}

impl RoomMemory {
    pub fn new(game_state: &mut GameState) -> Self {
        Self {
            room_type: RoomType::Neutral,
            danger: 0,
            last_scout: game_state.tick,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum RoomType {
    /// Rooms we control the controller of
    Commune,
    /// Rooms we intend to or will potentially harvest from
    Remote,
    /// Essentially anything that doesn't fall under another type
    Neutral,
    /// Rooms claimed by an enemy
    Enemy,
    /// Rooms claimed by an ally
    Ally,
    // The rooms surrounding the center of a sector that potentially contain portals
    Keeper,
    /// The center room of a sector
    Center,
    /// Rooms bordering sectors, excluding corners
    Highway,
    /// Rooms bordering sectors that are corners and potentially contain portals
    Intersection,
}

#[derive(Serialize, Deserialize)]
pub struct HighwayRoomMemory {
    pub deposits: Vec<u32>,
    pub power_banks: Vec<u32>,
}

impl HighwayRoomMemory {
    pub fn new() -> Self {
        Self {
            deposits: Vec::new(),
            power_banks: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CommuneRoomMemory {
    pub source_positions: Vec<Position>,
    pub controller_pos: Position,
}

impl CommuneRoomMemory {
    pub fn new(room_name: &RoomName, game_state: &mut GameState) -> Self {

        let room = game_state.rooms.get(room_name).unwrap();

        Self {
            source_positions: Vec::new(),
            controller_pos: room.controller().unwrap().pos(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RemoteRoomMemory {
    pub source_positions: Vec<Position>,
    pub controller_pos: Position,
    pub commune: RoomName,
    pub source_paths: Vec<Vec<Position>>,
    pub cost: u32,
    pub abandon: u32,
}

impl RemoteRoomMemory {
    pub fn new(room_name: &RoomName, game_state: &mut GameState, cost: u32, source_paths: Vec<Vec<Position>>,) -> Self {

        let room = game_state.rooms.get(room_name).unwrap();

        Self {
            source_positions: Vec::new(),
            controller_pos: room.controller().unwrap().pos(),
            commune: *room_name,
            source_paths,
            cost,
            abandon: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NeutralRoomMemory {
    pub source_positions: Vec<Position>,
    pub controller_pos: Position,
}

impl NeutralRoomMemory {
    pub fn new(room_name: &RoomName, game_state: &mut GameState) -> Self {

        let room = game_state.rooms.get(room_name).unwrap();

        Self {
            source_positions: Vec::new(),
            controller_pos: room.controller().unwrap().pos(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IntersectionRoomMemory {
    pub portals: Vec<u32>,
}

impl IntersectionRoomMemory {
    pub fn new() -> Self {
        Self {
            portals: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CenterRoomMemory {
    pub portals: Vec<u32>,
    pub source_positions: Vec<Position>,
}

impl CenterRoomMemory {
    pub fn new() -> Self {
        Self {
            portals: Vec::new(),
            source_positions: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct KeeperRoomMemory {
    pub keeper_positions: HashSet<Position>,
    pub source_coords: Vec<Position>,
    pub mineral_coords: Vec<Position>,
    pub invader_core_level: Option<u32>,
    pub keeper_lair_positions: Vec<Position>,
}

impl KeeperRoomMemory {
    pub fn new(room_name: &RoomName, game_state: &mut GameState) -> Self {

        let room = game_state.rooms.get(room_name).unwrap();

        // Keeper lair positions

        let mut keeper_lair_positions = Vec::new();
        let keeper_lairs = &room_ops::structures(room_name, game_state).keeper_lair;

        for keeper_lair in keeper_lairs {
            keeper_lair_positions.push(keeper_lair.pos());
        }

        Self {
            keeper_positions: HashSet::new(),
            source_coords: Vec::new(),
            mineral_coords: Vec::new(),
            keeper_lair_positions,
            invader_core_level: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AllyRoomMemory {
    pub owner: String,
}

impl AllyRoomMemory {
    pub fn new() -> Self {
        Self {
            owner: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EnemyRoomMemory {
    pub owner: String,
    pub terminal: bool,
    pub stored_energy: u32,
    pub min_hits_to_breach: Option<u32>,
}

impl EnemyRoomMemory {
    pub fn new() -> Self {
        Self {
            owner: "".to_string(),
            terminal: false,
            stored_energy: 0,
            min_hits_to_breach: None,
        }
    }
}
