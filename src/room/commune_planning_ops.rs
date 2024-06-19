use screeps::RoomName;

use crate::{
    memory::game_memory::GameMemory,
    state::{game::GameState, room::CommunePlan},
};

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

    if try_config_planner(room_name, game_state, memory) == CommunePlanResult::Never {
        return CommunePlanResult::Never;
    }

    CommunePlanResult::Success
}

fn try_config_planner(
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

    find_fast_filler_start_positions(room_name, game_state, memory);

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
) {
}

fn try_config_plan(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
    plan: &mut CommunePlan,
) {

    let room_state = game_state.room_states.get_mut(room_name).unwrap();
    let Some(commune_plan) = &room_state.commune_plan else {
        return
    };

    room_state.commune_plan = Some(CommunePlan::new());
}
