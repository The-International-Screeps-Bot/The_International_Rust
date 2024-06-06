use wasm_bindgen::prelude::*;
use log::{info, LevelFilter};
use screeps::game;

use crate::{logging, memory::game_memory::GameMemory, settings::Settings, state::game::GameState, GAME_STATE, MEMORY, SETTINGS};

#[wasm_bindgen]
/// Runs every global reset
pub fn init() {
    logging::setup_logger(LevelFilter::Trace);

    MEMORY.with_borrow_mut(|memory| {
        SETTINGS.with_borrow_mut(|settings| {
            GAME_STATE.with_borrow_mut(|game_state| {
                
                init_with_params(memory, game_state, settings);
            });
        });
    });
}

fn init_with_params(memory: &mut GameMemory, game_state: &mut GameState, settings: &mut Settings) {

    init_settings(settings);
}

fn init_settings(settings: &mut Settings) {
    settings.allies.insert(String::from("PandaMaster"));
}