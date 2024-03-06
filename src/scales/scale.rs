use std::ops::{Deref, DerefMut};
use std::convert::{TryInto};
use std::iter::FromIterator;
use std::fmt;

use super::ScaleType;
use crate::units::{Note, Scaled, Octave, Direction, RawNote, Decorators, Moves};

#[derive(Debug)]
pub enum ScaleItem {
    Note(Note<Scaled>),
    Degree(Degree),
}

impl fmt::Display for ScaleItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScaleItem::Note(note) => { write!(f, "{}", note) }
            ScaleItem::Degree(degree) => { write!(f, "{}", degree) }

        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Scale {
    key: Note<Scaled>,
    scale_type: ScaleType,
    direction: Direction,
    bounded: bool,
    low_octave: Octave,
    high_octave: Octave,
    degree: bool,
}

impl Scale {

    pub fn new() -> ScaleBuilder {
        ScaleBuilder::new()
    }

    fn to_degree(degree: usize, dec: Vec<Decorators>) -> Degree {
        Degree {
            degree,
            dec
        }
    }
}

#[derive(Debug)]
pub struct Degree {
    degree: usize,
    dec: Vec<Decorators>,
}

impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        for decor in self.dec.iter() {
            ret.push_str(&(*decor.to_string()));
            ret.push_str(",");
        }
        write!(f, "{}{}", self.degree, ret)
    }
}

#[derive(Debug, PartialEq)]
pub struct ScaleIter<'a> {
    scale: &'a Scale,
    note: Note<Scaled>,
    idx: usize,
    oct_idx: usize,
    upper: usize,
}

impl<'a> Iterator for ScaleIter<'a> {
    type Item = ScaleItem;

    fn next(&mut self) -> Option<Self::Item> {  
        if self.oct_idx == self.upper {
            self.note = self.scale.key.clone(); 
            self.idx = 0;
            self.oct_idx = 0;
        }
        let ret = self.note.clone();
        let moves = Moves { 
            interval: self.scale.scale_type.steps[self.idx].try_into().unwrap(), 
            direction: self.scale.direction
        };
        let len = self.scale.scale_type.steps.len();
        self.note.move_with(
            moves,
            1
        );
        self.idx = (self.idx + 1) % len;
        self.oct_idx += 1;
        Some(ScaleItem::Note(ret))
    }

    fn collect<B: FromIterator<Self::Item>>(self) -> B {
        let mut vec = Vec::<Self::Item>::new();
        let upper = self.upper.clone();
        for (idx, note) in self.enumerate() {
            if idx == upper { break; } 
            vec.push(note); 
        }
        B::from_iter(vec)
    }
}

impl<'a> IntoIterator for &'a Scale {
    type Item = ScaleItem;
    type IntoIter = ScaleIter<'a>;


    fn into_iter(self) -> Self::IntoIter {
        let len = self.scale_type.steps.len();
        let upper = ((self.high_octave as i8 - self.low_octave as i8) as usize) * len + 1;
        ScaleIter {
            scale: self,
            note: self.key.clone(),
            idx: 0,
            oct_idx: 0,
            upper 
        }
    }
}

pub struct ScaleBuilder (Scale);

impl ScaleBuilder {
    pub fn new() -> ScaleBuilder {
        ScaleBuilder(Scale{
            bounded: false,
            high_octave: Octave::O4,
            degree: true,
            ..Default::default()
        })
    }

    pub fn low(mut self, low: Octave) -> Self {
        self.0.low_octave = low;
        self
    }

    pub fn high(mut self, high: Octave) -> Self {
        self.0.high_octave = high;
        self
    }

    pub fn bound(mut self, bound: bool) -> Self {
        self.0.bounded = bound;
        self
    }

    pub fn direction(mut self, dir: Direction) -> Self {
        self.0.direction = dir;
        self
    }

    pub fn key(mut self, key: Note<Scaled>) -> Self {
        self.0.key = key; 
        self
    }

    pub fn scale_type(mut self, stype: ScaleType) -> Self {
        self.0.scale_type = stype;
        self
    }

    pub fn degree(mut self, degree: bool) -> Self {
        self.0.degree = degree;
        self
    }

