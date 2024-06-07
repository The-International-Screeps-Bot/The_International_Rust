use std::{collections::HashSet, default};

use screeps::{game::map::RoomStatus, HasPosition, ObjectId, Position, RoomName, Source};
use serde::{Deserialize, Serialize};

use crate::state::game::GameState;

#[derive(Serialize, Deserialize)]
pub struct RoomMemory {
    pub room_type: RoomType,
    pub danger: u32,
}

impl RoomMemory {
    pub fn new() -> Self {
        Self {
            room_type: RoomType::Neutral,
            danger: 0,
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
}

impl RemoteRoomMemory {
    pub fn new(room_name: &RoomName, game_state: &mut GameState) -> Self {

        let room = game_state.rooms.get(room_name).unwrap();

        Self {
            source_positions: Vec::new(),
            controller_pos: room.controller().unwrap().pos(),
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
}

impl KeeperRoomMemory {
    pub fn new() -> Self {

        Self {
            keeper_positions: HashSet::new(),
            source_coords: Vec::new(),
            mineral_coords: Vec::new(),
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
