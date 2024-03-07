use std::collections::HashMap;

use screeps::RoomName;

#[derive(Default, Debug)]
pub struct CommunesState {
    pub spawn_energy_capacitys: HashMap<RoomName, u32>
}

impl CommunesState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}