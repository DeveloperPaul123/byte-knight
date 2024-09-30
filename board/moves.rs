/*
 * moves.rs
 * Part of the byte-knight project
 * Created Date: Monday, August 19th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Sat Aug 31 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::fmt::Display;

use crate::{
    pieces::{Piece, SQUARE_NAME},
    square::{to_square, Square},
};
const MOVE_INFO_CAPTURED_PIECE_SHIFT: u32 = 19;
const MOVE_INFO_PIECE_SHIFT: u32 = 16;
const MOVE_INFO_FROM_MASK: u32 = 0b1111110000000000;
const MOVE_INFO_FROM_SHIFT: u32 = 10;
const MOVE_INFO_TO_MASK: u32 = 0b1111110000;
const MOVE_INFO_TO_SHIFT: u32 = 4;
const MOVE_INFO_FLAGS_MASK: u32 = 0b1111;

pub struct Flags;
impl Flags {
    pub const NONE: u8 = 0b0000;
    pub const EN_PASSANT_CAPTURE: u8 = 0b0001;
    pub const CASTLE: u8 = 0b0010;
    pub const PAWN_TWO_UP: u8 = 0b0011;
    pub const PROMOTE_TO_QUEEN: u8 = 0b0100;
    pub const PROMOTE_TO_KNIGHT: u8 = 0b0101;
    pub const PROMOTE_TO_ROOK: u8 = 0b0110;
    pub const PROMOTE_TO_BISHOP: u8 = 0b0111;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveType {
    Quiet,
    Capture,
    All,
}

/// Compact, 32-bit move representation
/// Taken from https://github.com/SebLague/Chess-Challenge/blob/main/Chess-Challenge/src/Framework/Chess/Board/Move.cs
/// Also inspired by Rustic's move representation: https://github.com/mvanthoor/rustic/blob/master/src/movegen/defs.rs
#[derive(Debug, Clone, Copy)]
pub struct Move {
    /// The move information, from LSB to MSB:
    /// The first 4 bits represent the flags.
    /// The next 6 bits represent the to square.
    /// The next 6 bits represent the from square.
    /// The next 3 bits represent the piece doing the move.
    /// The next 3 bits represent the captured piece (if any).
    /// The last 10 bits are unused.
    /// 00000000 cccppp fffff tttttt 0000
    move_info: u32,
}

impl Default for Move {
    fn default() -> Self {
        Self { move_info: 0 }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Move: from: {}, to: {}, flags: {}, piece: {}, captured_piece: {}",
            SQUARE_NAME[self.from() as usize],
            SQUARE_NAME[self.to() as usize],
            self.flags(),
            self.piece(),
            self.captured_piece().unwrap_or(Piece::None)
        )
    }
}

impl Move {
    /// Creates a new [`Move`].
    pub fn new(
        from: &Square,
        to: &Square,
        flags: u8,
        piece: Piece,
        captured_piece: Option<Piece>,
    ) -> Self {
        let from_index = to_square(from.file as u8, from.rank as u8) as u32;
        let to_index = to_square(to.file as u8, to.rank as u8) as u32;

        let move_info = (captured_piece.unwrap_or(Piece::None) as u32)
            << MOVE_INFO_CAPTURED_PIECE_SHIFT
            | (piece as u32) << MOVE_INFO_PIECE_SHIFT
            | (from_index << MOVE_INFO_FROM_SHIFT)
            | (to_index << MOVE_INFO_TO_SHIFT)
            | flags as u32;
        Self { move_info }
    }

    pub fn new_castle(king_from: &Square, king_to: &Square) -> Self {
        let king_from_index = to_square(king_from.file as u8, king_from.rank as u8) as u32;
        let king_to_index = to_square(king_to.file as u8, king_to.rank as u8) as u32;

        let move_info = (Piece::None as u32) << MOVE_INFO_CAPTURED_PIECE_SHIFT
            | (Piece::King as u32) << MOVE_INFO_PIECE_SHIFT
            | (king_from_index << MOVE_INFO_FROM_SHIFT)
            | (king_to_index << MOVE_INFO_TO_SHIFT)
            | Flags::CASTLE as u32;

        return Self { move_info };
    }

    /// Returns the from [`Square`] of the move.
    pub fn from(&self) -> u8 {
        return ((self.move_info & MOVE_INFO_FROM_MASK) >> MOVE_INFO_FROM_SHIFT) as u8;
    }

    /// Returns the to [`Square`] of the move.
    pub fn to(&self) -> u8 {
        return ((self.move_info & MOVE_INFO_TO_MASK) >> MOVE_INFO_TO_SHIFT) as u8;
    }

    pub fn flags(&self) -> u8 {
        return (self.move_info & MOVE_INFO_FLAGS_MASK) as u8;
    }

    pub fn is_en_passant_capture(&self) -> bool {
        return (self.flags() & Flags::EN_PASSANT_CAPTURE) != 0;
    }

    pub fn is_castle(&self) -> bool {
        return (self.flags() & Flags::CASTLE) != 0;
    }

    pub fn is_pawn_two_up(&self) -> bool {
        return (self.flags() & Flags::PAWN_TWO_UP) != 0;
    }

    pub fn is_promote_to_queen(&self) -> bool {
        return (self.flags() ^ Flags::PROMOTE_TO_QUEEN) == 0;
    }

    pub fn is_promote_to_knight(&self) -> bool {
        return (self.flags() ^ Flags::PROMOTE_TO_KNIGHT) == 0;
    }

    pub fn is_promote_to_rook(&self) -> bool {
        return (self.flags() ^ Flags::PROMOTE_TO_ROOK) == 0;
    }

    pub fn is_promote_to_bishop(&self) -> bool {
        return (self.flags() ^ Flags::PROMOTE_TO_BISHOP) == 0;
    }

    pub fn is_promotion(&self) -> bool {
        return self.is_promote_to_queen()
            || self.is_promote_to_knight()
            || self.is_promote_to_rook()
            || self.is_promote_to_bishop();
    }

    pub fn promotion_piece(&self) -> Option<Piece> {
        if self.is_promote_to_queen() {
            return Some(Piece::Queen);
        } else if self.is_promote_to_knight() {
            return Some(Piece::Knight);
        } else if self.is_promote_to_rook() {
            return Some(Piece::Rook);
        } else if self.is_promote_to_bishop() {
            return Some(Piece::Bishop);
        } else {
            return None;
        }
    }

    pub fn captured_piece(&self) -> Option<Piece> {
        // shift right and then mask 3 bits
        let piece_value = (self.move_info >> MOVE_INFO_CAPTURED_PIECE_SHIFT) & 0b111 as u32;
        if piece_value == Piece::None as u32 {
            return None;
        }

        return Some(Piece::try_from(piece_value as u8).unwrap());
    }

    pub fn piece(&self) -> Piece {
        // shift right and then mask 3 bits
        let piece_value = (self.move_info >> MOVE_INFO_PIECE_SHIFT) & 0b111 as u32;
        return Piece::try_from(piece_value as u8).unwrap();
    }

    pub(crate) fn is_null_move(&self) -> bool {
        // this is the default value, and should be interpreted as a null move
        // the reason for this is that a move at a minimum should always have a to and from square
        // and a piece. So if there is no information about the move, it is a null move
        return self.move_info == 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::definitions::{File, Rank};
    use crate::moves::{Flags, Move};
    use crate::pieces::Piece;
    use crate::square::Square;
    #[test]
    fn new_move() {
        {
            let from = Square::new(File::B, Rank::R1);
            let to = Square::new(File::C, Rank::R2);
            let m = Move::new(&from, &to, Flags::NONE, Piece::Pawn, None);
            assert_eq!(m.from(), 1);
            assert_eq!(m.to(), 10);
            assert!(!m.is_promotion());
            assert_eq!(m.captured_piece(), None);
            assert_eq!(m.piece(), Piece::Pawn);
        }
        {
            let from = Square::new(File::H, Rank::R8);
            let to = Square::new(File::A, Rank::R8);
            let m = Move::new(&from, &to, Flags::NONE, Piece::Queen, Some(Piece::Rook));
            assert_eq!(m.from(), 63);
            assert_eq!(m.to(), 56);
            assert!(!m.is_promotion());
            assert_eq!(m.captured_piece().unwrap(), Piece::Rook);
            assert_eq!(m.piece(), Piece::Queen);
        }
        {
            let from = Square::new(File::A, Rank::R2);
            let to = Square::new(File::A, Rank::R4);
            let m = Move::new(
                &from,
                &to,
                Flags::EN_PASSANT_CAPTURE | Flags::PAWN_TWO_UP | Flags::CASTLE,
                Piece::Bishop,
                Some(Piece::Rook),
            );
            assert_eq!(m.from(), 8);
            assert_eq!(m.to(), 24);
            assert!(m.is_en_passant_capture());
            assert!(m.is_pawn_two_up());
            assert!(m.is_castle());
            assert_eq!(m.captured_piece().unwrap(), Piece::Rook);
            assert_eq!(m.piece(), Piece::Bishop);
        }
        {
            let from = Square::new(File::A, Rank::R7);
            let to = Square::new(File::A, Rank::R8);
            let m = Move::new(&from, &to, Flags::PROMOTE_TO_QUEEN, Piece::Pawn, None);
            assert_eq!(m.from(), 48);
            assert_eq!(m.to(), 56);
            assert!(m.is_promote_to_queen());
            assert!(m.is_promotion());
            assert_eq!(m.promotion_piece().unwrap(), Piece::Queen);
            assert_eq!(m.captured_piece(), None);
            assert_eq!(m.piece(), Piece::Pawn);
        }
        {
            let from = Square::new(File::A, Rank::R7);
            let to = Square::new(File::A, Rank::R8);
            let m = Move::new(&from, &to, Flags::PROMOTE_TO_KNIGHT, Piece::Pawn, None);
            assert_eq!(m.from(), 48);
            assert_eq!(m.to(), 56);
            assert!(m.is_promote_to_knight());
            assert!(m.is_promotion());
            assert_eq!(m.promotion_piece().unwrap(), Piece::Knight);
            assert_eq!(m.captured_piece(), None);
            assert_eq!(m.piece(), Piece::Pawn);
        }
        {
            let from = Square::new(File::A, Rank::R7);
            let to = Square::new(File::A, Rank::R8);
            let m = Move::new(&from, &to, Flags::PROMOTE_TO_ROOK, Piece::Pawn, None);
            assert_eq!(m.from(), 48);
            assert_eq!(m.to(), 56);
            assert!(m.is_promote_to_rook());
            assert!(m.is_promotion());
            assert_eq!(m.promotion_piece().unwrap(), Piece::Rook);
            assert_eq!(m.captured_piece(), None);
            assert_eq!(m.piece(), Piece::Pawn);
        }
        {
            let from = Square::new(File::A, Rank::R7);
            let to = Square::new(File::A, Rank::R8);
            let m = Move::new(&from, &to, Flags::PROMOTE_TO_BISHOP, Piece::Pawn, None);
            assert_eq!(m.from(), 48);
            assert_eq!(m.to(), 56);
            assert!(m.is_promote_to_bishop());
            assert!(!m.is_promote_to_rook());
            assert!(!m.is_promote_to_queen());
            assert!(!m.is_promote_to_knight());
            assert!(m.is_promotion());
            assert_eq!(m.promotion_piece().unwrap(), Piece::Bishop);
            assert_eq!(m.captured_piece(), None);
            assert_eq!(m.piece(), Piece::Pawn);
        }
    }
}
