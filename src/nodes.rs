/// TODO: Octave Awareness

use std::{ops::{Deref, DerefMut}};
use std::cell::Cell;
use crate::units::*;
use crate::traits::NodeCalc;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Node {
    node: Cell<Note>,
    decorators: Vec<Decorators>,
    octave: Octaves
}

impl Node {
    pub fn new(node: Note) -> NodeBuilder {
        NodeBuilder::new(node)
    }

    pub fn get_note(&self) -> Note {
        self.node.get()
    }
}

impl Into<u8> for Node {
    fn into(self) -> u8 {
        match self.node.get() {
            Note::C => 0,
            Note::D => 1,
            Note::E => 2,
            Note::F => 3,
            Note::G => 4,
            Note::A => 5,
            Note::B => 6,
        }
    }
}


pub struct NodeBuilder {
    inner: Node
}

impl Deref for NodeBuilder {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for NodeBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner 
    }
}

impl NodeBuilder {

    pub fn new(node: Note) -> Self {
        Self {
            inner: Node {
                node: Cell::new(node),
                decorators: Vec::<Decorators>::default(),
                octave: Octaves::default()
            }
        }
    }

    pub fn decorators(mut self, decors: Vec<Decorators>) -> Self {
        self.decorators = decors;
        self
    }

    pub fn octave(mut self, octave: Octaves) -> Self {
        self.octave = octave;
        self
    }

    pub fn build(mut self) -> Node {
        if self.decorators == Vec::<Decorators>::default() {
            self.decorators.push(Decorators::Natural);
        }
        self.inner
    }

}

struct NoContextCacl {
    inner: Node,
}

impl Deref for NoContextCacl {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for NoContextCacl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl NodeCalc for NoContextCacl {
    type Target = Self;

    fn new(node: Node) -> NoContextCacl {
        NoContextCacl {inner: node}
    }

    fn add(&mut self, step: StepType) -> &mut Self {
        match step {
            StepType::Half => {self.decorators.push(Decorators::Sharp)},
            StepType::Whole => {
                self.decorators.push(Decorators::Sharp);
                self.decorators.push(Decorators::Sharp);
            },
        }
        self
    }

    

    fn resolve(&mut self) -> Node {
        let mut sharps = 0_u8; 
        let mut flats = 0_u8;
        let mut decorators_left = Vec::<Decorators>::new();
        for decor in self.decorators.iter() {
            match *decor {
                Decorators::Sharp => {
                    sharps += 1;      
                },
                Decorators::Flat => {
                    flats += 1;
                },
                Decorators::Natural => { continue; }
            }
            if sharps > 0 {
                let steps = self.node.get().get_step();
                match steps {
                    StepType::Half => {
                        if sharps > 0 {
                            self.node.set(self.node.get().next().unwrap());
                            sharps -= 1;
                        }
                    },
                    StepType::Whole => {
                        if sharps > 1 {
                            self.node.set(self.node.get().next().unwrap());
                            sharps -= 2;
                        }
                    }
                }
            }
            if flats > 0 {
                match self.node.get().peek_prev().unwrap().get_step() {
                    StepType::Half => {
                        if flats > 0 {
                            self.node.set(self.node.get().prev().unwrap());
                            flats -= 1;
                        }
                    },
                    StepType::Whole => {
                        if flats > 1 {
                            self.node.set(self.node.get().prev().unwrap());
                            flats -= 2;
                        }
                    }
                }
            }
        }
        if sharps != flats {
            if sharps > 0 {
                decorators_left.push(Decorators::Sharp);
            }
            if flats > 0 {
                decorators_left.push(Decorators::Flat);
            }
        } else {
            decorators_left.push(Decorators::Natural);
        }


        self.decorators = decorators_left;
        self.inner.clone()

        
    }

    fn mul_add(&mut self, step: Vec<StepType>) -> &mut Self{
        todo!();
    }
}

#[cfg(test)]
mod test {
    use crate::units::{Decorators, Interval, Note};

    use super::*;

    #[test]
    fn test_note() {
        let note_left = Node::new(Note::C)
                                            .decorators(vec![Decorators::Sharp, Decorators::Sharp])
                                            .octave(Octaves::Three)
                                            .build();
        let note_right = Node {
            node: Cell::new(Note::C),
            decorators: vec![Decorators::Sharp, Decorators::Sharp],
            octave: Octaves::Three
        };
        assert_eq!(note_left, note_right);
    }

    #[test]
    fn add_note() {
        let mut note_left = Node::new(Note::C)
                                            .decorators(vec![
                                                Decorators::Sharp, 
                                                Decorators::Sharp,
                                                Decorators::Sharp, 
                                                Decorators::Sharp,
                                            ])
                                            .octave(Octaves::Three)
                                            .build();
        note_left = NoContextCacl::new(note_left).add(StepType::Half).resolve();
        let note_right = Node {
            node: Cell::new(Note::F),
            decorators: vec![Decorators::Natural],
            octave: Octaves::Three
        };
        println!("{:?}", note_left);
        println!("{:?}", note_right);
        assert_eq!(note_left, note_right);
    }

    #[test]
    fn sub_note() {
        let mut note_left = Node::new(Note::C)
                                            .decorators(vec![
                                                Decorators::Flat, 
                                                Decorators::Flat, 
                                                Decorators::Flat, 
                                            ])
                                            .octave(Octaves::Three)
                                            .build();
        note_left = NoContextCacl::new(note_left).resolve();
        let note_right = Node {
            node: Cell::new(Note::A),
            decorators: vec![Decorators::Natural],
            octave: Octaves::Three
        };
        println!("{:?}", note_left);
        println!("{:?}", note_right);
        assert_eq!(note_left, note_right);
    }

    #[test]
    fn sub_add_note() {
        let mut note_left = Node::new(Note::C)
                                            .decorators(vec![
                                                Decorators::Flat, 
                                                Decorators::Sharp, 
                                                Decorators::Natural, 
                                            ])
                                            .octave(Octaves::Three)
                                            .build();
        note_left = NoContextCacl::new(note_left).resolve();
        let note_right = Node {
            node: Cell::new(Note::C),
            decorators: vec![Decorators::Natural],
            octave: Octaves::Three
        };
        println!("{:?}", note_left);
        println!("{:?}", note_right);
        assert_eq!(note_left, note_right);
    }
}