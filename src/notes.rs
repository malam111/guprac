use std::{ops::{Deref, DerefMut}};
use crate::units::*;

#[derive(Default, PartialEq, Debug)]
pub struct Node {
    node: Note,
    decorators: Vec<Decorators>,
    octave: Octaves
}

impl Node {
    pub fn new(node: Note) -> NodeBuilder {
        NodeBuilder::new(node)
    }
}

impl Into<u8> for Node {
    fn into(self) -> u8 {
        match self.node {
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
                node,
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

    pub fn build(self) -> Node {
        self.inner
    }

}

struct NodeCalc {
    before: Node,
    after: Node
}

impl NodeCalc {
    fn new(node: Node) -> NodeCalc {
        NodeCalc {
            before: node,
            after: node
        }
    }

    fn add(&mut self, step: StepType) -> Node {
        match step {
            StepType::Half => {
                return Node::new(self.before.node.next().unwrap()).build();
            },
            StepType::Whole => {
                return Node::new(self.before.node.next().unwrap().next().unwrap()).build();
            }
        }
    }

    fn multiple_add(&mut self, step: Vec<StepType>) -> Node {

    }
}

#[cfg(test)]
mod test {
    use crate::units::{Decorators, Interval, Note};

    use super::*;

    // #[test]
    // fn test_interval() {
    //     let note_left = Node::new(enums::Interval::One)
    //                                             .decorators(vec![enums::Decorators::Flat])
    //                                             .octave(enums::Octaves::default())
    //                                             .build();

    //     let note_right = Node::<Interval> {
    //         node: Interval::One,
    //         decorators: vec![enums::Decorators::Flat],
    //         octave: enums::Octaves::default()
    //     };
    //     assert_eq!(note_left, note_right);
    // }


    #[test]
    fn test_note() {
        let note_left = Node::new(Note::C)
                                            .decorators(vec![Decorators::Sharp, Decorators::Sharp])
                                            .octave(Octaves::Three)
                                            .build();
        let note_right = Node {
            node: Note::C,
            decorators: vec![Decorators::Sharp, Decorators::Sharp],
            octave: Octaves::Three
        };
        assert_eq!(note_left, note_right);
    }

    #[test]
    fn add_note() {
        let mut note_left = Node::new(Note::C)
                                            .decorators(vec![Decorators::Sharp, Decorators::Sharp])
                                            .octave(Octaves::Three)
                                            .build();
        note_left.add(StepType::Whole);
        let note_right = Node {
            node: Note::D,
            decorators: vec![Decorators::Natural],
            octave: Octaves::Three
        };
        assert_eq!(note_left, note_right);
    }
}