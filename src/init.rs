use log::{debug, info, LevelFilter};
use screeps::game;
use wasm_bindgen::prelude::*;

use crate::{
    init_settings::init_settings, international::stat_services, logging,
    memory::game_memory::GameMemory, settings::Settings, state::game::GameState, GAME_STATE,
    MEMORY, SETTINGS,
};

/// Runs every global reset
pub fn init() {
    debug!("Running init");

    MEMORY.with_borrow_mut(|memory| {
        SETTINGS.with_borrow_mut(|settings| {
            GAME_STATE.with_borrow_mut(|game_state| {
                init_with_params(memory, game_state, settings);
            });
        });
    });

    debug!("Completed init");
}

fn init_with_params(memory: &mut GameMemory, game_state: &mut GameState, settings: &mut Settings) {
    logging::setup_logging(settings.log_filter);

    stat_services::tick_update(game_state, memory);
    init_settings(settings, game_state);
}
