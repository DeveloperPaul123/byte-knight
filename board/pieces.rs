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
pub const PIECE_SHORT_NAMES: [char; NumberOf::PIECE_TYPES] = ['K', 'Q', 'R', 'B', 'N', 'P'];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    KING = 0,
    QUEEN = 1,
    ROOK = 2,
    BISHOP = 3,
    KNIGHT = 4,
    PAWN = 5,
    NONE = 6,
}

impl Piece {
    /// Returns `true` if the piece is [`KING`].
    ///
    /// [`KING`]: Piece::KING
    #[must_use]
    pub fn is_king(&self) -> bool {
        matches!(self, Self::KING)
    }

    /// Returns `true` if the piece is [`QUEEN`].
    ///
    /// [`QUEEN`]: Piece::QUEEN
    #[must_use]
    pub fn is_queen(&self) -> bool {
        matches!(self, Self::QUEEN)
    }

    /// Returns `true` if the piece is [`ROOK`].
    ///
    /// [`ROOK`]: Piece::ROOK
    #[must_use]
    pub fn is_rook(&self) -> bool {
        matches!(self, Self::ROOK)
    }

    /// Returns `true` if the piece is [`BISHOP`].
    ///
    /// [`BISHOP`]: Piece::BISHOP
    #[must_use]
    pub fn is_bishop(&self) -> bool {
        matches!(self, Self::BISHOP)
    }

    /// Returns `true` if the piece is [`KNIGHT`].
    ///
    /// [`KNIGHT`]: Piece::KNIGHT
    #[must_use]
    pub fn is_knight(&self) -> bool {
        matches!(self, Self::KNIGHT)
    }

    /// Returns `true` if the piece is [`PAWN`].
    ///
    /// [`PAWN`]: Piece::PAWN
    #[must_use]
    pub fn is_pawn(&self) -> bool {
        matches!(self, Self::PAWN)
    }

    /// Returns `true` if the piece is [`NONE`].
    ///
    /// [`NONE`]: Piece::NONE
    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::NONE)
    }
}

impl Default for Piece {
    fn default() -> Self {
        Piece::NONE
    }
}

impl TryFrom<u8> for Piece {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Piece::KING),
            1 => Ok(Piece::QUEEN),
            2 => Ok(Piece::ROOK),
            3 => Ok(Piece::BISHOP),
            4 => Ok(Piece::KNIGHT),
            5 => Ok(Piece::PAWN),
            6 => Ok(Piece::NONE),
            _ => Err(()),
        }
    }
}
