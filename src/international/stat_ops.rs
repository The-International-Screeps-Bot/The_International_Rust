use crate::state::game::GameState;

pub struct StatOps;

impl StatOps {
    pub fn find_combined_rcl(game_state: &GameState) -> u32 {
        let mut combined_rcl: u32 = 0;

        let communes = &game_state.communes;
        for room_name in communes {

            let Some(room) = game_state.rooms.get(room_name) else {
                continue;
            };

            let Some(controller) = room.controller() else {
                continue;
            };

            combined_rcl += controller.level() as u32;
        }

        combined_rcl
    }
}