    pub fn build(mut self) -> Scale {
        if self.0.high_octave < self.0.low_octave {
            let temp = self.0.high_octave;  
            self.0.high_octave = self.0.low_octave;
            self.0.low_octave = temp;
        }
        if self.0.direction == Direction::Up {
            self.0.key.octave = self.0.low_octave;
        } else {
            self.0.key.octave = self.0.high_octave;
        }
        self.0
    }
}
//impl Deref for ScaleBuilder {
//    type Target = Scale;    
//
//    fn deref() -> &Self::Target {
//        &self.0 
//    }
//}
//
//impl DerefMut for ScaleBuilder {
//    fn deref_mut(&mut self) -> &mut Self::Target {
//        &mut self.0
//    }
//}
//
//impl ScaleBuilder {
//    pub fn new() -> Self {
//        ScaleBuilder (Scale::default())
//    }
//
//    pub fn key(&mut self, key: Note<Scaled>) -> &mut Self {
//        self.key = key;
//        self
//    }
//
//    pub fn scale_type(&mut self, scale_type: ScaleType) -> &mut Self {
//        self.scale_type = scale_type;
//        self
//    }
//
//    // TODO: impl this 
//    fn construct_scale(&self) {
//        
//    }
//
//    pub fn build(self) -> Scale {
//        self.construct_scale();
//        self.0
//    }
//}

#[derive(Debug)]
pub struct ScaleChunk(Vec<ScaleItem>);

impl FromIterator<ScaleItem> for ScaleChunk {

    fn from_iter<T: IntoIterator<Item=ScaleItem>>(iter: T) -> Self {
        Self (iter.into_iter().collect())
    }
}

impl Deref for ScaleChunk {
    type Target = Vec<ScaleItem>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl fmt::Display for ScaleChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        for item in self.0.iter() {
            match item {
                ScaleItem::Note(note) => {
                    ret.push_str(note.to_string().as_ref());
                },
                ScaleItem::Degree(degree) => {
                    ret.push_str(degree.to_string().as_ref());
                }
            }
            ret.push_str(",");
        }
        write!(f, "{}", ret)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn scale_into_iter_test() {
        let data = r#"
                {
                    "name": "Ionian",
                    "steps": [
                        2, 2, 1, 2, 2, 2, 1
                    ],
                    "degree": [
                        1, 2, 3, 4, 5, 6, 7
                    ]
                }
        "#;        
        let scale_type: ScaleType = serde_json::from_str(data).unwrap();

        let note: Note<Scaled> = Note::<Scaled>::new()
                        .raw(RawNote::A)
                        .decorator(Decorators::Natural)
                        .scaled()
                        .build();

        let scale = Scale {
            key: note,
            scale_type: scale_type,
            direction: Direction::Up,
            bounded: false,
            low_octave: Default::default(),
            high_octave: Octave::O4,
            degree: false,
        };

        let scale_iter = ScaleIter {
            scale: &scale,
            note: scale.key.clone(),
            idx: 0,
            oct_idx: 0,
            upper: 1
        };
        for (idx, note) in (&scale).into_iter().enumerate() {
            if idx == 10 { break; }
        }
        assert_eq!(scale_iter, scale.into_iter());
    }

    #[test]
    fn scale_sharp_collect_test() {
        let left = "Bb3,C4,D4,Eb4,F4,G4,A4,";
        let data = r#"
                {
                    "name": "Ionian",
                    "steps": [
                        2, 2, 1, 2, 2, 2, 1
                    ],
                    "degree": [
                        1, 2, 3, 4, 5, 6, 7
                    ]
                }
        "#;        
        let scale_type: ScaleType = serde_json::from_str(data).unwrap();

        let note: Note<Scaled> = Note::<Scaled>::new()
                        .raw(RawNote::B)
                        .decorator(Decorators::Flat)
                        .scaled()
                        .build();

        let scale = Scale {
            key: note,
            scale_type: scale_type,
            direction: Direction::Up,
            bounded: false,
            low_octave: Default::default(),
            high_octave: Octave::O4,
            degree: false,
        };

        let scale_iter = ScaleIter {
            scale: &scale,
            note: scale.key.clone(),
            idx: 0,
            oct_idx: 0,
            upper: 1
        };
        let collected: ScaleChunk = (&scale).into_iter().collect();
        assert_eq!(left, collected.to_string());
    }
}
