use std::default;

use screeps::{game::map::RoomStatus, ObjectId, Source};
use serde::{Deserialize, Serialize};

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
    pub sources: Vec<ObjectId<Source>>,
}

impl CommuneRoomMemory {
    pub fn new() -> Self {
        Self { sources: Vec::new() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RemoteRoomMemory {
    pub sources: Vec<ObjectId<Source>>,
}

impl RemoteRoomMemory {
    pub fn new() -> Self {
        Self { sources: Vec::new() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IntersectionRoomMemory {
    pub portals: Vec<u32>,
}

impl IntersectionRoomMemory {
    pub fn new() -> Self {
        Self { portals: Vec::new() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CenterRoomMemory {
    pub portals: Vec<u32>,
}

impl CenterRoomMemory {
    pub fn new() -> Self {
        Self { portals: Vec::new() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct KeeperRoomMemory {
    pub portals: Vec<u32>,
}

impl KeeperRoomMemory {
    pub fn new() -> Self {
        Self { portals: Vec::new() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AllyRoomMemory {
    pub owner: String,
}

impl AllyRoomMemory {
    pub fn new() -> Self {
        Self { owner: "".to_string() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EnemyRoomMemory {
    pub owner: String,
}

impl EnemyRoomMemory {
    pub fn new() -> Self {
        Self { owner: "".to_string() }
    }
}


