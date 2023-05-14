use educe::Educe;
use std::{ops::{Deref, DerefMut}, marker::PhantomData};
use std::mem;

use crate::{units::{Octave, Direction, Moves, Decorators, ScaleMap}, scales::Scale};

#[derive(Debug, Copy, Clone, Educe)]
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

#[derive(Debug)]
pub struct Contexted;
#[derive(Debug)]
pub struct NoContexted;

#[derive(Debug, Clone, Educe)]
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

    fn resolve(&self) {
        
    }
}

impl Note<Contexted> {
    pub fn move_with(&mut self, moves: Moves) {
        let mut scale_map = ScaleMap::from(&*self);
        let mut target = self.raw.clone();
        match moves.direction {
            Direction::Up => target.next(),
            Direction::Down => target.prev()
        };
        scale_map.move_with(moves);
        let dst = ScaleMap::from(target);
        let distance = ScaleMap::distance(&scale_map, &dst);
        *self = Self::from_scale_map(scale_map, target, distance);
    }

    fn from_scale_map(scale_map: ScaleMap, dst: RawNote, distance: Moves) -> Self {    
        let mut decorators = Vec::<Decorators>::new();
        let decor = if distance.direction == Direction::Up { Decorators::Flat } else {Decorators::Sharp};
        for idx in 0_u8..distance.interval as u8 {
            decorators.push(decor);
        }
        Self {
            raw: dst,
            decorators,
            ..Default::default()
        }
    }
}


impl Note<NoContexted> {
    pub fn move_with(&mut self, moves: Moves) {
        // FIXME: Improve?
        let mut scale_map = ScaleMap::from(&*self);
        scale_map.move_with(moves);
        *self = scale_map.into();
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

    fn decorators(&mut self, decorator: Vec<Decorators>) -> &mut Self {
        self.decorators.extend(decorator);
        self
    }

    fn decorator(&mut self, decorator: Decorators) -> &mut Self {
        self.decorators.push(decorator);
        self
    }

    fn build(mut self) -> Note<T> {
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
        println!("{:?}", note_c);
    }

    #[test]
    fn c_to_d_scaled() {
        let mut note_c = Note::<Contexted>::new(RawNote::C).build();
        let mov =  Moves {
            direction: Direction::Up, 
            interval: crate::units::Interval::Per4};
        note_c.move_with(mov);
        println!("{:?}", note_c);
    }
}