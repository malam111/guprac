#[derive(PartialEq, Debug)]
#[repr(u8)]
pub enum StepType {
    Half,
    Whole,
}

impl Into<u8> for StepType {

    fn into(self) -> u8 {
        match self {
            Self::Half => 1,
            Self::Whole => 2,
        }
    }
}