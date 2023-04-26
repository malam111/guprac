use crate::units::steps::StepType;

#[derive(PartialEq, Debug)]
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