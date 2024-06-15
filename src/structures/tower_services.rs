use core::fmt;
use std::error::Error;

use screeps::{
    find, Creep, HasHits, HasPosition, ObjectId, Position, Room, RoomName, RoomPosition,
    StructureTower, HEAL_POWER, RAMPART_DECAY_AMOUNT, RAMPART_DECAY_TIME,
};

use crate::{
    memory::game_memory::GameMemory,
    room::room_ops,
    state::game::GameState,
    utils::{self, general::{is_tick_interval, GeneralUtils}},
};

pub fn run_towers(room_name: &RoomName, game_state: &mut GameState, memory: &mut GameMemory) {
    let mut towers = room_ops::structures_by_type(room_name, game_state).tower.clone();

    if towers_creep_actions(room_name, game_state, memory, &mut towers) == TowersResult::Stop {
        return;
    }
    if towers_structure_actions(room_name, game_state, memory, &mut towers) == TowersResult::Stop {
        return;
    }
}

fn towers_creep_actions(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &GameMemory,
    towers: &mut Vec<StructureTower>,
) -> TowersResult {
    if tower_my_creep_actions(room_name, game_state, memory, towers) == TowersResult::Stop {
        return TowersResult::Stop;
    }
    if tower_not_my_creep_actions(room_name, game_state, memory, towers) == TowersResult::Stop {
        return TowersResult::Stop;
    }

    TowersResult::Continue
}

fn tower_my_creep_actions(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &GameMemory,
    towers: &mut Vec<StructureTower>,
) -> TowersResult {
    let room_state = game_state.room_states.get_mut(room_name).unwrap();
    let mut creep_names = &mut room_state.my_creeps;
    
    // Also make sure the creep isn't in enemy attack coords and hasn't been healed too often over the last few ticks (use a tower heal heat metric)

    creep_names.retain(|creep_name| {
        let my_creep = game_state.creeps.get(creep_name).unwrap();

        my_creep.inner().hits() < my_creep.inner().hits_max()
    });

    if creep_names.is_empty() {
        return TowersResult::Continue;
    }

    towers.retain(|tower| {

        let my_creep = game_state.creeps.get(&creep_names[0]).unwrap();
        tower.heal(my_creep.inner());

        false
    });

    match towers.len() {
        0 => TowersResult::Stop,
        _ => TowersResult::Continue,
    }
}

fn tower_not_my_creep_actions(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &GameMemory,
    towers: &mut Vec<StructureTower>,
) -> TowersResult {
    let mut creeps = room_ops::not_my_creeps(room_name, game_state, memory);

    if towers_attack_enemies(room_name, game_state, memory, towers, &mut creeps.enemy)
        == TowersResult::Stop
    {
        return TowersResult::Stop;
    }
    if towers_heal_ally_creeps(room_name, game_state, memory, towers, &mut creeps.ally)
        == TowersResult::Stop
    {
        return TowersResult::Stop;
    };

    TowersResult::Continue
}

fn towers_attack_enemies(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &GameMemory,
    towers: &mut Vec<StructureTower>,
    // Will need to mutate in the future to decide who to attack
    creeps: &mut Vec<Creep>,
) -> TowersResult {
    if creeps.is_empty() {
        return TowersResult::Continue;
    }

    // Filter creeps that we can deal enough damage to kill

    // Need to consider towers that don't have enough energy
    creeps.retain(|creep| {
        let max_potential_damage = find_towers_attack_power(towers, &creep.pos());
        let max_potential_heal = HEAL_POWER as u8 * creep.get_active_bodyparts(screeps::Part::Heal);

        // If we can damage more than they can heal plus a little, keep them as a target
        max_potential_damage as f32 > (max_potential_heal as f32 * 1.1)
    });

    if creeps.is_empty() {
        return TowersResult::Continue;
    }

    towers.retain(|tower| {
        tower.attack(&creeps[0]);

        false
    });

    match towers.len() {
        0 => TowersResult::Stop,
        _ => TowersResult::Continue,
    }
}

pub fn find_towers_attack_power(towers: &[StructureTower], target_pos: &Position) -> u32 {
    let mut total_attack_power = 0;

    for tower in towers {
        let range = utils::general::pos_range(&tower.pos(), target_pos);

        total_attack_power += screeps_utils::math::tower_attack_power_at_range(range as u8)
    }

    total_attack_power
}

fn towers_heal_ally_creeps(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &GameMemory,
    towers: &mut Vec<StructureTower>,
    creeps: &mut Vec<Creep>,
) -> TowersResult {
    creeps.retain(|creep| creep.hits() < creep.hits_max());

    if creeps.is_empty() {
        return TowersResult::Continue;
    }

    towers.retain(|tower| {
        tower.heal(&creeps[0]);

        false
    });

    match towers.len() {
        0 => TowersResult::Stop,
        _ => TowersResult::Continue,
    }
}

// repair ramparts that are too low

fn towers_structure_actions(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &GameMemory,
    towers: &mut Vec<StructureTower>,
) -> TowersResult {
    if (is_tick_interval(game_state.tick, RAMPART_DECAY_TIME / 2)) {
        return TowersResult::Continue;
    }

    if towers_repair_ramparts(room_name, game_state, memory, towers) == TowersResult::Stop {
        return TowersResult::Stop;
    }

    TowersResult::Continue
}

fn towers_repair_ramparts(
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &GameMemory,
    towers: &mut Vec<StructureTower>,
) -> TowersResult {
    let mut ramparts = room_ops::structures_by_type(room_name, game_state).rampart.clone();

    if ramparts.is_empty() {
        return TowersResult::Continue;
    }

    ramparts.retain(|rampart| rampart.hits() <= RAMPART_DECAY_AMOUNT * 2);

    if ramparts.is_empty() {
        return TowersResult::Continue;
    }

    towers.retain(|tower| {
        tower.repair(&ramparts[0]);

        false
    });

    match towers.len() {
        0 => TowersResult::Stop,
        _ => TowersResult::Continue,
    }
}

#[derive(PartialEq)]
/// Describes the control flow of tower services
pub enum TowersResult {
    Continue,
    Stop,
}
