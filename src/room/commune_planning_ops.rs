use std::collections::{HashMap, HashSet};

use screeps::{HasPosition, Position, RoomName};

use crate::{
    memory::game_memory::GameMemory,
    pathfinding::{
        pathfinding_services::{self, PathfindingOpts},
        room_pather,
    },
    state::{game::GameState, room::CommunePlan},
};

use super::room_ops;

pub struct CommunePlanningOps;

pub const UNPROTECTED_COORD_WEIGHT: u32 = 3 * 16;
pub const DYNAMIC_DISTANCE_WEIGHT: u32 = 8;
pub const TOWER_DISTANCE_WEIGHT: u32 = 25;

pub enum CommuneStampType {
    FastFiller,
    Hub,
    InputLab,
    OutputLab,
    Tower,
    Observer,
    SourceLink,
    SourceExtension,
    Container,
    Extractor,
    Road,
    MinCutRampart,
    OnboardingRampart,
    ShieldRampart,
    GridExtension,
    Nuker,
    PowerSpawn,
}

#[derive(PartialEq)]
pub enum CommunePlanResult {
    Success,
    /// Stop trying to plan this tick, continue later
    StopForTick,
    /// Never try to plan this room again
    Never,
}

pub struct CommunePlanStampArgs {}

pub fn attempt_plan(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) -> CommunePlanResult {
    let room_state = game_state.room_states.get_mut(room_name).unwrap();
    room_state.commune_plan = Some(CommunePlan::new());

    if try_config_plan(room_name, game_state, memory) == CommunePlanResult::Never {
        return CommunePlanResult::Never;
    }

    CommunePlanResult::Success
}

fn try_config_plan(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) -> CommunePlanResult {
    let room_state = game_state.room_states.get_mut(room_name).unwrap();
    let mut plan = CommunePlan::new();

    if plan.fast_filler_start_positions.is_none() {
        return CommunePlanResult::Success;
    }

    plan.terrain_map = [0; 50 * 50];

    plan.fast_filler_start_positions = Some(find_fast_filler_start_positions(
        room_name, game_state, memory,
    ));

    let Some(fast_filler_start_positions) = plan.fast_filler_start_positions else {
        return CommunePlanResult::Never;
    };

    if fast_filler_start_positions.is_empty() {
        return CommunePlanResult::Never;
    }

    CommunePlanResult::Success
}

fn find_fast_filler_start_positions(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) -> Vec<Position> {
    let mut start_positions = Vec::new();

    let sources = room_ops::get_sources(room_name, game_state);
    let mut shortest_path: Option<Vec<Position>> = None;
    let controller_pos = room_ops::controller(room_name, game_state)
        .as_ref()
        .unwrap()
        .pos();

    for source in sources {
        let source_pos = source.pos();

        start_positions.push(source_pos);

        let mut goals: HashMap<Position, u32> = HashMap::new();
        goals.insert(controller_pos, 1);

        if let Ok(path) =
            pathfinding_services::try_find_path(&source_pos, &goals, PathfindingOpts::new(), memory)
        {
            let shortest_len = {
                if let Some(shortest_path) = &shortest_path {
                    shortest_path.len()
                } else {
                    usize::MAX
                }
            };

            if path.len() < shortest_len {
                shortest_path = Some(path);
            }
        }
    }

    // use chunker and select valid fastFiller for each chunk

    start_positions
}

fn try_config_current_plan(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
    plan: &mut CommunePlan,
) {

    // let room_state = game_state.room_states.get_mut(room_name).unwrap();
    // let Some(commune_plan) = &room_state.commune_plan else {
    //     return
    // };

    // room_state.commune_plan = Some(CommunePlan::new());
}
