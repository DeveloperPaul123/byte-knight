use chess::{
    board::Board, definitions::NumberOf, move_generation::MoveGenerator, moves::Move,
    pieces::Piece, side::Side,
};

use crate::{score::Score, tt_table::TranspositionTableEntry};

pub struct Evaluation;

// similar setup to Rustic https://rustic-chess.org/search/ordering/mvv_lva.html
// MVV-LVA (Most Valuable Victim - Least Valuable Attacker) is a heuristic used to order captures.
// MVV_LVA[victim][attacker] = victim_value - attacker_value
const MVV_LVA: [[i64; NumberOf::PIECE_TYPES + 1]; NumberOf::PIECE_TYPES + 1] = [
    [0, 0, 0, 0, 0, 0, 0],             // victim K, attacker K, Q, R, B, N, P, None
    [500, 510, 520, 530, 540, 550, 0], // victim Q, attacker K, Q, R, B, N, P, None
    [400, 410, 420, 430, 440, 450, 0], // victim R, attacker K, Q, R, B, N, P, None
    [300, 310, 320, 330, 340, 350, 0], // victim B, attacker K, Q, R, B, N, P, None
    [200, 210, 220, 230, 240, 250, 0], // victim N, attacker K, Q, R, B, N, P, None
    [100, 110, 120, 130, 140, 150, 0], // victim P, attacker K, Q, R, B, N, P, None
    [0, 0, 0, 0, 0, 0, 0],             // victim None, attacker K, Q, R, B, N, P, None
];

impl Evaluation {
    pub(crate) fn evaluate_position(board: &Board, move_gen: &MoveGenerator) -> Score {
        let score = if board.is_in_check(move_gen) {
            Score::MATE
        } else if board.is_checkmate(move_gen) {
            Score::INF
        } else if board.is_draw() {
            Score::DRAW
        } else {
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
                sum += (white_bb.number_of_occupied_squares() as i64
                    - black_bb.number_of_occupied_squares() as i64)
                    * piece_value;
            }
            Score::new(sum)
        };

        if board.side_to_move() == Side::White {
            score
        } else {
            -score
        }
    }

    pub(crate) fn score_moves_for_ordering(
        mv: &Move,
        tt_entry: &Option<TranspositionTableEntry>,
    ) -> Score {
        if tt_entry.is_some_and(|tt| *mv == tt.board_move) {
            return -Score::INF;
        }
        let mut score = Score::new(0);

        score += MVV_LVA[mv.captured_piece().unwrap_or(Piece::None) as usize][mv.piece() as usize];

        // negate the score to get the best move first
        -score
    }
}
