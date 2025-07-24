use screeps::{Position, ROOM_SIZE};

pub fn get_adjacent_positions(pos: &Position) -> Vec<Position> {

    let mut adjacent_positions: Vec<Position> = Vec::new();
    let pos_world_coords = pos.world_coords();

    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }

            if !is_xy_in_room(x as u8, y as u8) {
                continue;
            }

            adjacent_positions.push(Position::from_world_coords(pos_world_coords.0 + x, pos_world_coords.1 + y));
        }
    }
    adjacent_positions
}

pub fn get_adjacent_positions_conditional(pos: &Position, condition: &dyn Fn(&Position) -> bool) -> Vec<Position> {

    let mut adjacent_positions: Vec<Position> = Vec::new();
    let pos_world_coords = pos.world_coords();

    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }

            if !is_xy_in_room(x as u8, y as u8) {
                continue;
            }

            let new_pos = Position::from_world_coords(pos_world_coords.0 + x, pos_world_coords.1 + y);
            if condition(&new_pos) {
                adjacent_positions.push(new_pos);
            }
        }
    }
    adjacent_positions
}

/// Potentially unsafe if bounds are not checked beforehand
pub fn get_adjacent_positions_unbounded(pos: &Position) -> Vec<Position> {

    let mut adjacent_positions: Vec<Position> = Vec::new();
    let pos_world_coords = pos.world_coords();

    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }

            adjacent_positions.push(Position::from_world_coords(pos_world_coords.0 + x, pos_world_coords.1 + y));
        }
    }
    adjacent_positions
}

pub fn for_positions_in_range_in_room(pos: &Position, range: u8, callback: &dyn Fn(&Position)) {

    let world_coords = pos.world_coords();
    let pos_x = pos.x().0;
    let pos_y = pos.y().0;

    for x in (pos_x - range)..=(range + pos_x) {
        for y in (pos_y - range)..=(pos_y + range) {

            if !is_xy_in_room(x, y) {
                continue;
            }
            callback(&Position::from_world_coords(pos.world_coords().0 + x as i32, pos.world_coords().1 + y as i32));
        }
    }
}

pub fn get_positions_in_range_in_room(pos: &Position, range: u8) -> Vec<Position> {

    let mut positions = Vec::new();

    let world_coords = pos.world_coords();
    let pos_x = pos.x().0;
    let pos_y = pos.y().0;

    for x in (pos_x - range)..=(pos_x + range) {
        for y in (pos_y - range)..=(pos_y + range) {

            if !is_xy_in_room(x, y) {
                continue;
            }
            
            let new_pos = Position::from_world_coords(pos.world_coords().0 + x as i32, pos.world_coords().1 + y as i32);
            positions.push(new_pos);
        }
    }

    positions
}

pub fn is_xy_in_room(x: u8, y: u8) -> bool {

    x < ROOM_SIZE && y < ROOM_SIZE
}

pub fn is_xy_exit(x: u8, y: u8) -> bool {

    x == 0 || y == 0 || x == ROOM_SIZE - 1 || y == ROOM_SIZE - 1
}