use std::ops::{Deref, DerefMut};
use std::fmt;


#[derive(PartialEq, Debug, Copy, Clone)]
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

impl fmt::Display for Decorators {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{}", match self {
            Self::Natural => "",
            Self::Flat => "b",
            Self::Sharp => "#",
        })
    }
}
