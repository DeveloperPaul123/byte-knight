use crate::bitboard::Bitboard;

pub const SPACE: &str = " ";
pub const NEWLINE: &str = "\n";
pub const DASH: &str = "-";
pub const SLASH: &str = "/";

pub struct NumberOf;
impl NumberOf {
    pub const PIECE_TYPES: usize = 6;
    pub const SQUARES: usize = 64;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
    pub const SIDES: usize = 2;
}

pub struct File;
impl File {
    pub const A: u8 = 0;
    pub const B: u8 = 1;
    pub const C: u8 = 2;
    pub const D: u8 = 3;
    pub const E: u8 = 4;
    pub const F: u8 = 5;
    pub const G: u8 = 6;
    pub const H: u8 = 7;
}

pub struct Rank;
impl Rank {
    pub const R1: u8 = 0;
    pub const R2: u8 = 1;
    pub const R3: u8 = 2;
    pub const R4: u8 = 3;
    pub const R5: u8 = 4;
    pub const R6: u8 = 5;
    pub const R7: u8 = 6;
    pub const R8: u8 = 7;
}

pub const EMPTY: u64 = 0;

pub struct Side;
impl Side {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
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
