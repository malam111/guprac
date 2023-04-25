use super::traits::{ScaleNode};

#[derive(PartialEq, Debug)]
pub enum Decorators {
    Natural,
    Sharp,
    Flat,
}

impl Default for Decorators {
    fn default() -> Self {
        Self::Natural
    }
}

#[derive(PartialEq, Debug)]
pub enum Octaves {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Default for Octaves {
    fn default() -> Self {
        Self::Two
    }
}

#[derive(PartialEq, Debug)]
pub enum Interval {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl Default for Interval {
   fn default() -> Self {
       Self::One
   } 
}

impl ScaleNode for Interval {

}

pub enum StepType {
    Half,
    Whole,
}

pub enum ScaleType {
    Major,
    Minor,
}

impl ScaleType {
    pub fn scale_pattern(&self) -> Vec<StepType> {
        match self {
            Self::Major => {
                vec!(StepType::Whole, StepType::Whole, StepType::Half, StepType::Whole, StepType::Whole, StepType::Whole, StepType::Half)
            },
            Self::Minor => {
                vec!(StepType::Whole, StepType::Half, StepType::Whole, StepType::Whole, StepType::Half, StepType::Whole, StepType::Whole)
            }
        }
    }
}

impl Default for ScaleType {
    fn default() -> Self {
        Self::Major
    }
}

pub enum ChordType {
    Major,
    Minor,
    Dominant,
    Diminish,
    HalfDiminish,
    Augmented,
}

#[derive(PartialEq, Debug)]
pub enum ENote {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl ScaleNode for ENote{

}

impl Default for ENote {
    fn default() -> Self {
        Self::C
    }
}

pub enum NodeType {
    ENote,
    Interval
}

impl Default for NodeType {
    fn default() -> Self {
        Self::Interval
    }
}