// FIXME: remove this, but right now it's just causing warning fatigue
#![allow(unused)]

use core::cell::RefCell;
use std::collections::{HashMap, HashSet};

use creep::{my_creep::MyCreep, role_services};
use international::{construction_site_services, global_request_ops, global_request_services};
use log::*;
use memory::game_memory::GameMemory;
use room::commune::{commune_services, my_room::MyRoom};
use screeps::{game, RoomName};
use state::{creep::CreepState, room::RoomState};
use wasm_bindgen::prelude::*;

use crate::{
    settings::Settings,
    state::game::{GameState},
};

mod constants;
mod creep;
mod international;
mod logging;
mod memory;
mod pathfinding;
mod room;
mod settings;
mod state;
mod structures;
mod utils;
mod init;

thread_local! {
    static GAME_STATE: RefCell<GameState> = RefCell::new(GameState::new());
    static SETTINGS: RefCell<Settings> = RefCell::new(Settings::new());
    static MEMORY: RefCell<GameMemory> = RefCell::new(GameMemory::load_from_memory_or_default());

    static ROOM_STATES: RefCell<HashMap<RoomName, RoomState>> = RefCell::new(HashMap::new());
    static MY_ROOM_STATES: RefCell<HashMap<RoomName, MyRoom>> = RefCell::new(HashMap::new());
    static CREEP_STATES: RefCell<HashMap<String, CreepState>> = RefCell::new(HashMap::new());
    static MY_CREEP_STATES: RefCell<HashMap<String, MyCreep>> = RefCell::new(HashMap::new());
}

#[wasm_bindgen]
pub fn init() {
    logging::setup_logger(LevelFilter::Trace);
    info!("Initializing...");
    GAME_STATE.with_borrow_mut(|game_state| {
        game_state.init_tick = game::time();
    });

    SETTINGS.with_borrow_mut(|settings| {
        settings.allies.insert(String::from("PandaMaster"));
    });
}

#[wasm_bindgen]
pub fn game_loop() {
    #[cfg(feature = "profile")]
    {
        screeps_timing::start_trace(Box::new(|| {
            (screeps::game::cpu::get_used() * 1000.0) as u64
        }));
    }

    let tick = game::time();
    let bucket = game::cpu::bucket();
    info!("Starting game tick {} with {} bucket", tick, bucket);

    trace!("this is a trace message");
    debug!("this is a debug message");
    info!("this is an info message");
    warn!("this is an important warning!");
    error!("this is a critical error");

    MEMORY.with_borrow_mut(|memory| {
        SETTINGS.with_borrow(|settings| {
            GAME_STATE.with_borrow_mut(|game_state| {
                
                loop_with_params(memory, game_state, settings);
                debug!("{:#?}", game_state);
            });
        });

        memory.write();
    });

    #[cfg(feature = "profile")]
    {
        let trace = screeps_timing::stop_trace();

        if let Ok(trace_output) = serde_json::to_string(&trace) {
            info!("{}", trace_output);
        }
    }

    info!("Ending tick {}: {:.3} CPU", tick, game::cpu::get_used());
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
fn loop_with_params(memory: &mut GameMemory, game_state: &mut GameState, settings: &Settings) {

    /* let mut my_creeps: HashMap<String, MyCreep> = HashMap::new();
    let mut my_creep_names: Vec<String> = Vec::new();

    let mut my_rooms: HashMap<RoomName, MyRoom> = HashMap::new();
    let mut my_room_names: Vec<RoomName> = Vec::new(); */

    game_state.update(memory);

    construction_site_services::manage_sites(game_state, memory);
    global_request_services::manage_requests(game_state, memory);
    commune_services::run_towers(game_state, memory);

    role_services::try_register_scout_targets(game_state, memory);

    role_services::try_scouts(game_state, memory);
}