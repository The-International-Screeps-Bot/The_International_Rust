use std::{collections::HashSet, default};

use enum_map::Enum;
use screeps::{
    find,
    game::{self, map::RoomStatus},
    FindConstant, HasPosition, ObjectId, Position, Room, RoomName, Source,
};
use serde::{Deserialize, Serialize};

use crate::{room::room_ops, state::game::GameState};

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomMemory {
    pub room_type: StaticRoomType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub danger: Option<u32>,
    pub last_scout: u32,
}

impl RoomMemory {
    pub fn new(game_state: &mut GameState) -> Self {
        Self {
            room_type: StaticRoomType::Neutral,
            danger: 0,
            last_scout: game_state.tick,
        }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug, Enum)]
/// Distinct descriptions of rooms that implies certain properties and does not change over time
pub enum StaticRoomType {
    /// Rooms we control the controller of
    #[serde(rename = "0")]
    Claimable,
    /// The rooms surrounding the center of a sector that potentially contain portals
    #[serde(rename = "1")]
    Keeper,
    /// The center room of a sector
    #[serde(rename = "2")]
    Center,
    /// Rooms bordering sectors, excluding corners
    #[serde(rename = "3")]
    Highway,
    /// Rooms bordering sectors that are corners and potentially contain portals
    #[serde(rename = "4")]
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

}

impl CommuneRoomMemory {
    pub fn new(room_name: &RoomName, game_state: &mut GameState) -> Self {

        Self {
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
    pub fn new(
        room_name: &RoomName,
        game_state: &mut GameState,
        cost: u32,
        source_paths: Vec<Vec<Position>>,
    ) -> Self {
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
    /// Communes which are blacklisted from potentially making this room a remote, likely because of too far of distance
    pub remote_blacklist: HashSet<RoomName>,
}

impl NeutralRoomMemory {
    pub fn new(room_name: &RoomName, game_state: &mut GameState) -> Self {
        let room = game_state.rooms.get(room_name).unwrap();

        Self {
            source_positions: Vec::new(),
            controller_pos: room.controller().unwrap().pos(),
            remote_blacklist: HashSet::new(),
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
        let keeper_lairs = &room_ops::structures_by_type(room_name, game_state).keeper_lair;

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

#[derive(Default)]
/// Used for all rooms that have a controller
pub struct ClaimableRoomMemory {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The name of the owner if it isn't owned by us
    pub non_me_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Wether or not we have claimed this room
    pub my_claim: Option<bool>,
    pub source_positions: Vec<Position>,
    // #[serde(serialize_with="serialize_pos")]
    #[serde(with = "screeps::local::serde_position_packed")]
    pub controller_pos: Position,
}

impl ClaimableRoomMemory {
    pub fn new(room_name: &RoomName, game_state: &mut GameState) -> Self {

        let sources = room_ops::get_sources(room_name, game_state);
        let source_positions: Vec<Position> = sources.iter().map(|source| source.pos()).collect();

        let room = game_state.rooms.get(room_name).unwrap();
        let controller_pos = room.controller().unwrap().pos();

        Self {
            source_positions,
            controller_pos,
            ....Default::default()
        }
    }
}
