use std::convert::{TryFrom, TryInto};
use std::error;
use crate::units::{Interval, ErrInterval};
use rand::Rng;


pub trait Moveable {

    fn move_with(&mut self, moves: Moves);
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Up
    }
}

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Moves {
    pub interval: Interval,
    pub direction: Direction,
}


#[derive(Debug, Default)]
pub struct ErrMoves;

impl Moves {
    pub fn from_vec(value: Vec<i8>) -> Result<Vec<Self>, ErrMoves> {
        let val_len = value.len();
        let moves: Vec<Result<Moves, ErrMoves>> = value.into_iter()
                                                    .map(|step|  { 
                                                        let step: Result<Moves, _> = Moves::try_from(step); 
                                                        return step 
                                                    })
                                                    .filter(|x| x.is_ok())
                                                    .collect();
        if moves.len() == val_len {
            return Ok(moves.into_iter().map(Result::unwrap).collect::<Vec<Moves>>());
        }
        Err(ErrMoves::default()) 
    }

    // not random yet
    pub fn rand() -> Self {
        rand::thread_rng().gen_range(-12_i8..=12).try_into().unwrap()
    }
}


impl TryFrom<i8> for Moves {
    type Error = ErrMoves;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let result : Result<Interval, ErrInterval> = Interval::try_from(value);
        if let Ok(interval) = result {

            Ok(Moves {

                interval,
                direction: if value.is_positive() {Direction::Up} else {Direction::Down}
            })
        } else {
            Err(Self::Error::default())
        }
    }
}
