use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use enum_map::EnumMap;
use screeps::{
    AccountPowerCreep, Creep, MaybeHasId, OwnedStructureProperties, Room, RoomName,
    SharedCreepProperties, StructureType,
    game::{self, shard},
};

use super::{
    commune::{self, CommuneState},
    market::MarketState,
    my_creep::MyCreepState,
    room::RoomState,
    segments::Segments,
    structure::{self, StructuresState},
    tick_intervals::TickIntervals,
};
use crate::{
    constants::{creep::CreepRole, general::GeneralResult},
    creep::my_creep::{self, MyCreep},
    memory::{game_memory::GameMemory, room_memory::CommuneRoomMemory},
    room::room_ops,
    settings::Settings,
    state::creep::CreepState,
    utils::{self, general::GeneralUtils},
};

#[derive(Debug)]
/// Contains important information about the game
#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
pub struct GameState {
    pub init_tick: u32,
    pub tick: u32,
    pub shard: String,
    pub highest_rcl: u8,
    pub creeps: HashMap<String, MyCreep>,
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
    pub my_creep_states: HashMap<String, MyCreepState>,
    /// Current scout targets by scout creeps
    pub scout_targets: HashSet<RoomName>,
    pub intervals: TickIntervals,
    pub segments: Segments,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            init_tick: game::time(),
            tick: game::time(),
            shard: shard::name(),
            highest_rcl: 0,
            creeps: HashMap::new(),
            account_power_creeps: HashMap::new(),
            rooms: HashMap::new(),
            communes: HashSet::new(),
            creep_id_index: 0,
            terminal_communes: HashSet::new(),
            market_state: MarketState::new(),
            structures_state: StructuresState::new(),
            room_states: HashMap::new(),
            commune_states: HashMap::new(),
            creep_states: HashMap::new(),
            my_creep_states: HashMap::new(),
            scout_targets: HashSet::new(),
            intervals: TickIntervals::new(),
            segments: Segments::new(),
        }
    }

    /// Called every possible tick, including the tick when initialized
    pub fn tick_update(&mut self, memory: &mut GameMemory) {
        self.tick = game::time();

        self.update_my_creeps();
        // TODO
        // GameStateOps::update_account_power_creeps(self);
        self.update_rooms();
        self.update_communes(memory);
        self.update_creep_id_index();

        // state type updating

        self.update_rooms_state();
        self.update_communes_state();
        self.update_my_creeps_state();
        self.update_creeps_state();
        self.update_structures_state();

        //

        self.find_highest_rcl();
        self.update_terminal_communes();
    }

    fn update_my_creeps(&mut self) {
        self.creeps.clear();

        let js_creeps = screeps::game::creeps();

        for (creep_name, any_creep) in js_creeps.keys().zip(js_creeps.values()) {
            let Some(creep) = MyCreep::new(&any_creep).ok() else {
                continue;
            };

            if !self.my_creep_states.contains_key(&creep_name) {
                self.my_creep_states.insert(
                    creep_name.clone(),
                    MyCreepState::new(creep_name.as_str(), &any_creep),
                );
            } else {
                let my_creep_state = self.my_creep_states.get_mut(&creep_name).unwrap();

                my_creep_state.tick_update(&creep);
            }

            if !self.creep_states.contains_key(&creep_name) {
                self.creep_states
                    .insert(creep_name.clone(), CreepState::new(creep_name.as_str()));
            }

            self.creeps.insert(creep_name, creep);
        }
    }

    // fn update_account_power_creeps(&mut self) {
    //     self.account_power_creeps.clear();

    //     let js_creeps = screeps::game::power_creeps();

    //     for creep_name in js_creeps.keys() {
    //         let Some(creep) = js_creeps.get(creep_name) else { continue; };
    //         self.account_power_creeps.insert(creep_name, creep);
    //     }
    // }

    fn update_rooms(&mut self) {
        self.rooms.clear();

        let js_rooms = screeps::game::rooms();

        for (room_name, room) in js_rooms.keys().zip(js_rooms.values()) {
            if !self.room_states.contains_key(&room_name) {
                self.room_states
                    .insert(room_name, RoomState::new(room_name, self));
            }

            self.rooms.insert(room_name, room);
        }
    }

    fn update_communes(&mut self, memory: &mut GameMemory) {
        self.communes.clear();

        let room_names = self.rooms.keys().cloned().collect::<Vec<RoomName>>();
        for room_name in room_names {
            let room = self.rooms.get(&room_name).unwrap();

            let Some(controller) = room.controller() else {
                return;
            };

            if !controller.my() {
                return;
            }

            self.communes.insert(room_name);
        }
    }

    fn update_creep_id_index(&mut self) {
        self.creep_id_index = 0;
    }

    fn update_rooms_state(&mut self) {
        for (room_name, room_state) in &mut self.room_states {
            room_state.tick_update(room_name);

            let has_vision = self.rooms.contains_key(room_name);
            room_state.track_vision(has_vision, self.tick);
        }

        if !utils::general::is_tick_interval(self.tick, 100) {
            return;
        }

        self.room_states.retain(|room_name, state| !state.expired);

        for (room_name, room_state) in &mut self.room_states {
            room_state.interval_update(room_name);
        }
    }

    fn update_communes_state(&mut self) {
        for (room_name, commune_state) in &mut self.commune_states {
            commune_state.tick_update(room_name);
        }

        if !utils::general::is_tick_interval(self.tick, 100) {
            return;
        }

        self.commune_states
            .retain(|room_name, _| self.communes.contains(room_name));

        for (room_name, commune_state) in &mut self.commune_states {
            commune_state.interval_update(room_name);
        }
    }

    fn update_my_creeps_state(&mut self) {
        // Tick update done in update_my_creeps

        if !utils::general::is_tick_interval(self.tick, 100) {
            return;
        }

        self.my_creep_states
            .retain(|creep_name, _| self.creeps.contains_key(creep_name));

        for (creep_name, my_creep_state) in &mut self.my_creep_states {
            my_creep_state.interval_update();
        }
    }

    fn update_creeps_state(&mut self) {
        if !utils::general::is_tick_interval(self.tick, 100) {
            return;
        }

        // Delete creep states every so often
        self.creep_states = HashMap::new();

        // for (creep_name, creep_state) in &mut self.creep_states {
        //     creep_state.interval_update();
        // }
    }

    fn update_structures_state(&mut self) {
        if !utils::general::is_tick_interval(self.tick, 100) {
            return;
        }

        self.structures_state.active_statuses.clear()
    }

    fn find_highest_rcl(&mut self) {
        let mut highest_rcl = 0;

        for (room_name, commune_state) in &self.commune_states {
            if commune_state.rcl <= highest_rcl {
                continue;
            }

            highest_rcl = commune_state.rcl;
        }

        self.highest_rcl = highest_rcl
    }

    fn update_terminal_communes(&mut self) {
        let mut terminal_communes: HashSet<RoomName> = HashSet::new();

        let room_names: Vec<RoomName> = self.rooms.keys().cloned().collect();
        for room_name in &room_names {
            let Some(room_state) = self.room_states.get_mut(room_name) else {
                continue;
            };

            let Some(terminal) = room_ops::terminal(room_name, self) else {
                continue;
            };

            terminal_communes.insert(*room_name);
        }

        self.terminal_communes = terminal_communes;
    }

    pub fn get_or_create_room_state_mut(&mut self, room_name: &RoomName) -> &mut RoomState {
        if self.room_states.contains_key(room_name) {
            return self.room_states.get_mut(room_name).unwrap();
        }

        let room_state = RoomState::new(*room_name, self);
        self.room_states.insert(*room_name, room_state);
        self.room_states.get_mut(room_name).unwrap()
        
        // let maybe_state = self.room_states.get_mut(room_name);
        
        // match maybe_state {
        //     Some(state) => state,
        //     None => {
        //         drop(maybe_state);
                
        //         let room_state = RoomState::new(*room_name, self);
        //         self.room_states.insert(*room_name, room_state);
        //         self.room_states.get_mut(room_name).unwrap()
        //     }
        // }
    }

    pub fn get_or_create_creep_state(&mut self, creep_name: &str) -> &mut CreepState {
        if self.creep_states.contains_key(creep_name) {
            return self.creep_states.get_mut(creep_name).unwrap();
        }

        let creep_state = CreepState::new(creep_name);
        self.creep_states
            .insert(creep_name.to_string(), creep_state);
        self.creep_states.get_mut(creep_name).unwrap()
    }
}
