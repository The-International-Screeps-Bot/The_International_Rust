use log::*;
use screeps::game;
use wasm_bindgen::prelude::*;

mod logging;

#[wasm_bindgen]
pub fn init() {
    logging::setup_logger(LevelFilter::Trace);
    info!("Initializing...");
}

#[wasm_bindgen]
pub fn game_loop() {
    let tick = game::time();
    let bucket = game::cpu::bucket();
    info!("Starting game tick {} with {} bucket", tick, bucket);

    trace!("this is a trace message");
    debug!("this is a debug message");
    info!("this is an info message");
    warn!("this is an important warning!");
    error!("this is a critical error");

    info!("Ending tick {}: {:.3} CPU", tick, game::cpu::get_used());
}
