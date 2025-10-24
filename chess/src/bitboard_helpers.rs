/*
 * bitboard_helpers.rs
 * Part of the byte-knight project
 * Created Date: Monday, November 25th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Tue Nov 26 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use crate::bitboard::Bitboard;

/// Returns the index of the next bit set to 1 in the bitboard and sets it to 0.
///
/// # Arguments
///
/// * `bitboard` - The bitboard to get the next bit from.
///
/// # Returns
///
/// The index of the next bit set to 1 in the bitboard.
///
/// # Examples
///
/// ```
/// use chess::bitboard::Bitboard;
/// use chess::bitboard_helpers::next_bit;
///
/// let mut bb = Bitboard::new(0x8000000000000001);
/// assert_eq!(next_bit(&mut bb), 0);
/// assert_eq!(next_bit(&mut bb), 63);
/// assert_eq!(bb.as_number(), 0);
///
/// ```
///
/// ```
/// use chess::bitboard::Bitboard;
/// use chess::bitboard_helpers::next_bit;
///
/// let mut bb = Bitboard::new(0xFFFFFFFFFFFFFFFF);
/// for i in 0..64 {
///    assert_eq!(next_bit(&mut bb), i);
/// }
///
/// ```
///
pub fn next_bit(bitboard: &mut Bitboard) -> usize {
    let square = bitboard.as_number().trailing_zeros();
    *bitboard ^= 1u64 << square;
    square as usize
}

pub fn north_fill(bitboard: &Bitboard) -> Bitboard {
    let mut b = *bitboard;
    b |= b << 8;
    b |= b << 16;
    b |= b << 32;
    b
}

#[cfg(test)]
mod tests {
    use crate::definitions::Squares;

    #[test]
    fn test_next_bit() {
        use super::*;
        let mut bb = Bitboard::new(0x8000000000000001);
        assert_eq!(next_bit(&mut bb), 0);
        assert_eq!(next_bit(&mut bb), 63);
        assert_eq!(bb.as_number(), 0);

        {
            let mut bb = Bitboard::new(0xFFFFFFFFFFFFFFFF);
            for i in 0..64 {
                assert_eq!(next_bit(&mut bb), i);
            }
        }
    }

    #[test]
    fn test_north_fill() {
        use super::*;
        let bb = Bitboard::new(0x00000000000000FF);

        let test_bbs = [
            Bitboard::new(0x00000000000000FF),
            Bitboard::new(0x000000000000FFFF),
            Bitboard::new(0x00000000FFFFFFFF),
            Bitboard::new(0xFFFFFFFFFFFFFFFF),
            Bitboard::from_square(Squares::C3) | Bitboard::from_square(Squares::E6),
        ];

        for bb in test_bbs {
            let filled = north_fill(&bb);
            println!("{}", bb);
            println!("filled");
            println!("{}", filled);
            println!("+-------+");
        }

        println!("{}", bb);
        let filled = north_fill(&bb);
        println!("+-------+");
        println!("{}", filled);
        assert_eq!(filled.as_number(), 0xFFFFFFFFFFFFFFFF);
    }
}
