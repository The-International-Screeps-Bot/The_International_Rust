use screeps::{ObjectId, SharedCreepProperties};

use crate::{creep::my_creep_ops::MyCreepOps, memory::game_memory::GameMemory, room::room_ops::RoomOps, state::game::GameState};

pub struct SourceHarvesterOps;

impl SourceHarvesterOps {
    pub fn harvest_steps(creep_name: &String, game_state: &mut GameState, memory: &mut GameMemory) {

        /* let creep = game_state.creeps.get(creep_name).unwrap();
        let creep_memory = memory.creeps.get(creep_name).unwrap();

        let room = game_state.rooms.get(&creep.inner().room().unwrap().name().clone()).unwrap();
        let sources = RoomOps::get_sources(room, game_state);

        let Some(source_index) = creep_memory.source_index else {
            return;
        };

        let source = sources.get(source_index).unwrap();

        MyCreepOps::drop_harvest(creep_name, &source, game_state, memory); */
    }
}