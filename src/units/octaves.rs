#[derive(Debug, Copy, Clone, Educe)]
#[educe(Default)]
pub enum Octave {
    #[educe(Default)]
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
}

impl Octave {
    pub fn next(mut self) {
        self = match self {
            Self::O1 => Self::O2,
            Self::O2 => Self::O3,
            Self::O3 => Self::O4,
            Self::O4 => Self::O5,
            Self::O5 => Self::O6,
            Self::O6 => Self::O7,
            Self::O7 => Self::O8,
            Self::O8 => Self::O1,
        };
    }

    pub fn prev(mut self) {
        self = match self {
            Self::O1 => Self::O8,
            Self::O2 => Self::O1,
            Self::O3 => Self::O2,
            Self::O4 => Self::O3,
            Self::O5 => Self::O4,
            Self::O6 => Self::O5,
            Self::O7 => Self::O6,
            Self::O8 => Self::O7,
        };
    }   
}