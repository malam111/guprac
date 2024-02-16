use std::{ops::{Deref, DerefMut}, marker::PhantomData};
use std::mem;
use std::convert::{TryInto, TryFrom};
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use crate::{units::{Octave, Direction, Moves, Decorators, ScaleMap, Interval, Moveable}, /*scales::Scale*/};

use educe::Educe;

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
        let diff: i8 = *src as i8 - *dst as i8;
        Moves {
            interval: diff.abs().try_into().unwrap(),
            direction: if diff > 0 {
                    Direction::Up 
                } else {
                    Direction::Down 
                }
        }
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

impl fmt::Display for RawNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            RawNote::A => "A",
            RawNote::B => "B",
            RawNote::C => "C",
            RawNote::D => "D",
            RawNote::E => "E",
            RawNote::F => "F",
            RawNote::G => "G"
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Scaled;

#[derive(PartialEq, Debug, Clone)]
pub struct NotScaled;

#[derive(PartialEq, Debug, Clone, Educe)]
#[educe(Default)]
pub struct Note<State> {
    // FIXME: remove pub, add new()/builder
    raw: RawNote,
    octave: Octave,
    decorators: Vec<Decorators>,
    _state: PhantomData<State>,
}

impl<T> From<&Note<T>> for ScaleMap {
    fn from(value: &Note<T>) -> Self {
        let mut temp: ScaleMap = value.raw.into();
        for dec in value.decorators.iter() {
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
    pub fn new() -> NoteBuilder<State> {
        NoteBuilder::new()
    }

    pub fn raw(&self) -> RawNote {
        self.raw
    }
    
    pub fn dec(&self) -> &[Decorators] {
        &self.decorators
    }

}

impl<T> fmt::Display for Note<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display = String::new();
        display.push_str(self.raw.to_string().as_ref());
        for dec in self.decorators.iter() {
            display.push_str(dec.to_string().as_ref());
        }
        display.push_str(self.octave.to_string().as_ref());
        write!(f, "{}", display)
    }
}

impl Note<Scaled> {
    // FIXME: check times
    pub fn move_with(&mut self, moves: Moves, times: u8) {
        let mut target = self.raw.clone();
        for _ in 0..times {
            match moves.direction {
                Direction::Up => target.next(),
                Direction::Down => target.prev()
            };
        }

        let mut scale_map = ScaleMap::from(&*self);
        let target_scale_map = Rc::new(RefCell::new(scale_map.clone()));
        let target_octave = Rc::new(RefCell::new(self.octave));
        let move_scale: Box<dyn Fn() -> Option<()>> = match moves.direction {
            Direction::Up => Box::new(|| 
                (*target_scale_map.clone().borrow_mut()).next()
            ),
            Direction::Down => Box::new(|| 
                (*target_scale_map.clone().borrow_mut()).prev()
            )
        };
        let move_octave: Box<dyn Fn() -> ()> = match moves.direction {
            Direction::Up => 
                Box::new(|| 
                    *target_octave
                        .clone()
                        .borrow_mut() = (target_octave.clone()).take().next()
            ),
            Direction::Down => 
                Box::new(|| 
                    *target_octave
                        .clone()
                        .borrow_mut() = (target_octave.clone()).take().prev()
            )
        };
        let dst = ScaleMap::from(target);
        scale_map.move_with(moves);
        loop {
            if *target_scale_map.clone().borrow() == dst {
                break;
            }
            if let Some(_) = (*move_scale)() {
                (*move_octave)();
            }
        }
        let distance = ScaleMap::distance(&scale_map, &dst);

        *self = Self::from_scale_map(scale_map, target, distance, target_octave.clone().take());
    }

    // todo.1. remove target_octave 
    fn from_scale_map(scale_map: ScaleMap, dst: RawNote, distance: Moves, octave: Octave) -> Self {    
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
            octave,
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

#[derive(Educe, Clone)]
#[educe(Default)]
pub struct NoteBuilder<State> {
    raw: RawNote,
    decorators: Vec<Decorators>,
    octave: Octave,
    _state: PhantomData<State>,
}

impl<State> NoteBuilder<State> {
    
    fn new() -> Self {
        NoteBuilder {
            ..Default::default()
        }
    }

    //fn from_string<S: AsRef<&str>>(text: S) -> Self {
    //    todo!();
    //}

    pub fn raw(mut self, raw: RawNote) -> Self {
        self.raw = raw; 
        self
    }

    pub fn decorators(mut self, decorators: Vec<Decorators>) -> Self {
        self.decorators.extend(decorators);
        self
    }

    pub fn decorator(mut self, decorator: Decorators) -> Self {
        self.decorators.push(decorator);
        self
    }

    pub fn octave(mut self, octave: Octave) -> Self {
        self.octave = octave;
        self
    }

    pub fn scaled(mut self) -> NoteBuilder<Scaled> {
        NoteBuilder {
            _state: PhantomData::<Scaled>,
            raw: self.raw,
            decorators: self.decorators,
            octave: self.octave,
        }
    }

    pub fn noscaled(mut self) -> NoteBuilder<NotScaled> {
        NoteBuilder {
            _state: PhantomData::<NotScaled>,
            raw: self.raw,
            decorators: self.decorators,
            octave: self.octave,
        }
    }


}

impl NoteBuilder<Scaled> {

    pub fn build(&mut self) -> Note<Scaled> {
        if self.decorators.len() == 0 {
            self.decorators.push(Decorators::Natural);
        }
        Note {
            raw: self.raw.clone(),
            decorators: self.decorators.clone(),
            octave: self.octave.clone(),
            _state: self._state.clone()
        }
    }
}

impl NoteBuilder<NotScaled> {

    pub fn build(&mut self) -> Note<NotScaled> {
        if self.decorators.len() == 0 {
            self.decorators.push(Decorators::Natural);
        }
        Note {
            raw: self.raw.clone(),
            decorators: self.decorators.clone(),
            octave: self.octave.clone(),
            _state: self._state.clone()
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn note_display_test() {
        let left = "C#3";
        let right = Note::<NotScaled>::new().raw(RawNote::C).decorator(Decorators::Sharp).octave(Octave::O3).build();
        assert_eq!(left, format!("{}", right));
    }

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
            octave: Octave::O4,
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

    #[test]
    fn note_rawnote_distance() {
        let moves = Moves {
            interval: 2_i8.try_into().unwrap(),
            direction: Direction::Down
        };
        assert_eq!(RawNote::distance(&RawNote::E, &RawNote::G), moves);
    }
}
