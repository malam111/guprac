use super::traits::*;
use super::enums::*;
use super::notes::*;
use std::default::Default;

struct Scale<T> {
    node: Vec<Node<T>>,
    node_type: NodeType,
    key: ENote,
    mode: ScaleType,
}

impl<T: ScaleNode> Scale<T> {
    fn new<N>() -> ScaleBuilder<N> {
        ScaleBuilder::<N>::new()
    }

}

struct ScaleBuilder<T> {
   inner: Scale<T> 
}

impl<T> ScaleBuilder<T> {
    fn new() -> ScaleBuilder<T> {
        Self {
            inner: Scale {
               node: Vec::<Node<T>>::new(),
               node_type: NodeType::default(),
               key: ENote::C,
               mode: ScaleType::default(),
            }
        }
    }

    fn node(mut self, node_type: NodeType) -> Self {
        self.inner.node_type = node_type;
        self
    }

    fn key(mut self, key: ENote) -> Self {
        self.inner.key = key;
        self
    }

    fn mode(mut self, mode_type: ScaleType) -> Self {
        self.inner.mode = mode_type;
        self
    }

    fn build(mut self) -> Self {
        self.inner.node = self.inner_scale_builder();
        self
    }

    fn inner_scale_builder(&self) -> Vec<Node<T>> {
        let steps = self.inner.mode.scale_pattern();
        let notes = Vec::<Node<T>>::new();
        for step in steps.into_iter() {
        }
        notes
    }
}

impl<T> Default for ScaleBuilder<T> {
    fn default() -> Self {
        Self::new()        
    }
}


