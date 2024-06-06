use js_sys::Math::max;
use screeps::{game, OwnedStructureProperties, Position};

pub struct GeneralUtils;

impl GeneralUtils {
    pub fn is_tick_interval(interval: u32) -> bool {
        let tick = game::time();

        tick % interval == 0
    }

    pub fn is_exit(pos: Position) -> bool {
        // TODO
        false
    }

    pub fn pos_range(pos1: &Position, pos2: &Position) -> u32 {
        pos1.world_x().abs_diff(pos2.world_x()) + pos1.world_y().abs_diff(pos2.world_y())
    }

    pub fn pos_range_euc(pos1: &Position, pos2: &Position) -> u32 {
        (pos1.world_x().abs_diff(pos2.world_x()).pow(2)
            + (pos1.world_y().abs_diff(pos2.world_y()).pow(2)))
        .pow(1 / 2)
    }

    // Is somewhat inaccurate
    pub fn round_to<T, U>(number: T, decimals: U) -> T
    where
        T: From<f64> + Into<f64>,
        U: Into<f64>,
    {
        let number_f64: f64 = number.into();

        let multiplier = 10u32.pow(decimals.into() as u32);
        let multiplier_f64: f64 = multiplier.into();

        T::from((number_f64 * multiplier_f64).round() / multiplier_f64)
    }

    /// Currently dysfunctional
    pub fn for_adjacent_positions(position: Position, operation: &dyn Fn(&Position)) {
        let (pos_x, pos_y) = position.coords_signed();

        let mut x = pos_x - 1;
        while x < pos_x + 1 {
            let mut y = pos_y - 1;

            while y < pos_y + 1 {
                if x == pos_x && y == pos_y {
                    continue;
                }

                let adjacent_pos = Position::from_world_coords(x.into(), y.into());
                operation(&adjacent_pos);

                y += 1;
            }

            x += 1;
        }
    }

    pub fn me() -> Result<String, ()> {
        let js_rooms = screeps::game::rooms();

        for room_name in js_rooms.keys() {
            let Some(room) = js_rooms.get(room_name) else {
                continue;
            };

            let Some(controller) = room.controller() else {
                continue;
            };

            if !controller.my() {
                continue;
            };

            let Some(owner) = controller.owner() else {
                continue
            };

            return Ok(owner.username())
        }

        Err(())
    }

    pub fn find_index_with_lowest_score<T>(iter: &[T], f: &dyn Fn(&T) -> u32/* for<'a> fn(val: &'a T) -> u32 *//* impl Fn(val: T) -> number | false */) -> (u32, u32) {
        let (mut lowest_score, mut index) = (u32::MAX, 0);
    
        for (i, val) in iter.iter().enumerate() {
            let val_score = f(val);
    
            if val_score < lowest_score {
                (lowest_score, index) = (val_score, i as u32);
            }
        }
    
        (lowest_score, index)
    }
}