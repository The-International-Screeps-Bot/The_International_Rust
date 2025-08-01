use std::collections::HashMap;
use std::f64;

use enum_map::{enum_map, EnumMap};
use log::debug;
use screeps::{
    constants::creep::Part, BodyPart, Room, RoomName, SpawnOptions, Spawning, MAX_CREEP_SIZE,
};

use crate::{
    constants::{
        creep::{BodypartCounts, CreepPart, CreepParts},
        spawning::{
            GroupDiverseSpawnRequestArgs, GroupUniformSpawnRequestArgs,
            IndividualUniformSpawnRequestArgs, SpawnRequest, SpawnRequestArgs,
        },
    }, logging, memory::game_memory::GameMemory, state::{commune::CommuneState, game::GameState, room::RoomState}
};

pub fn spawn_request_individual_uniform(
    args: IndividualUniformSpawnRequestArgs,
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) -> Vec<SpawnRequest> {
    let max_cost_per_creep = get_max_cost_per_creep(
        args.min_cost_per_creep,
        Some(args.max_cost_per_creep),
        room_name,
        game_state,
    );
    let mut creeps_quota = args.creeps_quota;

    let mut spawn_requests = Vec::new();

    while creeps_quota > 0 {
        let mut body_part_counts: BodypartCounts = enum_map! {
            CreepPart::Move => 0,
            CreepPart::Attack => 0,
            CreepPart::Carry => 0,
            CreepPart::Claim => 0,
            CreepPart::Heal => 0,
            CreepPart::RangedAttack => 0,
            CreepPart::Tough => 0,
            CreepPart::Work => 0,
        };
        let mut tier = 0;
        let mut cost = 0;

        if !args.default_parts.is_empty() {
            tier += 1;
            for part in &args.default_parts {
                let part_cost = part.cost();
                if cost + part_cost > max_cost_per_creep {
                    break;
                }

                cost += part_cost;
                body_part_counts[*part] += 1;
            }
        }

        let mut remaining_allowed_parts = MAX_CREEP_SIZE - args.default_parts.len() as u32;

        if !args.extra_parts.is_empty() {
            let mut remaining_extra_parts =
                args.extra_parts_quota as i32 - args.default_parts.len() as i32;

            while cost < max_cost_per_creep
                && remaining_allowed_parts >= args.extra_parts.len() as u32
                && remaining_extra_parts > 0
            {
                tier += 1;

                for part in &args.extra_parts {
                    let part_cost = part.cost();
                    if cost + part_cost > max_cost_per_creep {
                        break;
                    }

                    cost += part_cost;
                    body_part_counts[(*part)] += 1;

                    remaining_allowed_parts -= 1;
                    remaining_extra_parts -= 1;
                }

                if cost >= args.min_cost_per_creep && remaining_extra_parts <= 0 {
                    break;
                }
            }
        }

        spawn_requests.push(SpawnRequest {
            role: args.role,
            priority: args.priority,
            body_part_counts,
            tier,
            cost,
            memory: args.memory_additions.clone(),
            spawn_target: args.spawn_target,
        });

        creeps_quota = creeps_quota.saturating_sub(1);
    }

    spawn_requests
}

