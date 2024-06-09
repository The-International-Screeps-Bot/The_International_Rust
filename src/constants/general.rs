use core::fmt;
use std::error::Error;

use screeps::Direction;

#[derive(PartialEq, Debug)]
/// Results intended to inform basic control flow
pub enum FlowResult {
    Stop,
    Continue,
}

impl Error for FlowResult {}

impl fmt::Display for FlowResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq)]
pub enum GeneralResult {
    Success,
    Fail,
}

#[derive(Debug)]
pub enum GeneralError {
    Fail,
}

impl Error for GeneralError {}

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

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

pub const DEFAULT_PLAINS_COST: u32 = 1;
pub const DEFAULT_ROAD_PLANNING_PLAINS_COST: u32 = 3;
pub const DEFAULT_ROAD_PLANNING_SWAMP_COST: u32 = 5;
pub const DEFAULT_CREEP_SWAMP_COST: u32 = 8;

pub const DEFAULT_DATA_DECAY: f32 = 0.99999;