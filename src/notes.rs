use std::{ops::{Deref, DerefMut}, mem};

use super::*;

pub struct Note<T> {
    node: T,
    decorators: Vec<enums::Decorators>,
    octave: enums::Octaves
}

impl<T: traits::ScaleNode> Note<T> {
    fn new(node: T) -> NodeBuilder<T> {
        NodeBuilder::new(node)
    }
}

struct NodeBuilder<T> {
    inner: Note<T>
}

impl<T> Deref for NodeBuilder<T> {
    type Target = Note<T>;

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
    fn new(node: T) -> Self {
        Self {
            inner: Note {
                node,
                decorators: Vec::<enums::Decorators>::new(),
                octave: enums::Octaves::default()
            }
        }
    }

    fn decorators(mut self, decors: Vec<enums::Decorators>) -> Self {
        self.decorators = decors;
        self
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interval() {
        let interval = enums::Interval::One;
        let note = Note::new(interval);
    }


    #[test]
    fn test_note() {
        let note = enums::ENote::C;
        let note = Note::new(note);
    }
}