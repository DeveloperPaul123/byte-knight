use crate::{definitions::Square, pieces::Piece, square::to_square};

const MOVE_INFO_FROM_MASK: u16 = 0b1111110000000000;
const MOVE_INFO_FROM_SHIFT: u16 = 10;
const MOVE_INFO_TO_MASK: u16 = 0b0000001111110000;
const MOVE_INFO_TO_SHIFT: u16 = 4;
const MOVE_INFO_FLAGS_MASK: u16 = 0b0000000000001111;
struct Flags;
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

/// Compact, 16-bit move representation
/// Taken from https://github.com/SebLague/Chess-Challenge/blob/main/Chess-Challenge/src/Framework/Chess/Board/Move.cs
#[derive(Debug)]
pub struct Move {
    /// The move information.
    /// The first 6 bits represent the from square.
    /// The next 6 bits represent the to square.
    /// The next 4 bits represent the flags.
    /// fffff tttttt 0000
    move_info: u16,
}

impl Move {
    pub fn default() -> Self {
        Self { move_info: 0 }
    }

    /// Creates a new [`Move`].
    pub fn new(from: Square, to: Square, flags: u8) -> Self {
        let from_index = to_square(from.0 as u8, from.1 as u8) as u16;
        let to_index = to_square(to.0 as u8, to.1 as u8) as u16;
        let move_info =
            (from_index << MOVE_INFO_FROM_SHIFT) | (to_index << MOVE_INFO_TO_SHIFT) | flags as u16;
        Self { move_info }
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
}

#[cfg(test)]
mod move_tests {
    use crate::definitions::{File, Rank};
    use crate::moves::{Flags, Move};
    use crate::pieces::Piece;
    #[test]
    fn test_move_new() {
        {
            let from = (File::B, Rank::R1);
            let to = (File::C, Rank::R2);
            let m = Move::new(from, to, Flags::NONE);
            assert_eq!(m.from(), 1);
            assert_eq!(m.to(), 10);
            assert!(!m.is_promotion());
        }
        {
            let from = (File::H, Rank::R8);
            let to = (File::A, Rank::R8);
            let m = Move::new(from, to, Flags::NONE);
            assert_eq!(m.from(), 63);
            assert_eq!(m.to(), 56);
            assert!(!m.is_promotion());
        }
        {
            let from = (File::A, Rank::R2);
            let to = (File::A, Rank::R4);
            let m = Move::new(
                from,
                to,
                Flags::EN_PASSANT_CAPTURE | Flags::PAWN_TWO_UP | Flags::CASTLE,
            );
            assert_eq!(m.from(), 8);
            assert_eq!(m.to(), 24);
            assert!(m.is_en_passant_capture());
            assert!(m.is_pawn_two_up());
            assert!(m.is_castle());
        }
        {
            let from = (File::A, Rank::R7);
            let to = (File::A, Rank::R8);
            let m = Move::new(from, to, Flags::PROMOTE_TO_QUEEN);
            assert_eq!(m.from(), 48);
            assert_eq!(m.to(), 56);
            assert!(m.is_promote_to_queen());
            assert!(m.is_promotion());
            assert_eq!(m.promotion_piece().unwrap(), Piece::Queen);
        }
        {
            let from = (File::A, Rank::R7);
            let to = (File::A, Rank::R8);
            let m = Move::new(from, to, Flags::PROMOTE_TO_KNIGHT);
            assert_eq!(m.from(), 48);
            assert_eq!(m.to(), 56);
            assert!(m.is_promote_to_knight());
            assert!(m.is_promotion());
            assert_eq!(m.promotion_piece().unwrap(), Piece::Knight);
        }
        {
            let from = (File::A, Rank::R7);
            let to = (File::A, Rank::R8);
            let m = Move::new(from, to, Flags::PROMOTE_TO_ROOK);
            assert_eq!(m.from(), 48);
            assert_eq!(m.to(), 56);
            assert!(m.is_promote_to_rook());
            assert!(m.is_promotion());
            assert_eq!(m.promotion_piece().unwrap(), Piece::Rook);
        }
        {
            let from = (File::A, Rank::R7);
            let to = (File::A, Rank::R8);
            let m = Move::new(from, to, Flags::PROMOTE_TO_BISHOP);
            assert_eq!(m.from(), 48);
            assert_eq!(m.to(), 56);
            assert!(m.is_promote_to_bishop());
            assert!(!m.is_promote_to_rook());
            assert!(!m.is_promote_to_queen());
            assert!(!m.is_promote_to_knight());
            assert!(m.is_promotion());
            assert_eq!(m.promotion_piece().unwrap(), Piece::Bishop);
        }
    }
}
