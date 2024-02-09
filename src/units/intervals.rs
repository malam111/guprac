use std::convert::{TryFrom, Into};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Interval {
    Per1,
    Min2,
    Maj2,
    Min3,
    Maj3,
    Per4,
    Tritone,
    Per5,
    Min6,
    Maj6,
    Min7,
    Maj7,
    Per8
}

impl Interval {
    fn try_unsigned(value: u8) -> Result<Self, ErrInterval> {
        match value {
            0 => Ok(Self::Per1),
            1 => Ok(Self::Min2),
            2 => Ok(Self::Maj2),
            3 => Ok(Self::Min3),
            4 => Ok(Self::Maj3),
            5 => Ok(Self::Per4),
            6 => Ok(Self::Tritone),
            7 => Ok(Self::Per5),
            8 => Ok(Self::Min6),
            9 => Ok(Self::Maj6),
            10 => Ok(Self::Min7),
            11 => Ok(Self::Maj7),
            12 => Ok(Self::Per8),

            _ => Err(ErrInterval::default())
        }
    }

}

impl Default for Interval {
    fn default() -> Self {
        Self::Per1
    }
}

#[derive(Debug, Default)]
pub struct ErrInterval;

impl TryFrom<u8> for Interval {
    type Error = ErrInterval;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_unsigned(value)
    }
}

impl TryFrom<i8> for Interval {
    type Error = ErrInterval;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let mut ret: u8 = (value & 0b0000_1111) as u8;
        if value < 0 {
            let signed: u8  = (value & 0b1000_0000_u8 as i8) as u8;
            let usigned: u8 = (value & 0b0111_1111_u8 as i8) as u8;
            ret = (signed - usigned) as u8;
        }
        Self::try_unsigned(ret)
    }
}

#[cfg(test)]
mod test{
    
    use super::*;

    #[test]
    fn u8_test() {
        assert_eq!(Interval::Maj3, Interval::try_from(4_u8).unwrap());
    }

    #[test]
    fn i8_test() {
        assert_eq!(Interval::Maj3, Interval::try_from(4_i8).unwrap());
    }
}
