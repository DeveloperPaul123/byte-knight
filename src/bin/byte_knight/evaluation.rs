use byte_board::{board::Board, move_generation::MoveGenerator, pieces::Piece, side::Side};

use crate::score::Score;

pub struct Evaluation;

impl Evaluation {
    pub fn evaluate_position(board: &Board, move_gen: &MoveGenerator) -> Score {
        if board.is_in_check(move_gen) {
            return if board.side_to_move() == Side::White {
                -Score::INF
            } else {
                Score::INF
            };
        }
        let mut sum: i64 = 0;
        for piece in [
            Piece::King,
            Piece::Bishop,
            Piece::Knight,
            Piece::Pawn,
            Piece::Queen,
            Piece::Rook,
        ]
        .into_iter()
        {
            let black_bb = board.piece_bitboard(piece, Side::Black);
            let white_bb = board.piece_bitboard(piece, Side::White);
            let piece_value = match piece {
                Piece::Pawn => 1,
                Piece::Knight => 3,
                Piece::Bishop => 3,
                Piece::Rook => 5,
                Piece::Queen => 9,
                Piece::King => 0,
                Piece::None => 0,
            };
            sum += (black_bb.as_number().count_ones() as i64
                - white_bb.as_number().count_ones() as i64)
                * piece_value;
        }

        let score_mult = if board.side_to_move() == Side::White {
            1
        } else {
            -1
        };

        return Score::new(sum * score_mult);
    }
}
