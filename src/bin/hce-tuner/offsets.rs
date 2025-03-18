use chess::{definitions::NumberOf, pieces::Piece};

pub(crate) struct Offsets;

pub const PARAMETER_COUNT: usize = Offsets::END as usize;
const PSQT_SIZE: u16 = 384; // 64 * 6

impl Offsets {
    pub const PSQT: u16 = 0;
    pub const END: u16 = Offsets::PSQT + PSQT_SIZE;

    pub(crate) fn offset_for_piece_and_square(square: usize, piece: Piece) -> usize {
        Offsets::PSQT as usize + (piece as usize * NumberOf::SQUARES) + square
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_calculation() {
        // verify that offset calculation is correct
        let sq = 33;
        let piece = Piece::Pawn;
        let offset = Offsets::offset_for_piece_and_square(sq, piece);
        assert_eq!(353, offset)
    }
}
