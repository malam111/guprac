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
    fn get_step(&self) -> StepType {
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
}

impl Iterator for Note {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Note::C => Some(Note::D),
            Note::D => Some(Note::E),
            Note::E => Some(Note::F),
            Note::F => Some(Note::G),
            Note::G => Some(Note::A),
            Note::A => Some(Note::B),
            Note::B => Some(Note::C),
        }
    }

}

impl ScaleNode for Note{

}

impl Default for Note {
    fn default() -> Self {
        Self::C
    }
}