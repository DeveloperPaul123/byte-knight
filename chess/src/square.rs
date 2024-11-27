/*
 * square.rs
 * Part of the byte-knight project
 * Created Date: Friday, August 16th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Tue Nov 26 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use crate::{
    bitboard::Bitboard, bitboard_helpers, color::Color, definitions::DARK_SQUARES, file::File,
    rank::Rank,
};

use anyhow::Result;

/// Represents a square on the chess board.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }

    /// Creates a new square from a file character and rank number.
    pub fn from_file_rank(file: char, rank: u8) -> Result<Self> {
        let file = File::try_from(file)?;
        let rank = Rank::try_from(rank)?;
        Ok(Self { file, rank })
    }

    /// Creates a new square from a bitboard.
    ///
    /// This will get the first square from the bitboard and convert it to a [`Square`].
    pub fn from_bitboard(bitboard: &Bitboard) -> Self {
        let sq = bitboard_helpers::next_bit(&mut bitboard.to_owned());
        Self::from_square_index(sq as u8)
    }

    /// Convert to a raw square index (0-63).
    pub fn to_square_index(&self) -> u8 {
        to_square(self.file as u8, self.rank as u8)
    }

    /// Convert a square index to a [`Square`] object.
    pub fn from_square_index(square: u8) -> Self {
        let (file, rank) = from_square(square);
        Self {
            file: File::try_from(file).unwrap(),
            rank: Rank::try_from(rank).unwrap(),
        }
    }

    /// Offset the square by the given file and rank deltas.
    ///
    /// Returns `None` if the resulting square is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::square::Square;
    /// use chess::rank::Rank;
    /// use chess::file::File;
    ///
    /// let square = Square::try_from("e4").unwrap();
    /// let new_square = square.offset(1, 1).unwrap();
    /// assert_eq!(new_square.file, File::F);
    /// assert_eq!(new_square.rank, Rank::R5);
    ///
    /// let square = Square::try_from("a1").unwrap();
    /// let new_square = square.offset(-1, -1);
    /// assert!(new_square.is_none());
    /// ```
    pub fn offset(&self, file_delta: i8, rank_delta: i8) -> Option<Self> {
        let new_file = self.file.offset(file_delta)?;
        let new_rank = self.rank.offset(rank_delta)?;
        Some(Self::new(new_file, new_rank))
    }

    /// Get the bitboard representation of the square.
    pub fn bitboard(&self) -> Bitboard {
        Bitboard::from_square(self.to_square_index())
    }

    /// Returns `true` if the square is a dark square.
    pub fn is_dark(&self) -> bool {
        Bitboard::from(DARK_SQUARES) & self.bitboard() != Bitboard::EMPTY
    }

    /// Returns `true` if the square is a light square.
    pub fn is_light(&self) -> bool {
        !self.is_dark()
    }

    /// Returns the color of the square.
    pub fn color(&self) -> Color {
        if self.is_dark() {
            Color::Black
        } else {
            Color::White
        }
    }
}

impl TryFrom<&str> for Square {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(anyhow::Error::msg(format!(
                "Input square must be at least 2 characters {}",
                value
            )));
        }

        // file can match directly to a char so we don't alter it
        let file = value.chars().nth(0).unwrap();
        // read the raw rank value (1-8)
        let rank = value.chars().nth(1).unwrap();
        // rank values are 1-8, so we need to convert to 0-7
        let rank_digit = rank.to_digit(10).unwrap() - 1;
        Square::from_file_rank(file, rank_digit as u8)
    }
}

/// Converts a file and rank tuple to a square
///
/// # Arguments
///
/// * `file` - The file to convert
/// * `rank` - The rank to convert
///
/// # Returns
///
/// The square corresponding to the given file and rank
pub const fn to_square(file: u8, rank: u8) -> u8 {
    rank * 8 + file
}

/// Converts a file and rank index to a [`Square`] object.
///
/// # Arguments
///
/// * `file` - The file index to convert
/// * `rank` - The rank index to convert
///
/// # Returns
///
/// A [`Square`] object corresponding to the given file and rank indices.
pub fn to_square_object(file: u8, rank: u8) -> Square {
    Square::new(File::try_from(file).unwrap(), Rank::try_from(rank).unwrap())
}

/// Converts a square to a file and rank tuple
///
/// # Arguments
///
/// * `square` - The square to convert
///
/// # Returns
///
/// A tuple containing the file and rank of the given square (file, rank)
pub const fn from_square(square: u8) -> (u8, u8) {
    let rank = square / 8;
    let file = square % 8;
    (file, rank)
}

/// Checks if a given square is on a given rank.
pub const fn is_square_on_rank(square: u8, rank: u8) -> bool {
    let (_, rnk) = from_square(square);
    rnk == rank
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::Squares,
        file::File,
        rank::Rank,
        square::{is_square_on_rank, Square},
    };

    #[test]
    fn check_square_on_rank() {
        assert!(is_square_on_rank(Squares::A1, Rank::R1 as u8));
        assert!(!is_square_on_rank(Squares::A1, Rank::R2 as u8));
        assert!(is_square_on_rank(Squares::C5, Rank::R5 as u8));
    }

    #[test]
    fn parse_square_from_uci_str() {
        let square = Square::try_from("e4").unwrap();
        assert_eq!(square.file, File::E);
        assert_eq!(square.rank, Rank::R4);
    }

    #[test]
    fn offset() {
        let square = Square::try_from("e4").unwrap();
        let new_square = square.offset(1, 1).unwrap();
        assert_eq!(new_square.file, File::F);
        assert_eq!(new_square.rank, Rank::R5);

        let square = Square::try_from("a1").unwrap();
        let new_square = square.offset(-1, -1);
        assert!(new_square.is_none());
    }
}
