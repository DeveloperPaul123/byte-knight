use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bitboard {
    data: u64,
}

// Implement bitwise operations for Bitboard
impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard {
            data: self.data & rhs.data,
        }
    }
}

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard {
            data: self.data | rhs.data,
        }
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard {
            data: self.data ^ rhs.data,
        }
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Bitboard { data: !self.data }
    }
}

impl Bitboard {
    pub fn new(data: u64) -> Self {
        Bitboard { data: data }
    }

    pub fn is_square_occupied(&self, square: usize) -> bool {
        self.data & (1 << square) != 0
    }

    pub fn set_square(&mut self, square: usize) {
        self.data |= 1 << square;
    }

    pub fn clear_square(&mut self, square: usize) {
        self.data &= !(1 << square);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_new() {
        let bb = Bitboard::new(0x8000000000000001);
        assert_eq!(bb.data, 0x8000000000000001);
    }

    #[test]
    fn test_is_square_occupied() {
        let bb = Bitboard::new(0x8000000000000001);
        assert!(bb.is_square_occupied(0));
        assert!(bb.is_square_occupied(63));
        assert!(!bb.is_square_occupied(1));
        assert!(!bb.is_square_occupied(62));
    }

    #[test]
    fn test_set_square() {
        let mut bb = Bitboard::new(0);
        bb.set_square(0);
        bb.set_square(63);
        assert_eq!(bb.data, 0x8000000000000001);
    }

    #[test]
    fn test_clear_square() {
        let mut bb = Bitboard::new(0xFFFFFFFFFFFFFFFF);
        bb.clear_square(0);
        bb.clear_square(63);
        assert_eq!(bb.data, 0x7FFFFFFFFFFFFFFE);
    }

    #[test]
    fn test_bitwise_operations() {
        let bb1 = Bitboard::new(0xF0F0F0F0F0F0F0F0);
        let bb2 = Bitboard::new(0x0F0F0F0F0F0F0F0F);

        // AND
        assert_eq!((bb1 & bb2).data, 0);

        // OR
        assert_eq!((bb1 | bb2).data, 0xFFFFFFFFFFFFFFFF);

        // XOR
        assert_eq!((bb1 ^ bb2).data, 0xFFFFFFFFFFFFFFFF);

        // NOT
        assert_eq!((!bb1).data, 0x0F0F0F0F0F0F0F0F);
    }
}
