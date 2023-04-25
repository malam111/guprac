use std::{ops::{Deref, DerefMut, Add}};

use crate::enums::StepType;
use super::*;

#[derive(Default, PartialEq, Debug)]
pub struct Node<T> {
    node: T,
    decorators: Vec<enums::Decorators>,
    octave: enums::Octaves
}

impl<T: traits::ScaleNode + Into<u8> + Default> Node<T> {
    pub fn new(node: T) -> NodeBuilder<T> {
        NodeBuilder::new(node)
    }

    pub fn add(&mut self, step: StepType) -> Self {
        let node: u8 = self.node.into();
        let step: u8 = step.into();
        let temp: u8 = node + step;
        Node::default()
    }
}

impl Into<u8> for Node<T> {
    fn into(self) -> u8 {
        match node {
             
        }
    }
}


pub struct NodeBuilder<T> {
    inner: Node<T>
}

impl<T> Deref for NodeBuilder<T> {
    type Target = Node<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for NodeBuilder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner 
    }
}

impl<T> NodeBuilder<T> {

    pub fn new(node: T) -> Self {
        Self {
            inner: Node {
                node,
                decorators: Vec::<enums::Decorators>::default(),
                octave: enums::Octaves::default()
            }
        }
    }

    pub fn decorators(mut self, decors: Vec<enums::Decorators>) -> Self {
        self.decorators = decors;
        self
    }

    pub fn octave(mut self, octave: enums::Octaves) -> Self {
        self.octave = octave;
        self
    }

    pub fn build(self) -> Node<T> {
        self.inner
    }

}


#[cfg(test)]
mod test {
    use crate::enums::{Decorators, Interval, ENote};

    use super::*;

    #[test]
    fn test_interval() {
        let note_left = Node::new(enums::Interval::One)
                                                .decorators(vec![enums::Decorators::Flat])
                                                .octave(enums::Octaves::default())
                                                .build();

        let note_right = Node::<Interval> {
            node: Interval::One,
            decorators: vec![enums::Decorators::Flat],
            octave: enums::Octaves::default()
        };
        assert_eq!(note_left, note_right);
    }


    #[test]
    fn test_note() {
        let note_left = Node::new(enums::ENote::C)
                                            .decorators(vec![enums::Decorators::Sharp, enums::Decorators::Sharp])
                                            .octave(enums::Octaves::Three)
                                            .build();
        let note_right = Node::<ENote> {
            node: ENote::C,
            decorators: vec![enums::Decorators::Sharp, enums::Decorators::Sharp],
            octave: enums::Octaves::Three
        };
        assert_eq!(note_left, note_right);
    }
}