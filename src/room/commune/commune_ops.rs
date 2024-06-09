use screeps::{Room, RoomName};

use crate::{
    constants::structure::SpawnsByActivity, memory::game_memory::GameMemory, room::room_ops, state::{
        commune::{self, CommuneState},
        game::GameState,
    }
};

use super::spawning::spawn_services;

pub fn spawns_by_activity<'state>(
    room_name: &RoomName,
    game_state: &'state mut GameState,
) -> &'state Option<SpawnsByActivity> {
    let mut spawns_by_activity = SpawnsByActivity::new();

    let structures = room_ops::structures(room_name, game_state);

    for spawn in &structures.spawn {
        match spawn.spawning() {
            Some(spawning) => spawns_by_activity.active.push(spawn.clone()),
            _ => spawns_by_activity.inactive.push(spawn.clone()),
        }
    }

    let Some(commune_state) = game_state.commune_states.get_mut(room_name) else {
        return &None;
    };

    commune_state.spawns_by_activity = Some(spawns_by_activity);
    &commune_state.spawns_by_activity
}