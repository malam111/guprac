use std::fmt;

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone, Educe)]
#[educe(Default)]
#[repr(i8)]
pub enum Octave {
    O0,
    O1,
    O2,
    #[educe(Default)]
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
}

impl Octave {
    pub fn next(mut self) -> Self {
        match self {
            Self::O0 => Self::O1,
            Self::O1 => Self::O2,
            Self::O2 => Self::O3,
            Self::O3 => Self::O4,
            Self::O4 => Self::O5,
            Self::O5 => Self::O6,
            Self::O6 => Self::O7,
            Self::O7 => Self::O8,
            Self::O8 => Self::O0,
        }
    }

    pub fn prev(mut self) -> Self {
        match self {
            Self::O0 => Self::O8,
            Self::O1 => Self::O0,
            Self::O2 => Self::O1,
            Self::O3 => Self::O2,
            Self::O4 => Self::O3,
            Self::O5 => Self::O4,
            Self::O6 => Self::O5,
            Self::O7 => Self::O6,
            Self::O8 => Self::O7,
        }
    }   

    pub fn distance(src: Self, dst: Self) -> i8 {
        todo!();
    }
}

impl fmt::Display for Octave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as i8)
    }
}
