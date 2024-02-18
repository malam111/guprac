use std::ops::{Deref, DerefMut};
use std::convert::{TryInto};
use std::iter::FromIterator;
use std::fmt;

use super::ScaleType;
use crate::units::{Note, Scaled, Octave, Direction, RawNote, Decorators, Moves};

#[derive(Debug, PartialEq)]
pub struct Scale {
    key: Note<Scaled>,
    scale_type: ScaleType,
    direction: Direction,
}

#[derive(Debug, PartialEq)]
pub struct ScaleIter<'a> {
    scale: &'a Scale,
    note: Note<Scaled>,
    idx: usize,
    upper: usize,
}

impl<'a> Iterator for ScaleIter<'a> {
    type Item = Note<Scaled>;

    fn next(&mut self) -> Option<Self::Item> {  
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
        Some(ret) 
    }

    fn collect<B: FromIterator<Note<Scaled>>>(self) -> B {
        let mut vec = Vec::<Note<Scaled>>::new();
        let len = self.scale.scale_type.steps.len();
        let max = self.upper * len;
        for (idx, note) in self.enumerate() {
            if idx == max { break; } 
            vec.push(note); 
        }
        B::from_iter(vec)
    }
}

impl<'a> IntoIterator for &'a Scale {
    type Item = Note<Scaled>;
    type IntoIter = ScaleIter<'a>;


    fn into_iter(self) -> Self::IntoIter {
        ScaleIter {
            scale: self,
            note: self.key.clone(),
            idx: 0,
            upper: 1
        }
    }
}

struct ScaleBuilder (Scale);

impl ScaleBuilder {
    //fn new() -> ScaleBuilder {
    //    ScaleBuilder(Scale{
    //        ..Default::default()
    //    })
    //}

    fn octave_from_to(&mut self, from: Octave, to: Octave) {
        todo!() 
    }

}
//impl Deref for ScaleBuilder {
//    type Target = Scale;    
//
//    fn deref(&self) -> &Self::Target {
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
struct ScaleChunk(Vec<Note<Scaled>>);

impl FromIterator<Note<Scaled>> for ScaleChunk {

    fn from_iter<T: IntoIterator<Item=Note<Scaled>>>(iter: T) -> Self {
        Self (iter.into_iter().collect())
    }
}


impl fmt::Display for ScaleChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        for note in self.0.iter() {
            ret.push_str(note.to_string().as_ref());
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
        };

        let scale_iter = ScaleIter {
            scale: &scale,
            note: scale.key.clone(),
            idx: 0,
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
        };

        let scale_iter = ScaleIter {
            scale: &scale,
            note: scale.key.clone(),
            idx: 0,
            upper: 1
        };
        // FIXME: buggy
        let collected: ScaleChunk = (&scale).into_iter().collect();
        assert_eq!(left, collected.to_string());
    }
}
