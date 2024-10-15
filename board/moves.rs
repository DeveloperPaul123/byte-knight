/*
 * moves.rs
 * Part of the byte-knight project
 * Created Date: Monday, August 19th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Tue Oct 15 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::fmt::Display;

use crate::{
    pieces::{Piece, PIECE_SHORT_NAMES, SQUARE_NAME},
    square::{to_square, Square},
};
const MOVE_INFO_CAPTURED_PIECE_SHIFT: u32 = 20;
const MOVE_INFO_PIECE_SHIFT: u32 = 17;
const MOVE_INFO_FROM_SHIFT: u32 = 11;
const MOVE_INFO_TO_SHIFT: u32 = 5;
const MOVE_INFO_PROMOTION_DESCRIPTOR_SHIFT: u32 = 3;
const MOVE_INFO_IS_PROMOTION_SHIFT: u32 = 2;

const MOVE_INFO_FROM_MASK: u32 = 0b11111100000000000;
const MOVE_INFO_TO_MASK: u32 = 0b11111100000;
const MOVE_PROMOTION_DESCRIPTOR_MASK: u32 = 0b11000;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveDescriptor {
    None = 0,
    EnPassantCapture,
    Castle,
    PawnTwoUp,
}

impl Display for MoveDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveDescriptor::None => write!(f, "None"),
            MoveDescriptor::EnPassantCapture => write!(f, "EnPassantCapture"),
            MoveDescriptor::Castle => write!(f, "Castle"),
            MoveDescriptor::PawnTwoUp => write!(f, "PawnTwoUp"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PromotionDescriptor {
    Queen,
    Knight,
    Rook,
    Bishop,
}

impl PromotionDescriptor {
    pub(crate) fn to_piece(&self) -> Piece {
        match self {
            PromotionDescriptor::Queen => Piece::Queen,
            PromotionDescriptor::Knight => Piece::Knight,
            PromotionDescriptor::Rook => Piece::Rook,
            PromotionDescriptor::Bishop => Piece::Bishop,
        }
    }
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
    /// The first 2 bits represent the move descriptor
    /// The next 1 bit tells us if the move is a promotion or not
    /// The next 2 bits represent the promotion descriptor
    /// The next 6 bits represent the to square.
    /// The next 6 bits represent the from square.
    /// The next 3 bits represent the piece doing the move.
    /// The next 3 bits represent the captured piece (if any).
    /// The last 9 bits are unused.
    /// 000 000 000 ccc ppp fffff tttttt pp P mm
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
            "{}, type: {}, piece: {}, cap: {}, promo: {}",
            self.to_long_algebraic(),
            self.move_descriptor() as u8,
            self.piece(),
            self.captured_piece().unwrap_or(Piece::None),
            self.promotion_piece().unwrap_or(Piece::None)
        )
    }
}

impl Move {
    /// Creates a new [`Move`].
    pub fn new(
        from: &Square,
        to: &Square,
        descriptor: MoveDescriptor,
        piece: Piece,
        captured_piece: Option<Piece>,
        promotion_piece: Option<Piece>,
    ) -> Self {
        let from_index = to_square(from.file as u8, from.rank as u8) as u32;
        let to_index = to_square(to.file as u8, to.rank as u8) as u32;

        let is_promotion = promotion_piece.is_some();
        let promotion_descriptor = match promotion_piece {
            Some(Piece::Queen) => PromotionDescriptor::Queen as u32,
            Some(Piece::Knight) => PromotionDescriptor::Knight as u32,
            Some(Piece::Rook) => PromotionDescriptor::Rook as u32,
            Some(Piece::Bishop) => PromotionDescriptor::Bishop as u32,
            None => 0,
            _ => 0,
        };
        let move_info = (captured_piece.unwrap_or(Piece::None) as u32)
            << MOVE_INFO_CAPTURED_PIECE_SHIFT
            | (piece as u32) << MOVE_INFO_PIECE_SHIFT
            | (from_index << MOVE_INFO_FROM_SHIFT)
            | (to_index << MOVE_INFO_TO_SHIFT)
            | (promotion_descriptor << MOVE_INFO_PROMOTION_DESCRIPTOR_SHIFT)
            | (is_promotion as u32) << MOVE_INFO_IS_PROMOTION_SHIFT
            | descriptor as u32;
        Self { move_info }
    }

    pub fn new_castle(king_from: &Square, king_to: &Square) -> Self {
        return Self::new(
            king_from,
            king_to,
            MoveDescriptor::Castle,
            Piece::King,
            None,
            None,
        );
    }

    /// Returns the from [`Square`] of the move.
    pub fn from(&self) -> u8 {
        return ((self.move_info & MOVE_INFO_FROM_MASK) >> MOVE_INFO_FROM_SHIFT) as u8;
    }

    /// Returns the to [`Square`] of the move.
    pub fn to(&self) -> u8 {
        return ((self.move_info & MOVE_INFO_TO_MASK) >> MOVE_INFO_TO_SHIFT) as u8;
    }

    pub fn move_descriptor(&self) -> MoveDescriptor {
        return match self.move_info & 0b11 {
            0 => MoveDescriptor::None,
            1 => MoveDescriptor::EnPassantCapture,
            2 => MoveDescriptor::Castle,
            3 => MoveDescriptor::PawnTwoUp,
            _ => MoveDescriptor::None,
        };
    }

    pub fn is_en_passant_capture(&self) -> bool {
        return self.move_descriptor() == MoveDescriptor::EnPassantCapture;
    }

    pub fn is_castle(&self) -> bool {
        return self.move_descriptor() == MoveDescriptor::Castle;
    }

    pub fn is_pawn_two_up(&self) -> bool {
        return self.move_descriptor() == MoveDescriptor::PawnTwoUp;
    }

    pub fn promotion_description(&self) -> PromotionDescriptor {
        return match (self.move_info & MOVE_PROMOTION_DESCRIPTOR_MASK)
            >> MOVE_INFO_PROMOTION_DESCRIPTOR_SHIFT
        {
            0 => PromotionDescriptor::Queen,
            1 => PromotionDescriptor::Knight,
            2 => PromotionDescriptor::Rook,
            3 => PromotionDescriptor::Bishop,
            _ => PromotionDescriptor::Queen,
        };
    }

    pub fn is_promote_to_queen(&self) -> bool {
        return self.is_promotion() && self.promotion_description() == PromotionDescriptor::Queen;
    }

    pub fn is_promote_to_knight(&self) -> bool {
        return self.is_promotion() && self.promotion_description() == PromotionDescriptor::Knight;
    }

    pub fn is_promote_to_rook(&self) -> bool {
        return self.is_promotion() && self.promotion_description() == PromotionDescriptor::Rook;
    }

    pub fn is_promote_to_bishop(&self) -> bool {
        return self.is_promotion() && self.promotion_description() == PromotionDescriptor::Bishop;
    }

    pub fn is_promotion(&self) -> bool {
        return (self.move_info >> MOVE_INFO_IS_PROMOTION_SHIFT) & 0b1 == 1;
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

    pub fn to_long_algebraic(&self) -> String {
        let from = SQUARE_NAME[self.from() as usize];
        let to = SQUARE_NAME[self.to() as usize];
        // TODO: Do we need to handle promotion too? yes
        let promotion_piece = self.promotion_piece().unwrap_or(Piece::None);
        format!(
            "{}{}{}",
            from,
            to,
            PIECE_SHORT_NAMES[promotion_piece as usize].to_ascii_lowercase()
        )
        .trim()
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::definitions::{File, Rank};
    use crate::moves::{Move, MoveDescriptor};
    use crate::pieces::Piece;
    use crate::square::Square;
    #[test]
    fn new_move() {
        {
            let from = Square::new(File::B, Rank::R1);
            let to = Square::new(File::C, Rank::R2);
            let m = Move::new(&from, &to, MoveDescriptor::None, Piece::Pawn, None, None);
            assert_eq!(m.from(), 1);
            assert_eq!(m.to(), 10);
            assert!(!m.is_promotion());
            assert_eq!(m.captured_piece(), None);
            assert_eq!(m.piece(), Piece::Pawn);
        }
        {
            let from = Square::new(File::H, Rank::R8);
            let to = Square::new(File::A, Rank::R8);
            let m = Move::new(
                &from,
                &to,
                MoveDescriptor::None,
                Piece::Queen,
                Some(Piece::Rook),
                None,
            );
            assert_eq!(m.from(), 63);
            assert_eq!(m.to(), 56);
            assert!(!m.is_promotion());
            assert_eq!(m.captured_piece().unwrap(), Piece::Rook);
            assert_eq!(m.piece(), Piece::Queen);
        }
        {
            let from = Square::new(File::F, Rank::R4);
            let to = Square::new(File::E, Rank::R6);
            let m = Move::new(
                &from,
                &to,
                MoveDescriptor::EnPassantCapture,
                Piece::Pawn,
                Some(Piece::Pawn),
                None,
            );
            assert_eq!(m.from(), from.to_square_index());
            assert_eq!(m.to(), to.to_square_index());
            assert!(!m.is_pawn_two_up());
            assert!(!m.is_castle());
            assert!(m.is_en_passant_capture());
            assert_eq!(m.captured_piece().unwrap(), Piece::Pawn);
            assert_eq!(m.piece(), Piece::Pawn);
        }
        {
            let from = Square::new(File::A, Rank::R2);
            let to = Square::new(File::A, Rank::R4);
            let m = Move::new(
                &from,
                &to,
                MoveDescriptor::PawnTwoUp,
                Piece::Pawn,
                None,
                None,
            );
            assert_eq!(m.from(), 8);
            assert_eq!(m.to(), 24);
            assert!(!m.is_castle());
            assert!(!m.is_en_passant_capture());
            assert!(m.is_pawn_two_up());
            assert!(m.captured_piece().is_none());
            assert_eq!(m.piece(), Piece::Pawn);
        }
        {
            let from = Square::new(File::A, Rank::R7);
            let to = Square::new(File::A, Rank::R8);
            let m = Move::new(
                &from,
                &to,
                MoveDescriptor::None,
                Piece::Pawn,
                None,
                Some(Piece::Queen),
            );
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
            let m = Move::new(
                &from,
                &to,
                MoveDescriptor::None,
                Piece::Pawn,
                None,
                Some(Piece::Knight),
            );
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
            let m = Move::new(
                &from,
                &to,
                MoveDescriptor::None,
                Piece::Pawn,
                None,
                Some(Piece::Rook),
            );
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
            let m = Move::new(
                &from,
                &to,
                MoveDescriptor::None,
                Piece::Pawn,
                None,
                Some(Piece::Bishop),
            );
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
