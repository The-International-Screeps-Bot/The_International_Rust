use std::collections::HashMap;

use screeps::RoomName;

#[derive(Default)]
pub struct RoomState {

}

impl RoomState {
    pub fn new() -> Self {
        RoomState {  }
    }
}

pub type RoomsState = HashMap<RoomName, RoomState>;