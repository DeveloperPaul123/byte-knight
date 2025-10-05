use chess::{
    bitboard::Bitboard,
    bitboard_helpers,
    board::Board,
    definitions::{FILE_A, NumberOf, RANK_1},
    pieces::Piece,
    side::Side,
    square,
};

pub struct PawnStructure {
    pub passed_pawns: [Bitboard; 2],
}

pub struct PawnEvaluator {
    passed_pawn_masks: [[Bitboard; NumberOf::SQUARES]; NumberOf::SIDES],
}

impl PawnEvaluator {
    pub fn new() -> Self {
        let mut eval = PawnEvaluator {
            passed_pawn_masks: [[Bitboard::default(); NumberOf::SQUARES]; NumberOf::SIDES],
        };

        eval.initialize_pawn_masks();
        eval
    }

    pub fn detect_pawn_structure(&self, board: &Board) -> PawnStructure {
        // Get pawn structure from the board
        let white_pawns = *board.piece_bitboard(Piece::Pawn, Side::White);
        let black_pawns = *board.piece_bitboard(Piece::Pawn, Side::Black);

        // Detect passed pawns
        let mut passed_pawns_w = Bitboard::default();
        let mut passed_pawns_b = Bitboard::default();

        // We need mutable copies to iterate through the pawns since next_bit modifies the bitboard
        let mut white_pawns_mut = white_pawns.clone();
        let mut black_pawns_mut = black_pawns.clone();

        if white_pawns.number_of_occupied_squares() > 0 {
            // Initialize the first pawn square
            // Loop through all white pawns
            let mut white_pawn_sq = bitboard_helpers::next_bit(&mut white_pawns_mut) as u8;
            while white_pawns_mut > 0 {
                let mask = self.passed_pawn_mask(Side::White, white_pawn_sq);
                // Are there any black pawns in the mask? If not, it's a passed pawn
                if (mask & black_pawns).number_of_occupied_squares() == 0 {
                    passed_pawns_w |= Bitboard::from_square(white_pawn_sq);
                }

                // Get the next white pawn square
                white_pawn_sq = bitboard_helpers::next_bit(&mut white_pawns_mut) as u8;
            }
        }

        if black_pawns.number_of_occupied_squares() > 0 {
            // Initialize the first black pawn square
            // Loop through all black pawns
            let mut black_pawn_sq = bitboard_helpers::next_bit(&mut black_pawns_mut) as u8;
            while black_pawns_mut > 0 {
                // Are there any white pawns in the mask? If not, it's a passed pawn
                let mask = self.passed_pawn_mask(Side::Black, black_pawn_sq);
                if (mask & white_pawns).number_of_occupied_squares() == 0 {
                    passed_pawns_b |= Bitboard::from_square(black_pawn_sq);
                }

                // Get the next black pawn square
                black_pawn_sq = bitboard_helpers::next_bit(&mut black_pawns_mut) as u8;
            }
        }

        PawnStructure {
            passed_pawns: [passed_pawns_w, passed_pawns_b],
        }
    }

    fn passed_pawn_mask(&self, side: Side, square: u8) -> Bitboard {
        self.passed_pawn_masks[side as usize][square as usize]
    }

    fn initialize_pawn_masks(&mut self) {
        let files: [Bitboard; NumberOf::FILES] = [
            FILE_A.into(),
            (FILE_A << 1).into(),
            (FILE_A << 2).into(),
            (FILE_A << 3).into(),
            (FILE_A << 4).into(),
            (FILE_A << 5).into(),
            (FILE_A << 6).into(),
            (FILE_A << 7).into(),
        ];

        let ranks: [Bitboard; NumberOf::RANKS] = [
            RANK_1.into(),
            (RANK_1 << 1 * NumberOf::FILES).into(),
            (RANK_1 << 2 * NumberOf::FILES).into(),
            (RANK_1 << 3 * NumberOf::FILES).into(),
            (RANK_1 << 4 * NumberOf::FILES).into(),
            (RANK_1 << 5 * NumberOf::FILES).into(),
            (RANK_1 << 6 * NumberOf::FILES).into(),
            (RANK_1 << 7 * NumberOf::FILES).into(),
        ];

        for sq in 0..NumberOf::SQUARES as u8 {
            let (file, rank) = square::from_square(sq);

            let mut mask_w = Bitboard::default();
            let mut mask_b = Bitboard::default();
            // Mask for white pawns
            // All squares in front of the pawn on the same file and adjacent files
            for r in (rank + 1)..NumberOf::RANKS as u8 {
                if file > 0 {
                    mask_w |= files[(file - 1) as usize] & ranks[r as usize];
                }
                mask_w |= files[file as usize] & ranks[r as usize];
                if file < (NumberOf::FILES as u8 - 1) {
                    mask_w |= files[(file + 1) as usize] & ranks[r as usize];
                }
            }

            // Mask for black pawns
            // All squares in front of the pawn on the same file and adjacent files
            for r in 0..rank {
                if file > 0 {
                    mask_b |= files[(file - 1) as usize] & ranks[r as usize];
                }
                mask_b |= files[file as usize] & ranks[r as usize];
                if file < (NumberOf::FILES as u8 - 1) {
                    mask_b |= files[(file + 1) as usize] & ranks[r as usize];
                }
            }

            self.passed_pawn_masks[Side::White as usize][sq as usize] = mask_w;
            self.passed_pawn_masks[Side::Black as usize][sq as usize] = mask_b;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pawn_structure::PawnEvaluator;

    #[test]
    fn initialization() {
        let pawn_eval = PawnEvaluator::new();
        let board = chess::board::Board::default_board();
        let structure = pawn_eval.detect_pawn_structure(&board);
        assert_eq!(structure.passed_pawns[0].as_number(), 0);
        assert_eq!(structure.passed_pawns[1].as_number(), 0);
    }
}
