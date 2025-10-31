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

use crate::{bitboard::Bitboard, definitions::FILE_BITBOARDS, file::File};

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

/// Fills all bits to the north of the given bitboard.
/// # Arguments
/// * `bitboard` - The bitboard to fill to the north.
///
/// # Returns
/// A new bitboard with all bits to the north filled.
///
/// # Examples
///
/// ```
/// use chess::bitboard::Bitboard;
/// use chess::bitboard_helpers::north_fill;
/// use chess::definitions::Squares;
/// let bb = Bitboard::from_square(Squares::A2) | Bitboard::from_square(Squares::H2) | Bitboard::from_square(Squares::D4);
/// let filled = north_fill(&bb);
/// // Should look like this:
/// // x - - x - - - x
/// // x - - x - - - x
/// // x - - x - - - x
/// // x - - x - - - x
/// // x - - x - - - x
/// // x - - - - - - x
/// // x - - - - - - x
/// // - - - - - - - -
/// assert_eq!(filled.as_number(), 0x8989898989818100);
/// ```
pub fn north_fill(bitboard: &Bitboard) -> Bitboard {
    let mut b = *bitboard;
    b |= b << 8;
    b |= b << 16;
    b |= b << 32;
    b
}

/// Fills all bits to the south of the given bitboard.
/// # Arguments
/// * `bitboard` - The bitboard to fill to the south.
/// # Returns
/// A new bitboard with all bits to the south filled.
/// # Examples
/// ```
/// use chess::bitboard::Bitboard;
/// use chess::bitboard_helpers::south_fill;
/// use chess::definitions::Squares;
/// let bb = Bitboard::from_square(Squares::A7) | Bitboard::from_square(Squares::H7) | Bitboard::from_square(Squares::D5);
/// let filled = south_fill(&bb);
/// // // Should look like this:
/// // - - - - - - - -
/// // x - - - - - - x
/// // x - - - - - - x
/// // x - - x - - - x
/// // x - - x - - - x
/// // x - - x - - - x
/// // x - - x - - - x
/// // x - - x - - - x
/// assert_eq!(filled.as_number(), 0x81818989898989);
/// ```
pub fn south_fill(bitboard: &Bitboard) -> Bitboard {
    let mut b = *bitboard;
    b |= b >> 8;
    b |= b >> 16;
    b |= b >> 32;
    b
}

/// Fills all bits to the east of the given bitboard without wrap-around.
/// # Arguments
/// * `bitboard` - The bitboard to fill to the east.
/// # Returns
/// A new bitboard with all bits to the east filled.
/// # Examples
/// ```
/// use chess::bitboard::Bitboard;
/// use chess::bitboard_helpers::east_fill;
/// use chess::definitions::Squares;
/// let bb = Bitboard::from_square(Squares::A1) | Bitboard::from_square(Squares::C3);
/// let filled = east_fill(&bb);
/// // Should fill eastward on each rank without wrapping to the next rank
/// assert_eq!(filled.as_number(), 0x00000000000000FF | 0x0000000000FC0000);
/// ```
pub fn east_fill(bitboard: &Bitboard) -> Bitboard {
    // Mask to exclude H-file to prevent wrap-around
    let not_h_file: Bitboard = !FILE_BITBOARDS[File::H as usize];
    let mut b = *bitboard;
    b |= (b & not_h_file) << 1;
    b |= (b & not_h_file) << 1;
    b |= (b & not_h_file) << 1;
    b |= (b & not_h_file) << 1;
    b |= (b & not_h_file) << 1;
    b |= (b & not_h_file) << 1;
    b |= (b & not_h_file) << 1;
    b
}

/// Fills all bits to the west of the given bitboard without wrap-around.
/// # Arguments
/// * `bitboard` - The bitboard to fill to the west.
/// # Returns
/// A new bitboard with all bits to the west filled.
/// # Examples
/// ```
/// use chess::bitboard::Bitboard;
/// use chess::bitboard_helpers::west_fill;
/// use chess::definitions::Squares;
/// let bb = Bitboard::from_square(Squares::H2) | Bitboard::from_square(Squares::D4);
/// let filled = west_fill(&bb);
/// // Should fill eastward on each rank without wrapping to the next rank
/// assert_eq!(filled.as_number(), 0x000000000000FF00 | 0x000000000F000000);
/// ```
pub fn west_fill(bitboard: &Bitboard) -> Bitboard {
    let not_a_file: Bitboard = !FILE_BITBOARDS[File::A as usize];

    let mut b = *bitboard;
    b |= (b & not_a_file) >> 1;
    b |= (b & not_a_file) >> 1;
    b |= (b & not_a_file) >> 1;
    b |= (b & not_a_file) >> 1;
    b |= (b & not_a_file) >> 1;
    b |= (b & not_a_file) >> 1;
    b |= (b & not_a_file) >> 1;
    b
}
#[cfg(test)]
mod tests {
    use crate::{
        bitboard::Bitboard,
        bitboard_helpers,
        definitions::{RANK_BITBOARDS, Squares},
        rank::Rank,
    };

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

    #[test]
    fn test_south_fill() {}

    #[test]
    fn test_east_fill() {
        // Test with a single bit first
        let bb = Bitboard::from_square(Squares::A1);
        let filled = bitboard_helpers::east_fill(&bb);
        assert_eq!(filled, RANK_BITBOARDS[Rank::R1 as usize]);

        // Test with multiple bits
        let bb = Bitboard::from_square(Squares::A3)
            | Bitboard::from_square(Squares::C4)
            | Bitboard::from_square(Squares::D5);
        let filled = bitboard_helpers::east_fill(&bb);
        assert_eq!(filled, Bitboard::new(0x000000F8FCFF0000));
    }

    #[test]
    fn test_west_fill() {
        let bb = Bitboard::from_square(Squares::H8)
            | Bitboard::from_square(Squares::G7)
            | Bitboard::from_square(Squares::F6)
            | Bitboard::from_square(Squares::E5)
            | Bitboard::from_square(Squares::D4)
            | Bitboard::from_square(Squares::C3)
            | Bitboard::from_square(Squares::B2)
            | Bitboard::from_square(Squares::A1);
        let filled = bitboard_helpers::west_fill(&bb);
        assert_eq!(filled, Bitboard::new(0xFF7F3F1F0F070301));

        let single_sq_bb = Bitboard::from_square(Squares::H1);
        let filled_bb = bitboard_helpers::west_fill(&single_sq_bb);
        assert_eq!(filled_bb, RANK_BITBOARDS[Rank::R1 as usize]);
    }
}
