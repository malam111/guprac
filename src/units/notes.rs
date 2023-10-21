use educe::Educe;
use std::{ops::{Deref, DerefMut}, marker::PhantomData};
use std::mem;
use std::convert::TryInto;

use crate::{units::{Octave, Direction, Moves, Decorators, ScaleMap, Interval, Moveable}, scales::Scale};

#[derive(PartialEq, Debug, Copy, Clone, Educe)]
#[educe(Default)]
pub enum RawNote {
    #[educe(Default)]
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl RawNote {
    pub fn next(&mut self) {
        mem::replace(self, match self {
            Self::C => Self::D,
            Self::D => Self::E,
            Self::E => Self::F,
            Self::F => Self::G,
            Self::G => Self::A,
            Self::A => Self::B,
            Self::B => Self::C 
        });
    }

    pub fn prev(&mut self) {
        mem::replace(self, match self {
            Self::C => Self::B,
            Self::D => Self::C,
            Self::E => Self::D,
            Self::F => Self::E,
            Self::G => Self::F,
            Self::A => Self::G,
            Self::B => Self::A 
        });
    }
}

pub trait TempMove {}

#[derive(PartialEq, Debug, Clone)]
pub struct Contexted;

impl TempMove for Contexted {}
#[derive(PartialEq, Debug, Clone)]
pub struct NoContexted;

impl TempMove for NoContexted {}

#[derive(PartialEq, Debug, Clone, Educe)]
#[educe(Default)]
pub struct Note<State> {
    raw: RawNote,
    decorators: Vec<Decorators>,
    octave: Octave,
    _state: PhantomData<State>,
}

impl<State> Note<State> {
    pub fn new(raw: RawNote) -> NoteBuilder<State> {
        NoteBuilder::new(raw)
    }

    pub fn raw(&self) -> RawNote {
        self.raw
    }
    
    pub fn dec(&self) -> &[Decorators] {
        &self.decorators
    }

}

impl Moveable for Note<Contexted> {
    fn move_with(&mut self, moves: Moves) {
        let mut target = self.raw.clone();
        match moves.direction {
            Direction::Up => target.next(),
            Direction::Down => target.prev()
        };

        // scale_map, src after applied decor
        // dst, target note, the next note in the current scale context
        let mut scale_map = ScaleMap::from(&*self);
        scale_map.move_with(moves);
        let dst = ScaleMap::from(target);
        let distance = ScaleMap::distance(&scale_map, &dst);

        *self = Self::from_scale_map(scale_map, target, distance);
    }
}

impl Note<Contexted> {

    fn from_scale_map(scale_map: ScaleMap, dst: RawNote, distance: Moves) -> Self {    
        let mut decorators = Vec::<Decorators>::new();
        let mut decor = if distance.direction == Direction::Up { 
            Decorators::Flat 
        } else {
            Decorators::Sharp
        };

        for idx in 0_u8..distance.interval as u8 {
            decorators.push(decor);
        }
        if distance.interval == Interval::Per1 {
            decor = Decorators::Natural;   
            decorators.push(decor);
        }
        Self {
            raw: dst,
            decorators,
            ..Default::default()
        }
    }

    pub fn resolve(&mut self) {
        self.move_with(0_i8.try_into().unwrap());
    }
}


impl Moveable for Note<NoContexted> {
    fn move_with(&mut self, moves: Moves) {
        // FIXME: Improve?
        let mut scale_map = ScaleMap::from(&*self);
        scale_map.move_with(moves);
        *self = scale_map.into();
    }
}

impl Note<NoContexted> {
    pub fn resolve(&mut self) {
        self.move_with(0_i8.try_into().unwrap());
    }

}

impl From<ScaleMap> for Note<NoContexted> {
    fn from(value: ScaleMap) -> Self {
        let mut sharp = false;
        let raw: RawNote = match value {
                ScaleMap::A => RawNote::A,
                ScaleMap::A_ => {sharp = true; RawNote::A},
                ScaleMap::B => RawNote::B,
                ScaleMap::C => RawNote::C,
                ScaleMap::C_ => {sharp = true; RawNote::C},
                ScaleMap::D => RawNote::D,
                ScaleMap::D_ => {sharp = true; RawNote::D},
                ScaleMap::E => RawNote::E,
                ScaleMap::F => RawNote::F,
                ScaleMap::F_ => {sharp = true; RawNote::F},
                ScaleMap::G => RawNote::G,
                ScaleMap::G_ => {sharp = true; RawNote::G},
        };
        Note {
            raw,
            decorators: vec![if sharp {Decorators::Sharp} else { Decorators::Natural }],
            ..Default::default()
        }
    }
}

pub struct NoteBuilder<T> (Note<T>);

impl<T> Deref for NoteBuilder<T> {
    type Target = Note<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for NoteBuilder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> NoteBuilder<T> {
    
    fn new(raw: RawNote) -> Self {
        NoteBuilder(Note {raw, ..Note::default() })
    }

    pub fn decorators(mut self, decorator: Vec<Decorators>) -> Self {
        self.decorators.extend(decorator);
        self
    }

    pub fn decorator(mut self, decorator: Decorators) -> Self {
        self.decorators.push(decorator);
        self
    }

    pub fn build(mut self) -> Note<T> {
        if self.dec().len() == 0 {
            self.decorators.push(Decorators::Natural);
        }
        self.0
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn c_to_e() {
        let mut note_c = Note::<NoContexted>::new(RawNote::C).build();
        let mov =  Moves {
            direction: Direction::Up, 
            interval: crate::units::Interval::Maj3};
        note_c.move_with(mov);
        //println!("{:?}", note_c);
    }

    #[test]
    fn c_to_d_scaled() {
        let mut note_c = Note::<Contexted>::new(RawNote::C).build();
        let mov =  Moves {
            direction: Direction::Up, 
            interval: crate::units::Interval::Maj2};
        note_c.move_with(mov);
        //note_c.move_with((-2_i8).try_into().unwrap());
        //note_c.move_with((2_i8).try_into().unwrap());

        
        // note_c.move_with(mov!(M2));
        // note_c.moves_with(movs![M2, m2, P4])

        //println!("{:?}", note_c);
    }
}
