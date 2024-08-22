pub const SPACE: &str = " ";
pub const NEWLINE: &str = "\n";
pub const DASH: &str = "-";
pub const EM_DASH: char = '–';
pub const SLASH: &str = "/";

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

pub const EMPTY: u64 = 0;

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    White = 0,
    Black = 1,
    Both = 2,
}

impl Side {
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
