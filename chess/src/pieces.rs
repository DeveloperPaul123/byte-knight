/*
 * pieces.rs
 * Part of the byte-knight project
 * Created Date: Monday, November 25th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Tue Dec 17 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::fmt::Display;

use crate::definitions::NumberOf;

/// Names of squares on the board. The index of the square name corresponds to the square index as represented by the bitboard.
/// See the [crate::bitboard::Bitboard] for more information.
#[rustfmt::skip]
pub const SQUARE_NAME: [&str; NumberOf::SQUARES] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"
];

/// Fully qualified piece names. Use [Pieces] to index into this array.
pub const PIECE_NAMES: [&str; NumberOf::PIECE_TYPES] =
    ["King", "Queen", "Rook", "Bishop", "Knight", "Pawn"];

/// Short names for pieces. Use [Pieces] to index into this array.
pub const PIECE_SHORT_NAMES: [char; NumberOf::PIECE_TYPES + 1] =
    ['K', 'Q', 'R', 'B', 'N', 'P', ' '];

pub const SLIDER_PIECES: [Piece; 3] = [Piece::Rook, Piece::Bishop, Piece::Queen];
pub const ALL_PIECES: [Piece; 6] = [
    Piece::King,
    Piece::Queen,
    Piece::Rook,
    Piece::Bishop,
    Piece::Knight,
    Piece::Pawn,
];

/// Represents a chess piece.
///
/// **Note**: The ordinal value of the piece is used throughout the
/// code to index into arrays and tables. Changing the value of a piece
/// would likely be catastrophic and result in a number of bugs and possibly crashes.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    #[default]
    None = 6,
}

impl Piece {
    /// Returns `true` if the piece is [`KING`].
    ///
    /// [`KING`]: Piece::KING
    #[must_use]
    pub fn is_king(&self) -> bool {
        matches!(self, Self::King)
    }

    /// Returns `true` if the piece is [`QUEEN`].
    ///
    /// [`QUEEN`]: Piece::QUEEN
    #[must_use]
    pub fn is_queen(&self) -> bool {
        matches!(self, Self::Queen)
    }

    /// Returns `true` if the piece is [`ROOK`].
    ///
    /// [`ROOK`]: Piece::ROOK
    #[must_use]
    pub fn is_rook(&self) -> bool {
        matches!(self, Self::Rook)
    }

    /// Returns `true` if the piece is [`BISHOP`].
    ///
    /// [`BISHOP`]: Piece::BISHOP
    #[must_use]
    pub fn is_bishop(&self) -> bool {
        matches!(self, Self::Bishop)
    }

    /// Returns `true` if the piece is [`KNIGHT`].
    ///
    /// [`KNIGHT`]: Piece::KNIGHT
    #[must_use]
    pub fn is_knight(&self) -> bool {
        matches!(self, Self::Knight)
    }

    /// Returns `true` if the piece is [`PAWN`].
    ///
    /// [`PAWN`]: Piece::PAWN
    #[must_use]
    pub fn is_pawn(&self) -> bool {
        matches!(self, Self::Pawn)
    }

    /// Returns `true` if the piece is [`NONE`].
    ///
    /// [`NONE`]: Piece::NONE
    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` if the piece is a slider piece.
    ///
    /// A slider piece is a piece that can move any number of squares in a straight line.
    #[must_use]
    pub fn is_slider(&self) -> bool {
        self.is_rook() || self.is_bishop() || self.is_queen()
    }

    /// Returns the short name of the piece as a lowercase character.
    pub fn as_char(&self) -> char {
        PIECE_SHORT_NAMES[*self as usize].to_ascii_lowercase()
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::King => write!(f, "King"),
            Piece::Queen => write!(f, "Queen"),
            Piece::Rook => write!(f, "Rook"),
            Piece::Bishop => write!(f, "Bishop"),
            Piece::Knight => write!(f, "Knight"),
            Piece::Pawn => write!(f, "Pawn"),
            Piece::None => write!(f, "None"),
        }
    }
}

impl TryFrom<u8> for Piece {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Piece::King),
            1 => Ok(Piece::Queen),
            2 => Ok(Piece::Rook),
            3 => Ok(Piece::Bishop),
            4 => Ok(Piece::Knight),
            5 => Ok(Piece::Pawn),
            6 => Ok(Piece::None),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'K' => Ok(Piece::King),
            'Q' => Ok(Piece::Queen),
            'R' => Ok(Piece::Rook),
            'B' => Ok(Piece::Bishop),
            'N' => Ok(Piece::Knight),
            'P' => Ok(Piece::Pawn),
            _ => Err(()),
        }
    }
}
