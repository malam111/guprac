use crate::units::{Interval, Direction};

use super::{Scale};
use crate::units::Moves;
pub enum ScaleType {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,

    HarmonicMinor,
    MelodicMinor,

    PentatonicMajor,
    PentatonicMinor,

    Blues,
    WholeTone,
    Chromatic,
}

impl ScaleType {
    fn get_moves(&self, direction: Direction) -> Vec<Moves> {
        match self {
            Self::Ionian => vec![],
            _ => vec![]
        }
    }
}
