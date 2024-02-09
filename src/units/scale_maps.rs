use std::convert::{TryInto, TryFrom};
use std::mem::replace;

//use crate::scales::Scale;
use crate::units::{/*Note, RawNote,*/ Moves, Direction, Interval, Octave, Moveable};

use super::Decorators;

// TODO: ScaleMap Builder for interface octave
#[derive(Copy, Clone, PartialEq)]
pub enum ScaleMap {
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
    pub fn distance(src: &Self, dst: &Self) -> Moves {
        let next: u8 = {
            let mut temp = src.clone();
            let mut idx: u8 = 0;
            while &temp != dst {
                temp.next();
                idx += 1 ;
            }
            idx
        };
        let prev: u8 = {
            let mut temp = src.clone();
            let mut idx: u8 = 0;
            while &temp != dst {
                temp.prev();
                idx += 1 ;
            }
            idx
        };
        if next < prev {
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

    // return Some if octave changes
    pub fn next(&mut self) -> Option<()> {
        let mut change = None;
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
            Self::B => {change = Some(()); Self::C}
        });
        change
    }

    // return Some if octave changes
    pub fn prev(&mut self) -> Option<()> {
        let mut change = None;
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
            Self::B => {change = Some(()); Self::A_}
        });
        change
    }

    pub fn move_with(&mut self, moves: Moves) -> Option<()> {
        let steps: u8 = moves.interval as u8;
        let mut change = None;
        // FIXME: Improve This
        for i in 0_u8..steps {
            match moves.direction {
                Direction::Up => {  
                    match self.next() {
                        Some(ret) => { change = Some(ret); }
                        _ => {} 
                    }
                },
                Direction::Down => {
                    match self.prev() {
                        Some(ret) => { change = Some(ret); }
                        _ => {} 
                    }
                }
            }
        }
        change
    }
}

//impl From<RawNote> for ScaleMap {
//    fn from(value: RawNote) -> Self {
//        match value {
//            RawNote::A => Self::A,
//            RawNote::B => Self::B,
//            RawNote::C => Self::C,
//            RawNote::D => Self::D,
//            RawNote::E => Self::E,
//            RawNote::F => Self::F,
//            RawNote::G => Self::G
//        }
//    }
//}

//impl<T: Moveable> From<&Note<T>> for ScaleMap {
//    fn from(value: &Note<T>) -> Self {
//        let mut temp = ScaleMap::from(value.raw()) ;
//        for dec in value.dec().iter() {
//            match *dec {
//                Decorators::Sharp => {temp.next();},
//                Decorators::Flat => {temp.prev();},
//                _ => (),
//            }
//        }
//        temp
//    }
//}
//
//struct ScaleMapBuilder {
//    inner: ScaleMap,
//    octave: Octave,
//}
//
//impl ScaleMapBuilder {
//
//}

#[cfg(test)]
mod test {
    use super::*;

    #[test] 
    fn scale_map_builder_test() {
        assert_eq!(
            ScaleMap::distance(&ScaleMap::C, &ScaleMap::A_),
            Moves::try_from(-2).unwrap()
        );
    }
}
