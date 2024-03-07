use core::fmt;
use std::error::Error;

use screeps::Direction;

pub enum GeneralResult {
    Success,
    Fail,
}

#[derive(Debug)]
pub enum GeneralError {
    Fail,
}

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GeneralError {}

pub const MIN_CONSTRUCTION_SITE_AGE: u32 = 20000;
/// How much each point of progress is worth in terms of age
pub const CONSTRUCTION_PROGRESS_AGE_MULTIPLIER: u32 = 5;

pub const DIRECTIONS: [Direction; 8] = [
    Direction::Top,
    Direction::TopRight,
    Direction::Right,
    Direction::BottomRight,
    Direction::Bottom,
    Direction::BottomLeft,
    Direction::Left,
    Direction::TopLeft,
];