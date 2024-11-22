use crate::side::Side;
use anyhow::Result;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    R1 = 0,
    R2 = 1,
    R3 = 2,
    R4 = 3,
    R5 = 4,
    R6 = 5,
    R7 = 6,
    R8 = 7,
}

impl Rank {
    pub fn promotion_rank(side: Side) -> Rank {
        match side {
            Side::White => Rank::R8,
            Side::Black => Rank::R1,
            _ => Rank::R1,
        }
    }

    pub fn pawn_start_rank(side: Side) -> Rank {
        match side {
            Side::White => Rank::R2,
            Side::Black => Rank::R7,
            _ => panic!("Invalid side"),
        }
    }

    pub fn as_number(&self) -> u8 {
        *self as u8
    }

    pub fn offset(&self, delta: i8) -> Option<Self> {
        let new_rank = (*self as i8) + delta;
        if (0..=7).contains(&new_rank) {
            return Rank::try_from(new_rank as u8).ok();
        }
        None
    }
}

impl TryFrom<u8> for Rank {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::R1),
            1 => Ok(Self::R2),
            2 => Ok(Self::R3),
            3 => Ok(Self::R4),
            4 => Ok(Self::R5),
            5 => Ok(Self::R6),
            6 => Ok(Self::R7),
            7 => Ok(Self::R8),
            _ => Err(anyhow::Error::msg(format!("Invalid rank {}", value))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_offset() {
        assert_eq!(Rank::R1.offset(1), Some(Rank::R2));
        assert_eq!(Rank::R1.offset(-1), None);
        assert_eq!(Rank::R8.offset(1), None);
        assert_eq!(Rank::R8.offset(-1), Some(Rank::R7));
    }
}
