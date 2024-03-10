use std::collections::{HashMap, HashSet};

use screeps::{
    game, AccountPowerCreep, Creep, OwnedStructureProperties, Room, RoomName,
    SharedCreepProperties, StructureType,
};

use super::{
    commune::{CommuneState, CommuneStateOps},
    creep::CreepStateOps,
    market::MarketState,
    room::{RoomState, RoomStateOps},
    structure::{self, StructuresState},
};
use crate::{
    creep::owned_creep::{self, OwnedCreep},
    memory::game_memory::GameMemory,
    room::room_ops::RoomOps,
    settings::Settings,
    state::creep::CreepState,
    utils::general::GeneralUtils,
};

#[derive(Debug, Default)]
/// Contains important information about the game
pub struct GameState {
    pub init_tick: u32,
    pub tick: u32,
    pub creeps: HashMap<String, OwnedCreep>,
    pub account_power_creeps: HashMap<String, AccountPowerCreep>,
    pub rooms: HashMap<RoomName, Room>,
    pub communes: HashSet<RoomName>,
    pub creep_id_index: u32,
    pub terminal_communes: HashSet<RoomName>,
    pub market_state: MarketState,
    pub structures_state: StructuresState,
    pub room_states: HashMap<RoomName, RoomState>,
    pub commune_states: HashMap<RoomName, CommuneState>,
    pub creep_states: HashMap<String, CreepState>,
}

pub struct GameStateOps;

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
impl GameStateOps {
    /// Called every possible tick, including the tick when initialized
    pub fn update(game_state: &mut GameState, memory: &GameMemory) {
        game_state.tick = game::time();

        Self::update_creeps(game_state);
        // TODO
        // GameStateOps::update_account_power_creeps(game_state);
        Self::update_rooms(game_state, memory);
        Self::update_creep_id_index(game_state);

        // state type updating

        Self::update_rooms_state(game_state);
        Self::update_communes_state(game_state);
        Self::update_creeps_state(game_state);
        Self::update_structures_state(game_state);

        //

        Self::update_terminal_communes(game_state);
    }

    fn update_creeps(game_state: &mut GameState) {
        game_state.creeps.clear();

        let js_creeps = screeps::game::creeps();

        for creep_name in js_creeps.keys() {
            let Some(any_creep) = js_creeps.get(creep_name.clone())
            /* .and_then(|creep| OwnedCreep::new(&creep).ok()) */
            else {
                continue;
            };

            let Some(creep) = OwnedCreep::new(&any_creep).ok() else {
                continue;
            };

            if !game_state.creep_states.contains_key(&creep_name) {
                game_state
                    .creep_states
                    .insert(creep_name.clone(), CreepState::new(&creep, creep_name));
            }

            game_state.creeps.insert(creep.inner().name(), creep);
        }
    }

    // fn update_account_power_creeps(game_state: &mut GameState) {
    //     game_state.account_power_creeps.clear();

    //     let js_creeps = screeps::game::power_creeps();

    //     for creep_name in js_creeps.keys() {
    //         let Some(creep) = js_creeps.get(creep_name) else { continue; };
    //         game_state.account_power_creeps.insert(creep_name, creep);
    //     }
    // }

    fn update_rooms(game_state: &mut GameState, memory: &GameMemory) {
        game_state.rooms.clear();
        game_state.communes.clear();

        let js_rooms = screeps::game::rooms();

        for room_name in js_rooms.keys() {
            let Some(room) = js_rooms.get(room_name) else {
                continue;
            };

            Self::try_update_commune(&room, &room_name, game_state, memory);

            if !game_state.room_states.contains_key(&room_name) {
                game_state
                    .room_states
                    .insert(room_name.clone(), RoomState::new(&room, room_name));
            }

            game_state.rooms.insert(room_name, room);
        }
    }

    fn try_update_commune(
        room: &Room,
        room_name: &RoomName,
        game_state: &mut GameState,
        memory: &GameMemory,
    ) {
        let Some(controller) = room.controller() else {
            return;
        };

        if !controller.my() {
            return;
        }

        game_state.communes.insert(room_name.clone());
        // If the commune doesn't have a state, create one
        if !game_state.commune_states.contains_key(room_name) {
            game_state
                .commune_states
                .insert(room_name.clone(), CommuneState::new(room, *room_name));
        };
    }

    fn update_creep_id_index(game_state: &mut GameState) {
        game_state.creep_id_index = 0;
    }

    fn update_rooms_state(game_state: &mut GameState) {
        if !GeneralUtils::is_tick_interval(100) {
            return;
        }

        game_state
            .room_states
            .retain(|room_name, _| game_state.rooms.contains_key(room_name));

        for (room_name, room_state) in &mut game_state.room_states {
            RoomStateOps::update_state(room_state);
        }
    }

    fn update_communes_state(game_state: &mut GameState) {
        if !GeneralUtils::is_tick_interval(100) {
            return;
        }

        game_state
            .commune_states
            .retain(|room_name, _| game_state.communes.contains(room_name));

        for (room_name, commune_state) in &mut game_state.commune_states {
            CommuneStateOps::update_state(commune_state);
        }
    }

    fn update_creeps_state(game_state: &mut GameState) {
        if !GeneralUtils::is_tick_interval(100) {
            return;
        }

        game_state
            .creep_states
            .retain(|creep_name, _| game_state.creeps.contains_key(creep_name));

        for (creep_name, creep_state) in &mut game_state.creep_states {
            CreepStateOps::update_state(creep_state);
        }
    }

    fn update_structures_state(game_state: &mut GameState) {
        if !GeneralUtils::is_tick_interval(100) {
            return;
        }

        game_state.structures_state.active_statuses.clear()
    }

    fn update_terminal_communes(game_state: &mut GameState) {
        let mut terminal_communes: HashSet<RoomName> = HashSet::new();

        for room_name in &game_state.communes {
            let Some(room) = game_state.rooms.get(room_name) else {
                continue;
            };

            let Some(room_state) = game_state.room_states.get_mut(room_name) else {
                continue;
            };

            let structures = RoomOps::structures(room, room_state);

            let Some(terminals) = structures.get(&StructureType::Terminal) else {
                continue;
            };

            if terminals.len() < 1 {
                continue;
            }

            terminal_communes.insert(room_name.clone());
        }

        game_state.terminal_communes = terminal_communes;
    }
}
