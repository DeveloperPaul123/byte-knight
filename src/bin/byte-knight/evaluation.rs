/*
 * evaluation.rs
 * Part of the byte-knight project
 * Created Date: Thursday, November 21st 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Thu Nov 21 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * 
 */

use chess::{
    board::Board, definitions::NumberOf, move_generation::MoveGenerator, moves::Move,
    pieces::Piece, side::Side,
};

use crate::{psqt::Psqt, score::Score, tt_table::TranspositionTableEntry};

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

/// Returns a value of the provided `PieceKind`.
///
/// Values are obtained from here: <https://www.chessprogramming.org/Simplified_Evaluation_Function>
#[inline(always)]
pub const fn piece_value(kind: Piece) -> i64 {
    match kind {
        Piece::Pawn => 100,
        Piece::Knight => 320,
        Piece::Bishop => 330,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 0,
        Piece::None => 0,
    }
}

pub struct Evaluation {
    psqt: Psqt,
}

impl Evaluation {
    pub fn new() -> Self {
        Evaluation { psqt: Psqt::new() }
    }

    pub(crate) fn evaluate_position(self: &Self, board: &Board) -> Score {
        self.psqt.evaluate(board)
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
