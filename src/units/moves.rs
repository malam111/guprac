use crate::units::Interval;

pub enum Direction {
    Up,
    Down,
}

pub struct Move {
    interval: Interval,
    direction: Direction,
}