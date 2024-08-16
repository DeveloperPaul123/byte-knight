pub struct NumberOf;
impl NumberOf {
    pub const PIECE_TYPES: usize = 6;
    pub const SQUARES: usize = 64;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
    pub const SIDES: usize = 2;
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
