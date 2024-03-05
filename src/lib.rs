use log::*;
use memory::game_memory::GameMemory;
use screeps::game;
use wasm_bindgen::prelude::*;

use crate::memory::memory_ops::{MemoryOps, MEMORY};

mod logging;
mod settings;
mod pathfinding;
mod memory;
mod state;
mod utils;
mod constants;
mod creep;
mod room;
mod structures;
mod international;

#[wasm_bindgen]
pub fn init() {
    logging::setup_logger(LevelFilter::Trace);
    info!("Initializing...");
}

#[wasm_bindgen]
pub fn game_loop() {

    #[cfg(feature = "profile")]
    {
        screeps_timing::start_trace(Box::new(|| (screeps::game::cpu::get_used() * 1000.0) as u64));
    }

    let tick = game::time();
    let bucket = game::cpu::bucket();
    info!("Starting game tick {} with {} bucket", tick, bucket);

    trace!("this is a trace message");
    debug!("this is a debug message");
    info!("this is an info message");
    warn!("this is an important warning!");
    error!("this is a critical error");

    MEMORY.with(|mem_cell| {
        let mut memory = mem_cell.borrow_mut();

        with_memory(&mut memory);

        MemoryOps::write(&memory);
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

#[cfg(feature = "profile")]
fn with_memory(memory: &mut GameMemory) {
    use std::collections::HashSet;
    use state::game::GameState;

    use crate::settings::Settings;

    // This should only be called once

    let mut allies = HashSet::new();
    allies.insert(String::from("PandaMaster"));
    let settings = Settings::new(allies);

    let game_state = GameState::new();

    // 



}