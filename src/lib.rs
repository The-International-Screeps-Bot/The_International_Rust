use core::cell::RefCell;

use log::*;
use memory::game_memory::GameMemory;
use screeps::game;
use wasm_bindgen::prelude::*;

use crate::{
    memory::memory_ops::{MemoryOps, MEMORY},
    settings::Settings,
    state::game::{GameState, GameStateOps},
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

thread_local! {
    static GAME_STATE: RefCell<GameState> = RefCell::new(GameState::default());
    static SETTINGS: RefCell<Settings> = RefCell::new(Settings::default());
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
        with_memory(memory);

        MemoryOps::write(memory);
    });

    #[cfg(feature = "profile")]
    {
        let trace = screeps_timing::stop_trace();

        if let Some(trace_output) = serde_json::to_string(&trace).ok() {
            info!("{}", trace_output);
        }
    }

    info!("Ending tick {}: {:.3} CPU", tick, game::cpu::get_used());
}

#[cfg_attr(feature = "profile", screeps_timing_annotate::timing)]
fn with_memory(memory: &mut GameMemory) {
    GAME_STATE.with_borrow_mut(|game_state| {
        GameStateOps::update(game_state);
        debug!("{:#?}", game_state);
    });
}
