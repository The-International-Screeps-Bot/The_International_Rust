use screeps::{Position, RoomVisual};

pub fn visualize_path(path: &[Position]) {
    let first = path.first().unwrap();

    let room_visual = RoomVisual::new(Some(first.room_name()));
    let points = path
        .iter()
        .map(|pos| (pos.x().u8() as f32, pos.y().u8() as f32))
        .collect();
    room_visual.poly(points, None);
}
