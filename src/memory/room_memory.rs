use std::default;

use screeps::{game::map::RoomStatus, ObjectId, Source};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct RoomMemory {
    pub room_type: Option<RoomType>,
    pub status: Option<RoomStatus>,
    pub danger: Option<i32>,
    pub commune_sources: Vec<ObjectId<Source>>,
}

#[derive(Serialize, Deserialize, Default)]
pub enum RoomType {
    #[default]
    /// Rooms we control the controller of
    Commune,
    /// Includes reserved rooms. Essentially anything that doesn't fall under another type
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
