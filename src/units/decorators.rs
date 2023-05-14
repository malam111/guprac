use std::ops::{Deref, DerefMut};
#[derive(Debug, Copy, Clone)]
pub enum Decorators {
    Natural,
    Flat,
    Sharp
}

impl Default for Decorators {
    fn default() -> Self {
        Self::Natural
    }
}
