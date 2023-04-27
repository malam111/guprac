use std::ops::Sub;

#[derive(PartialEq, Debug, Clone)]
#[repr(i8)]
pub enum StepType {
    Half,
    Whole,
}

impl StepType {
    pub fn from_i8(val: i8) -> Option<Self> {
        match val as u8 {
            0 => None,
            1 => Some(Self::Half),
            2 => Some(Self::Whole),
            _ => None,
        }
    }
}

impl Sub for StepType {
    type Output = i8;
    fn sub(self, rhs: Self) -> Self::Output {
        self as i8 - rhs as i8
    }
}

