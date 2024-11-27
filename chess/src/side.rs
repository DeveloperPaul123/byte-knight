/*
 * side.rs
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

/// Represents a side to play in chess.
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    White = 0,
    Black = 1,
    Both = 2,
}

impl Side {
    /// Returns the opposite side.
    pub fn opposite(side: Side) -> Side {
        match side {
            Side::White => Side::Black,
            Side::Black => Side::White,
            _ => Side::Both,
        }
    }

    /// Returns `true` if the side is [`White`].
    ///
    /// [`White`]: Side::White
    #[must_use]
    pub fn is_white(&self) -> bool {
        matches!(self, Self::White)
    }

    /// Returns `true` if the side is [`Black`].
    ///
    /// [`Black`]: Side::Black
    #[must_use]
    pub fn is_black(&self) -> bool {
        matches!(self, Self::Black)
    }

    /// Returns `true` if the side is [`Both`].
    ///
    /// [`Both`]: Side::Both
    #[must_use]
    pub fn is_both(&self) -> bool {
        matches!(self, Self::Both)
    }
}

impl Default for Side {
    fn default() -> Self {
        Self::White
    }
}

impl TryFrom<u8> for Side {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::White),
            1 => Ok(Self::Black),
            2 => Ok(Self::Both),
            _ => Err(()),
        }
    }
}
