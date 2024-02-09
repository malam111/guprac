
use std::ops::{Deref, DerefMut};

use crate::units::{Interval, Direction};
//use super::{Scale};
use crate::units::Moves;

#[derive(Educe)]
#[educe(Default)]
pub enum ScaleType {
    #[educe(Default)]
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
    pub fn get_moves(&self, direction: Direction) -> ScaleMoves {
        ScaleMoves (
        match self {
            Self::Ionian => Moves::from_vec(vec![2, 2, 1, 2, 2, 2, 1]).unwrap(),
            _ => vec![]
        }
        )
    }
}

pub struct ScaleMoves(Vec<Moves>);

impl Deref for ScaleMoves {
    type Target = Vec<Moves>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ScaleMoves {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
