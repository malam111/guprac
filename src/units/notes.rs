use educe::Educe;
use std::{ops::{Deref, DerefMut}, marker::PhantomData};
use std::mem;
use std::convert::{TryInto, TryFrom};

use crate::{units::{Octave, Direction, Moves, Decorators, ScaleMap, Interval, Moveable}, /*scales::Scale*/};

#[derive(PartialEq, Debug, Copy, Clone, Educe)]
#[educe(Default)]
#[repr(i8)]
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

    pub fn distance(src: &Self, dst: &Self) -> Moves {
        todo!()
    }
}

impl From<RawNote> for ScaleMap {
    fn from(value: RawNote) -> Self {
        match value {
            RawNote::A => Self::A,
            RawNote::B => Self::B,
            RawNote::C => Self::C,
            RawNote::D => Self::D,
            RawNote::E => Self::E,
            RawNote::F => Self::F,
            RawNote::G => Self::G
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Scaled;

#[derive(PartialEq, Debug, Clone)]
pub struct NotScaled;

#[derive(PartialEq, Debug, Clone, Educe)]
#[educe(Default)]
pub struct Note<State> {
    raw: RawNote,
    decorators: Vec<Decorators>,
    octave: Octave,
    _state: PhantomData<State>,
}

impl<T> From<&Note<T>> for ScaleMap {
    fn from(value: &Note<T>) -> Self {
        let mut temp: ScaleMap = value.raw().into();
        for dec in value.dec().iter() {
            match *dec {
                Decorators::Sharp => {temp.next();},
                Decorators::Flat => {temp.prev();},
                _ => (),
            }
        }
        temp
    }
}

impl<State> Note<State> {
    //pub fn new(raw: RawNote) -> NoteBuilder<State> {
        //NoteBuilder::new(raw)
    //}

    pub fn raw(&self) -> RawNote {
        self.raw
    }
    
    pub fn dec(&self) -> &[Decorators] {
        &self.decorators
    }

}

impl Note<Scaled> {
    // FIXME: check times
    fn move_with(&mut self, moves: Moves, times: u8) {
        // TODO: adjust target with scale target
        let mut target = self.raw.clone();
        for _ in 0..times {
            match moves.direction {
                Direction::Up => target.next(),
                Direction::Down => target.prev()
            };
        }

        // scale_map, src after applied decor
        // dst, target note, the next note in the current scale context
        let mut scale_map = ScaleMap::from(&*self);
        scale_map.move_with(moves);
        let dst = ScaleMap::from(target);
        let distance = ScaleMap::distance(&scale_map, &dst);

        *self = Self::from_scale_map(scale_map, target, distance);
    }

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
        self.move_with(0_i8.try_into().unwrap(), 0);
    }
}


impl Note<NotScaled> {
    fn move_with(&mut self, moves: Moves) {
        // FIXME: Improve?
        let mut scale_map = ScaleMap::from(&*self);
        scale_map.move_with(moves);
        *self = scale_map.into();
    }

    pub fn resolve(&mut self) {
        self.move_with(0_i8.try_into().unwrap());
    }

}

impl From<ScaleMap> for Note<NotScaled> {
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn note_to_scalemap_sharp_test() {
        let note = Note::<NotScaled> {
            decorators: vec!(
                Decorators::Sharp, 
                Decorators::Sharp, 
                Decorators::Sharp
            ),
            ..Note::default()
        };
        let scalemap: ScaleMap = (&note).into();
         
        assert_eq!(ScaleMap::D_, scalemap);
    }

    #[test]
    fn note_to_scalemap_flat_test() {
        let note = Note::<Scaled> {
            raw: RawNote::D,
            decorators: vec!(
                Decorators::Flat, 
                Decorators::Flat, 
            ),
            ..Note::default()
        };
        let scalemap: ScaleMap = (&note).into();
         
        assert_eq!(ScaleMap::C, scalemap);
    }

    #[test]
    fn note_cs_to_f_not_scaled_test() {
        let mut note_right = Note::<NotScaled> {
            raw: RawNote::C,
            decorators: vec!(
                Decorators::Sharp, 
            ),
            ..Note::default() 
        };
        let note_left = Note::<NotScaled> {
            raw: RawNote::F,
            decorators: vec!(
                Decorators::Natural, 
            ),
            ..Note::default() 
        };
        let moves = Moves::try_from(4).unwrap();
        note_right.move_with(moves);
        assert_eq!(note_left, note_right);
    }

    #[test]
    fn note_f_to_cs_not_scaled_test() {
        let mut note_right = Note::<NotScaled> {
            raw: RawNote::F,
            decorators: vec!(
                Decorators::Natural, 
            ),
            ..Note::default() 
        };
        let note_left = Note::<NotScaled> {
            raw: RawNote::C,
            decorators: vec!(
                Decorators::Sharp, 
            ),
            ..Note::default() 
        };
        let moves = Moves::try_from(-4).unwrap();
        note_right.move_with(moves);
        assert_eq!(note_left, note_right);
    }

    #[test]
    fn note_b_to_css_sharp_scaled_test() {
        let note_left = Note::<Scaled> {
            raw: RawNote::D,
            decorators: vec!(
                Decorators::Sharp,
                Decorators::Sharp,
            ),
            ..Note::default()
        };

        let mut note_right = Note::<Scaled> {
            raw: RawNote::B,
            decorators: vec!(
                Decorators::Natural,
            ),
            ..Note::default()
        };
        let moves = Moves::try_from(5).unwrap(); 
        note_right.move_with(moves, 2);
        assert_eq!(note_left, note_right);
    }


    #[test]
    fn note_e_to_bbb_sharp_scaled_test() {
        let note_left = Note::<Scaled> {
            raw: RawNote::C,
            decorators: vec!(
                Decorators::Flat,
                Decorators::Flat,
            ),
            ..Note::default()
        };

        let mut note_right = Note::<Scaled> {
            raw: RawNote::E,
            decorators: vec!(
                Decorators::Natural,
            ),
            ..Note::default()
        };
        let moves = Moves::try_from(-6).unwrap(); 
        note_right.move_with(moves, 2);
        assert_eq!(note_left, note_right);
    }
}
