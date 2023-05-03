use crate::units::Interval;

pub enum Direction {
    Up,
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Up
    }
}

pub struct Moves {
    interval: Interval,
    direction: Direction,
}
