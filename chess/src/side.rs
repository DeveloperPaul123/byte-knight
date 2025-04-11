/*
 * side.rs
 * Part of the byte-knight project
 * Created Date: Monday, November 25th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Tue Dec 10 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::fmt::Display;

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

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => write!(f, "W"),
            Self::Black => write!(f, "B"),
            Self::Both => write!(f, "W|B"),
        }
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

impl TryFrom<char> for Side {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(Self::White),
            'b' => Ok(Self::Black),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn side_default() {
        let side: Side = Default::default();
        assert_eq!(side, Side::White);
    }

    #[test]
    fn side_from_u8() {
        assert_eq!(Side::try_from(0), Ok(Side::White));
        assert_eq!(Side::try_from(1), Ok(Side::Black));
        assert_eq!(Side::try_from(2), Ok(Side::Both));
        assert_eq!(Side::try_from(3), Err(()));
    }

    #[test]
    fn side_from_char() {
        assert_eq!(Side::try_from('w'), Ok(Side::White));
        assert_eq!(Side::try_from('b'), Ok(Side::Black));

        for char in ('a'..='z').filter(|val| *val != 'w' && *val != 'b') {
            assert!(Side::try_from(char).is_err());
        }
    }

    #[test]
    fn display_side() {
        assert_eq!(Side::White.to_string(), "W");
        assert_eq!(Side::Black.to_string(), "B");
        assert_eq!(Side::Both.to_string(), "W|B");
    }

    #[test]
    fn opposite() {
        assert_eq!(Side::opposite(Side::White), Side::Black);
        assert_eq!(Side::opposite(Side::Black), Side::White);
        assert_eq!(Side::opposite(Side::Both), Side::Both);
    }

    #[test]
    fn is_white() {
        assert!(Side::White.is_white());
        assert!(!Side::Black.is_white());
        assert!(!Side::Both.is_white());
    }

    #[test]
    fn is_black() {
        assert!(!Side::White.is_black());
        assert!(Side::Black.is_black());
        assert!(!Side::Both.is_black());
    }

    #[test]
    fn is_both() {
        assert!(!Side::White.is_both());
        assert!(!Side::Black.is_both());
        assert!(Side::Both.is_both());
    }
}
