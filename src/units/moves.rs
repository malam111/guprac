use std::convert::TryFrom;
use std::error;
use crate::units::{Interval, ErrInterval};

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

#[derive(Debug, Default)]
pub struct ErrMoves;

impl Moves {
    pub fn from_vec(value: Vec<i8>) -> Result<Vec<Moves>, ErrMoves> {
        let val_len = value.len();
        let moves: Vec<Result<Moves, ErrMoves>> = value.into_iter().map(|step|  { let step: Result<Moves, _> = step.try_into(); return step }).filter(|x| x.is_ok()).collect();
        if moves.len() == val_len {
            return Ok(moves.into_iter().map(Result::unwrap).collect::<Vec<Moves>>());
        }
        Err(ErrMoves::default()) 
    }
}


impl TryFrom<i8> for Moves {
    type Error = ErrMoves;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let result : Result<Interval, ErrInterval>= value.try_into();
        if let Ok(interval) = result {

            Ok(Moves {

                interval,
                direction: if (value.is_positive()) {Direction::Up} else {Direction::Down}
            })
        } else {
            Err(Self::Error::default())
        }
    }
}
