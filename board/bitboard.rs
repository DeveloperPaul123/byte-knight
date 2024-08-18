use std::{
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use super::pieces::{Piece, Pieces};

/// Bitboard representation of a chess board.
/// LSB (bit 0) is a1, MSB (bit 63) is h8.
/// The board is represented as a 64-bit integer.
/// A bit is set if the corresponding square is occupied.
///
/// v-a8 (bit 56)
/// 0 0 0 0 0 0 0 0 <- h8 (bit 63)
/// 0 0 0 0 0 0 0 0
/// 0 0 0 0 0 0 0 0
/// 0 0 0 0 0 0 0 0
/// 0 0 0 0 1 0 0 0
/// 0 0 0 0 0 0 0 0 <- h3 (bit 23)
/// 0 0 0 0 0 0 0 0
/// 0 0 0 0 0 0 0 0 <- h1 (bit 7)
/// ^-a1 (bit 0)
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bitboard {
    data: u64,
}

impl PartialEq<u64> for Bitboard {
    fn eq(&self, other: &u64) -> bool {
        self.data == *other
    }
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

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.data &= rhs.data;
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

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data |= rhs.data;
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

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.data ^= rhs.data;
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Bitboard { data: !self.data }
    }
}

impl Bitboard {
    /// Create a new Bitboard with the given data.
    pub fn new(data: u64) -> Self {
        Bitboard { data: data }
    }

    pub fn default() -> Self {
        Bitboard { data: 0 }
    }

    /// Check if a square is occupied.
    pub fn is_square_occupied(&self, square: usize) -> bool {
        self.data & (1 << square) != 0
    }

    /// Mark a square as occupied.
    pub fn set_square(&mut self, square: usize) {
        self.clear_square(square);
        self.data |= 1 << square;
    }

    /// Clear a given square.
    pub fn clear_square(&mut self, square: usize) {
        self.data &= !(1 << square);
    }

    pub fn number_of_occupied_squares(&self) -> u32 {
        self.data.count_ones()
    }

    pub fn as_number(&self) -> u64 {
        self.data
    }
}

// Allow printing the Bitboard
impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const LAST_BIT: u64 = 63;
        for rank in 0..8 {
            for file in (0..8).rev() {
                let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
                let symbol = if self.data & mask != 0 { 'x' } else { '-' };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_new() {
        let bb = Bitboard::new(0x8000000000000001);
        assert_eq!(bb.data, 0x8000000000000001);
        println!("{}", bb);
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

        bb = Bitboard::new(0);
        bb.set_square(28);
        assert_eq!(bb.data, 0x10000000);
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
