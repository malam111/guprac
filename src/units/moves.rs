use crate::units::Interval;

pub enum Direction {
    Up,
    Down,
}

pub struct Moves {
    interval: Interval,
    direction: Direction,
}