pub fn spawn_request_group_diverse(
    args: &GroupDiverseSpawnRequestArgs,
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) -> Vec<SpawnRequest> {
    let max_cost_per_creep = get_max_cost_per_creep(
        args.min_cost_per_creep,
        args.max_cost_per_creep,
        room_name,
        game_state,
    );

    let total_extra_parts = args.extra_parts.len() as u32 * args.extra_parts_quota;

    let max_parts_per_creep = u32::min(
        50 - args.default_parts.len() as u32,
        total_extra_parts,
    );

    let mut total_extra_parts = total_extra_parts;

    // Probably don't need threshold stuff anymore
    if (total_extra_parts as f32) < (max_parts_per_creep as f32) * (args.threshold.unwrap_or(0.25))
    {
        return Vec::new();
    }

    let mut max_creeps: u32 = args.max_creeps.unwrap_or(u32::MAX);

    let mut extra_parts_cost = 0;

    for part in &args.extra_parts {
        extra_parts_cost += part.cost();
    }

    let mut spawn_requests = Vec::new();

    while total_extra_parts >= args.extra_parts.len() as u32 && max_creeps > 0 {
        let mut body_part_counts: BodypartCounts = enum_map! {
            CreepPart::Move => 0,
            CreepPart::Attack => 0,
            CreepPart::Carry => 0,
            CreepPart::Claim => 0,
            CreepPart::Heal => 0,
            CreepPart::RangedAttack => 0,
            CreepPart::Tough => 0,
            CreepPart::Work => 0,
        };
        let mut tier = 0;
        let mut cost = 0;

        if !args.default_parts.is_empty() {
            tier += 1;
            for part in &args.default_parts {
                let part_cost = part.cost();
                if cost + part_cost > max_cost_per_creep {
                    break;
                }

                cost += part_cost;
                body_part_counts[(*part)] += 1;
            }
        }

        let mut remaining_allowed_parts = max_parts_per_creep as i32;

        tier += 1;

        for part in &args.extra_parts {
            cost += part.cost();
            body_part_counts[(*part)] += 1;

            remaining_allowed_parts -= 1;
            total_extra_parts -= 1;
        }

        let mut stop = false;

        while cost < max_cost_per_creep
            && remaining_allowed_parts - (args.extra_parts.len() as i32) >= 0
        {
            tier += 1;

            for part in &args.extra_parts {
                let part_cost = part.cost();
                if cost + part_cost > max_cost_per_creep && cost >= args.min_cost_per_creep {
                    stop = true;
                    break;
                }

                cost += part_cost;
                body_part_counts[(*part)] += 1;

                remaining_allowed_parts -= 1;
                total_extra_parts -= 1;
            }

            if stop {
                break;
            }
        }

        spawn_requests.push(SpawnRequest {
            role: args.role,
            priority: args.priority,
            body_part_counts,
            tier,
            cost,
            memory: args.memory_additions.clone(),
            spawn_target: args.spawn_target,
        });

        max_creeps = max_creeps.saturating_sub(1);
    }

    spawn_requests
}

pub fn spawn_request_group_uniform(
    args: &GroupUniformSpawnRequestArgs,
    room_name: &RoomName,
    game_state: &mut GameState,
    memory: &mut GameMemory,
) -> Vec<SpawnRequest> {
    if args.extra_parts.is_empty() {
        return Vec::new();
    }

    let max_cost_per_creep = get_max_cost_per_creep(
        args.min_cost_per_creep,
        args.max_cost_per_creep,
        room_name,
        game_state,
    );

    let mut max_creeps = args.max_creeps.unwrap_or(u32::MAX);

    let mut extra_parts_quota = args.extra_parts_quota;

    let mut spawn_requests = Vec::new();

    while extra_parts_quota > 0 && max_creeps > 0 {
        let mut body_part_counts: BodypartCounts = enum_map! {
            CreepPart::Move => 0,
            CreepPart::Attack => 0,
            CreepPart::Carry => 0,
            CreepPart::Claim => 0,
            CreepPart::Heal => 0,
            CreepPart::RangedAttack => 0,
            CreepPart::Tough => 0,
            CreepPart::Work => 0,
        };
        let mut parts_count = 0;
        let mut tier = 0;
        let mut cost = 0;

        if !args.default_parts.is_empty() {
            tier += 1;
            for part in &args.default_parts {
                let part_cost = part.cost();
                if cost + part_cost > max_cost_per_creep {
                    break;
                }

                cost += part_cost;
                body_part_counts[(*part)] += 1;
                parts_count += 1;
            }
        }

        let mut stop = false;

        while cost < max_cost_per_creep
            && (parts_count + args.extra_parts.len() as u32) <= MAX_CREEP_SIZE
        {
            tier += 1;

            for part in &args.extra_parts {
                let part_cost = part.cost();
                if cost + part_cost > max_cost_per_creep /* && cost >= args.min_cost_per_creep */ {
                    stop = true;
                    break;
                }

                cost += part_cost;
                body_part_counts[(*part)] += 1;
                parts_count += 1;
            }

            if stop {
                break;
            }
        }

        spawn_requests.push(SpawnRequest {
            role: args.role,
            priority: args.priority,
            body_part_counts,
            tier,
            cost,
            memory: args.memory_additions.clone(),
            spawn_target: args.spawn_target,
        });

        extra_parts_quota = extra_parts_quota.saturating_sub(parts_count);
        max_creeps = max_creeps.saturating_sub(1);
    }

    spawn_requests
}

fn get_max_cost_per_creep(
    min_cost: u32,
    max_cost: Option<u32>,
    room_name: &RoomName,
    game_state: &mut GameState,
) -> u32 {
    let commune_state = game_state.commune_states.get_mut(room_name).unwrap();

    match max_cost {
        Some(cost) => {
            if cost < min_cost {
                panic!("Max cost per creep cannot be less than min cost per creep");
                return 0;
            }

            if cost > commune_state.spawn_energy_capacity {
                panic!("Max cost per creep cannot be greater than spawn energy capacity");
                return 0;
            }

            cost
        }
        None => commune_state.spawn_energy_capacity,
    }
}
