/*
 * definitions.rs
 * Part of the byte-knight project
 * Created Date: Wednesday, August 21st 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Sat Aug 31 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

pub const SPACE: char = ' ';
pub const NEWLINE: char = '\n';
pub const DASH: char = '-';
pub const EM_DASH: char = '–';
pub const SLASH: char = '/';

pub const MAX_MOVE_LIST_SIZE: usize = 256;
pub const MAX_MOVES: usize = 3072;
// see the tests in move_generation.rs for how these numbers were calculated
pub const ROOK_BLOCKER_PERMUTATIONS: usize = 102_400;
pub const BISHOP_BLOCKER_PERMUTATIONS: usize = 5_248;

pub struct NumberOf;
impl NumberOf {
    pub const PIECE_TYPES: usize = 6;
    pub const SQUARES: usize = 64;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
    pub const SIDES: usize = 2;
    pub const CASTLING_OPTIONS: usize = 16;
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl TryFrom<u8> for File {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            3 => Ok(Self::D),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            6 => Ok(Self::G),
            7 => Ok(Self::H),
            _ => Err(()),
        }
    }
}

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

impl TryFrom<u8> for Rank {
    type Error = ();

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
            _ => Err(()),
        }
    }
}

pub const EMPTY: u64 = 0;

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    White = 0,
    Black = 1,
    Both = 2,
}

impl Side {}

impl Side {
    pub fn opposite(side: Side) -> Side {
        match side {
            Side::White => Side::Black,
            Side::Black => Side::White,
            _ => Side::Both,
        }
    }

    /// Returns `true` if the side is [`WHITE`].
    ///
    /// [`WHITE`]: Side::WHITE
    #[must_use]
    pub fn is_white(&self) -> bool {
        matches!(self, Self::White)
    }

    /// Returns `true` if the side is [`BLACK`].
    ///
    /// [`BLACK`]: Side::BLACK
    #[must_use]
    pub fn is_black(&self) -> bool {
        matches!(self, Self::Black)
    }

    /// Returns `true` if the side is [`BOTH`].
    ///
    /// [`BOTH`]: Side::BOTH
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

pub struct About;
impl About {
    pub const NAME: &'static str = "ByteKnight";
    pub const VERSION: &'static str = "0.1.0";
    pub const SHORT_DESCRIPTION: &'static str = "ByteKnight is a UCI compliant chess engine.";
    pub const AUTHORS: &'static str = "Paul T. (DeveloperPaul123)";
}

pub struct CastlingAvailability;
impl CastlingAvailability {
    pub const NONE: u8 = 0;
    pub const WHITE_KINGSIDE: u8 = 1;
    pub const WHITE_QUEENSIDE: u8 = 2;
    pub const BLACK_KINGSIDE: u8 = 4;
    pub const BLACK_QUEENSIDE: u8 = 8;
    pub const ALL: u8 =
        Self::WHITE_KINGSIDE | Self::WHITE_QUEENSIDE | Self::BLACK_KINGSIDE | Self::BLACK_QUEENSIDE;
}

pub static DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// 0001 0000 0001 0000 0001 0000 0001 0000 0001 0000 0001 0000 0001 0000 0001
// 72,340,172,838,076,673 as decimal
pub const FILE_A: u64 = 0x0101010101010101;
pub const RANK_1: u64 = 0xFF;

pub struct Squares;
impl Squares {
    pub const A1: u8 = 0;
    pub const B1: u8 = 1;
    pub const C1: u8 = 2;
    pub const D1: u8 = 3;
    pub const E1: u8 = 4;
    pub const F1: u8 = 5;
    pub const G1: u8 = 6;
    pub const H1: u8 = 7;

    pub const A2: u8 = 8;
    pub const B2: u8 = 9;
    pub const C2: u8 = 10;
    pub const D2: u8 = 11;
    pub const E2: u8 = 12;
    pub const F2: u8 = 13;
    pub const G2: u8 = 14;
    pub const H2: u8 = 15;

    pub const A3: u8 = 16;
    pub const B3: u8 = 17;
    pub const C3: u8 = 18;
    pub const D3: u8 = 19;
    pub const E3: u8 = 20;
    pub const F3: u8 = 21;
    pub const G3: u8 = 22;
    pub const H3: u8 = 23;

    pub const A4: u8 = 24;
    pub const B4: u8 = 25;
    pub const C4: u8 = 26;
    pub const D4: u8 = 27;
    pub const E4: u8 = 28;
    pub const F4: u8 = 29;
    pub const G4: u8 = 30;
    pub const H4: u8 = 31;

    pub const A5: u8 = 32;
    pub const B5: u8 = 33;
    pub const C5: u8 = 34;
    pub const D5: u8 = 35;
    pub const E5: u8 = 36;
    pub const F5: u8 = 37;
    pub const G5: u8 = 38;
    pub const H5: u8 = 39;

    pub const A6: u8 = 40;
    pub const B6: u8 = 41;
    pub const C6: u8 = 42;
    pub const D6: u8 = 43;
    pub const E6: u8 = 44;
    pub const F6: u8 = 45;
    pub const G6: u8 = 46;
    pub const H6: u8 = 47;

    pub const A7: u8 = 48;
    pub const B7: u8 = 49;
    pub const C7: u8 = 50;
    pub const D7: u8 = 51;
    pub const E7: u8 = 52;
    pub const F7: u8 = 53;
    pub const G7: u8 = 54;
    pub const H7: u8 = 55;

    pub const A8: u8 = 56;
    pub const B8: u8 = 57;
    pub const C8: u8 = 58;
    pub const D8: u8 = 59;
    pub const E8: u8 = 60;
    pub const F8: u8 = 61;
    pub const G8: u8 = 62;
    pub const H8: u8 = 63;
}
