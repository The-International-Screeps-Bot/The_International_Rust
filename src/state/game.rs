use std::collections::{HashMap, HashSet};

use screeps::{game, AccountPowerCreep, Creep, Room, RoomName, SharedCreepProperties};

use super::{creep::CreepsState, market::MarketState, room::RoomsState, structure::StructureState};

#[derive(Default)]
pub struct GameState {
    pub init_tick: u32,
    pub tick: u32,
    pub creeps: HashMap<String, Creep>,
    pub account_power_creeps: HashMap<String, AccountPowerCreep>,
    pub rooms: HashMap<RoomName, Room>,
    pub communes: HashSet<RoomName>,
    pub creeps_state: CreepsState,
    pub rooms_state: RoomsState,
    pub creep_id_index: u32,
    pub has_terminal: bool,
    pub market_state: MarketState,
    pub structure_state: StructureState,
}

/// Contains important information about the game
impl GameState {
    pub fn new() -> Self {
        let tick = game::time();

        GameState {
            init_tick: tick,
            tick,
            creeps: HashMap::new(),
            account_power_creeps: HashMap::new(),
            rooms: HashMap::new(),
            communes: HashSet::new(),
            creeps_state: HashMap::new(),
            rooms_state: HashMap::new(),
            ..Default::default()
        }
    }
}

pub struct GameStateOps;

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
impl GameStateOps {
    /// Called every possible tick, including the tick when initialized
    pub fn update(game_state: &mut GameState) {
        game_state.tick = game::time();

        Self::update_creeps(game_state);
        // GameStateOps::update_account_power_creeps(game_state);
        Self::update_rooms(game_state);
        Self::update_communes(game_state);
        Self::update_creep_id_index(game_state);
        Self::update_has_terminal(game_state);
    }

    fn update_creeps(game_state: &mut GameState) {
        game_state.creeps.clear();

        let js_creeps = screeps::game::creeps();

        for creep_name in js_creeps.keys() {
            let Some(creep) = js_creeps.get(creep_name) else {
                continue;
            };
            game_state.creeps.insert(creep.name(), creep);
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

    fn update_rooms(game_state: &mut GameState) {
        game_state.rooms.clear();

        let js_rooms = screeps::game::rooms();

        for room_name in js_rooms.keys() {
            let Some(room) = js_rooms.get(room_name) else {
                continue;
            };
            game_state.rooms.insert(room_name, room);
        }
    }

    fn update_communes(game_state: &mut GameState) {
        game_state.communes = game_state.rooms.keys().copied().collect();
    }

    fn update_creep_id_index(game_state: &mut GameState) {
        game_state.creep_id_index = 0;
    }

    fn update_has_terminal(game_state: &mut GameState) {

        let rooms = &game_state.rooms;
        for (room_name, room) in rooms {
            if room.controller().is_some() {
                game_state.has_terminal = true;
                return
            }
        }

        game_state.has_terminal = false
    }
}
