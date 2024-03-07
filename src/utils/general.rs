use js_sys::Math::max;
use screeps::{game, Position};

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
        (pos1.world_x().abs_diff(pos2.world_x()).pow(2) + (pos1.world_y().abs_diff(pos2.world_y()).pow(2))).pow(1/2)
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
    pub fn for_adjacent_positions(position: Position, operation: &dyn Fn(&Position) -> ()) {
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
            }
        }
    }
}
