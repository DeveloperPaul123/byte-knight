use chess::{definitions::NumberOf, pieces::Piece, side::Side, square};

pub(crate) struct Offsets;

pub const PARAMETER_COUNT: usize = Offsets::END as usize;
const PSQT_SIZE: u16 = 384; // 64 * 6 (pieces)
const PASSED_PAWN_SIZE: u16 = NumberOf::PASSED_PAWN_RANKS as u16;

impl Offsets {
    pub const PSQT: u16 = 0;
    pub const PASSED_PAWN: u16 = Offsets::PSQT + PSQT_SIZE;
    pub const END: u16 = Offsets::PASSED_PAWN + PASSED_PAWN_SIZE;

    pub(crate) fn offset_for_piece_and_square(square: usize, piece: Piece, side: Side) -> usize {
        Offsets::PSQT as usize
            + (piece as usize * NumberOf::SQUARES)
            + square::flip_if(side == Side::White, square as u8) as usize
    }

    pub(crate) fn offset_for_passed_pawn(square: usize, side: Side) -> usize {
        let (_file, rank) = square::from_square(square::flip_if(side == Side::White, square as u8));
        // Note: File and rank are 0 based
        Offsets::PASSED_PAWN as usize + (rank - 1) as usize
    }
}

#[cfg(test)]
mod tests {
    use chess::{file::File, rank::Rank, square::Square};

    use super::*;

    #[test]
    fn offset_calculation() {
        // verify that offset calculation is correct
        let sq = 33;
        let piece = Piece::Pawn;
        let offset = Offsets::offset_for_piece_and_square(sq, piece, Side::Black);
        assert_eq!(353, offset);
        let offset = Offsets::offset_for_piece_and_square(sq, piece, Side::White);
        assert_eq!(345, offset);
    }

    #[test]
    fn offset_pawn_calculation() {
        let file = File::C;
        let rank = Rank::R2;

        let sq = Square::from_file_rank(file.to_char(), rank.as_number()).unwrap();
        let offset = Offsets::offset_for_passed_pawn(sq.to_square_index() as usize, Side::Black);
        assert_eq!(Offsets::PASSED_PAWN as usize, offset);
        let offset = Offsets::offset_for_passed_pawn(sq.to_square_index() as usize, Side::White);
        assert_eq!(389, offset);
    }
}
