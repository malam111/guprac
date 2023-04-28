use std::ops::{Deref, DerefMut};
#[derive(Copy, Clone)]
pub enum RawDecorators {
    Natural,
    Flat,
    Sharp
}

impl Default for RawDecorators {
    fn default() -> Self {
        Self::Natural
    }
}

#[derive(Default)]
pub struct Decorators (Vec<RawDecorators>);

impl Deref for Decorators {
    type Target = Vec<RawDecorators>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Decorators {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}