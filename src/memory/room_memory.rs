use std::default;

use screeps::game::map::RoomStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct RoomMemory {
    pub room_type: Option<RoomType>,
    pub status: Option<RoomStatus>,
    pub danger: Option<i32>,
}

#[derive(Serialize, Deserialize, Default)]
pub enum RoomType {
    #[default]
    Commune,
    Neutral,
    Enemy,
    Ally,
    Keeper,
    Center,
    Highway,
    Intersection,
}