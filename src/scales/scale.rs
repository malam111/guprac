use std::ops::{Deref, DerefMut};
use std::convert::{TryInto};

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

    // FIXME: currently not working with decorators 
    fn next(&mut self) -> Option<Self::Item> {  
        let ret = self.note.clone();
        let moves = Moves { 
            interval: self.scale.scale_type.steps[self.idx].try_into().unwrap(), 
            direction: self.scale.direction
        };
        self.note.move_with(
            moves,
            1
        );
        self.idx = (self.idx + 1) % 7;
        Some(ret) 
    }

    //fn collect<B: FromIterator<A>>(self) -> B {
    //    B::from_iter(self);
    //    let max = self.upper * 7;
    //    for idx, note in self {
    //        if idx == max { break; } 
    //           
    //    }
    //}
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

struct ScaleChunk<'a> {
    name: &'a str,
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
                        .decorator(Decorators::Sharp)
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
        let collected: Vec<Note<Scaled>> = (&scale).into_iter().collect();
        println!("{:?}", collected);
        assert_eq!(1, 1);
    }
}
