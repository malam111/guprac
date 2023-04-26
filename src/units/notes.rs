use crate::units::steps::StepType;
use crate::traits::ScaleNode;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Note {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Note {
    pub fn get_step(self) -> StepType {
        match self {
            Note::C => StepType::Whole,
            Note::D => StepType::Whole,
            Note::E => StepType::Half,
            Note::F => StepType::Whole,
            Note::G => StepType::Whole,
            Note::A => StepType::Whole,
            Note::B => StepType::Half,
        }
    }

    pub fn prev(&mut self) -> Option<Self> {
        for i in 0..6 {
            self.next();
        }
        Some(self.clone())
    }
    
    pub fn peek_prev(&self) -> Option<Self> {
        let mut peek = self.clone();
        for i in 0..6 {
            peek.next();
        }
        Some(peek)
    }

    pub fn peek_next(&self) -> Option<Self> {
        let mut peek = self.clone();
        peek.next();
        Some(peek)
    }
}

impl Iterator for Note {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        *self = match self {
            Note::C => Note::D,
            Note::D => Note::E,
            Note::E => Note::F,
            Note::F => Note::G,
            Note::G => Note::A,
            Note::A => Note::B,
            Note::B => Note::C,
        };
        Some(self.clone())
    }

}

impl ScaleNode for Note{

}

impl Default for Note {
    fn default() -> Self {
        Self::C
    }
}