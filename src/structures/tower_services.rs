use screeps::{find, ObjectId, Room, StructureTower};

use crate::{room::room_ops::RoomOps, state::game::GameState};

pub struct TowerServices;

impl TowerServices {
    pub fn run_towers(room: &Room, game_state: &mut GameState) {
        // let towers = RoomOps::structures(room, game_state).towers;
        // let mut used_towers: Vec<ObjectId<StructureTower>> = Vec::new();

        // let creeps = RoomOps::not_my_creeps(room, game_state).enemy;
        // if creeps.len() == 0 {
        //     return
        // }

        // for tower in towers {
        //     tower.attack(creeps[0]);
        // }
    }
}