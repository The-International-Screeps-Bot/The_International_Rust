use screeps::{game::map::describe_exits, RoomName};

use crate::{memory::{creep_memory, game_memory::GameMemory}, room::room_ops, state::game::GameState, utils};

pub fn try_register_scout_target(creep_name: &String, game_state: &mut GameState, memory: &mut GameMemory) {

    let creep_memory = memory.creeps.get(creep_name).unwrap();

    // Stop if we don't have a scout target
    let Some(scout_target) = creep_memory.scout_target else {
        return;
    };

    register_scout_target(scout_target, game_state);
}

fn register_scout_target(scout_target: RoomName, game_state: &mut GameState) {
    game_state.scout_targets.insert(scout_target);
}

pub fn try_scout(creep_name: &String, room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) {

    let Some(scout_target) = find_scout_target(creep_name, room_name, game_state, memory) else {
        return;
    };

    scout(creep_name, room_name, game_state, memory);
}

fn find_scout_target(creep_name: &String, room_name: &RoomName, game_state: &mut GameState, memory: &GameMemory) -> Option<RoomName> {

    /// The smallest scout tick, if exists
    let oldest_scout_tick: u32 = 0;
    let mut scouted_target: Option<RoomName> = None;

    let mut unscouted_targets: Vec<RoomName> = Vec::new();
    
    let room_status = room_ops::room_status(room_name, game_state);

    let exits = describe_exits(*room_name);
    for (direction, exit_room_name) in exits.entries() {

        // If the room statuses do not match
        if room_status != room_ops::room_status(&exit_room_name, game_state) {
            continue;
        }

        if let Some(room_memory) = memory.rooms.get(room_name) {

            scouted_target = Some(exit_room_name);
        }

        unscouted_targets.push(exit_room_name);
    }

    // If there are unscouted targets
    if !unscouted_targets.is_empty() {
        return Some(choose_unscouted_target(creep_name, room_name, game_state, memory, unscouted_targets));
    }

    if let Some(scouted_target) = scouted_target {
        return Some(scouted_target)
    };

    None
}

fn choose_unscouted_target(creep_name: &String, room_name: &RoomName, game_state: &mut GameState, memory: &GameMemory, unscouted_targets: Vec<RoomName>) -> RoomName {

    let creep_memory = memory.creeps.get(creep_name).unwrap();

    let mut lowest_range = u32::MAX;
    let mut scout_target = unscouted_targets[0];

    for target_room_name in unscouted_targets {
        let range = utils::general::room_range(room_name, &target_room_name);

        if range >= lowest_range {
            continue;
        }

        lowest_range = range;
        scout_target = target_room_name;
    }

    scout_target
}

fn scout(creep_name: &String, room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) {

}