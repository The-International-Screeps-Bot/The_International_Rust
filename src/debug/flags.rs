use log::error;
use screeps::{HasPosition, RoomName, RoomVisual};

use crate::{memory::game_memory::GameMemory, room::room_ops, state::game::GameState};

pub fn run_flags(game_state: &mut GameState, memory: &mut GameMemory) {
    // Implement flag-related logic here

    let flag_names = game_state.flags.keys().cloned().collect::<Vec<String>>();
    for flag_name in flag_names {
        let flag_params = flag_name.split(" ").collect::<Vec<&str>>();

        let Some(flag_type) = flag_params.get(0) else {
            continue;
        };

        let flag_pos = {
            let flag = game_state.flags.get(&flag_name).unwrap();
            flag.pos()
        };

        match *flag_type {
            "reserved_positions" => {
                visualize_reserved_positions(flag_params, flag_pos.room_name(), game_state, memory);
            }
            "harvest_positions" => {
                visualize_harvest_positions(flag_params, flag_pos.room_name(), game_state, memory);
            }
            _ => {}
        }
    }
}

fn visualize_reserved_positions(
    flag_params: Vec<&str>,
    room_name: RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) {
    let room_visual = RoomVisual::new(Some(room_name));
    
    let Some(commune_state) = game_state.commune_states.get(&room_name) else {
        error!(
            "Placed reserved_positions flag in invalid room {}",
            room_name
        );
        return;
    };

    for pos in &commune_state.reserved_positions {
        room_visual.circle(pos.x().0 as f32, pos.y().0 as f32, None);
    }
}

fn visualize_harvest_positions(
    flag_params: Vec<&str>,
    room_name: RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) {
    let room_visual = RoomVisual::new(Some(room_name));

    let Some(harvest_positions) = room_ops::harvest_positions(&room_name, game_state, memory)
    else {
        error!(
            "Placed reserved_positions flag in invalid room {}",
            room_name
        );
        return;
    };

    for vec in harvest_positions {
        for pos in vec {
            room_visual.circle(pos.x().0 as f32, pos.y().0 as f32, None);
        }
    }
}
