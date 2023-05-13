use std::convert::{TryInto, TryFrom};
use std::mem::replace;

use crate::units::{Moves, Direction, Interval};

#[derive(Copy, Clone, PartialEq)]
enum ScaleMap {
    C,
    C_,
    D,
    D_,
    E,
    F,
    F_,
    G,
    G_,
    A,
    A_,
    B,
}

impl ScaleMap {
    pub fn distance(src: Self, dst: Self) -> Moves {
        let next: u8 = {
            let mut temp = src;
            let mut idx: u8 = 0;
            while (temp != dst) {
                temp.next();
                idx += 1 ;
            }
            idx
        };
        let prev: u8 = {
            let mut temp = src;
            let mut idx: u8 = 0;
            while (temp != dst) {
                temp.prev();
                idx += 1 ;
            }
            idx
        };
        if next > prev {
            return Moves {
                direction: Direction::Up,
                interval: Interval::try_from(next).unwrap()
            }
        } else {
            return Moves {
                direction: Direction::Down,
                interval: Interval::try_from(prev).unwrap()
            }
        }
    }

    pub fn next(&mut self) {
        replace(self, match self {
            Self::C => Self::C_,
            Self::C_ => Self::D,
            Self::D => Self::D_,
            Self::D_ => Self::E,
            Self::E => Self::F,
            Self::F => Self::F_,
            Self::F_ => Self::G,
            Self::G => Self::G_,
            Self::G_ => Self::A,
            Self::A => Self::A_,
            Self::A_ => Self::B,
            Self::B => Self::C 
        });
    }

    pub fn prev(&mut self) {
        replace(self, match self {
            Self::C => Self::B,
            Self::C_ => Self::C,
            Self::D => Self::C_,
            Self::D_ => Self::D,
            Self::E => Self::D_,
            Self::F => Self::E,
            Self::F_ => Self::F,
            Self::G => Self::F_,
            Self::G_ => Self::G,
            Self::A => Self::G_,
            Self::A_ => Self::A,
            Self::B => Self::A_ 
        });
    }

    fn move_with(&mut self, moves: Moves) {
        let steps: u8 = moves.interval as u8;
        // FIXME: Improve This
        for i in 0_u8..steps {
            match moves.direction {
                Direction::Up => {  
                    self.next()
                },
                Direction::Down => {
                    self.prev()
                }
            }
        }
    }
}
