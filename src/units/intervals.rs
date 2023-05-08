use std::convert::TryFrom;

#[repr(u8)]
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

#[derive(Debug, Default)]
pub struct ErrInterval;

impl TryFrom<u8> for Interval {
    type Error = ErrInterval;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
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

impl TryFrom<i8> for Interval {
    type Error = ErrInterval;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let value: u8 = (value & 0b01111111) as u8;
        value.try_into()
    }
}