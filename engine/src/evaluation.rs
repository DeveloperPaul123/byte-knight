/*
 * evaluation.rs
 * Part of the byte-knight project
 * Created Date: Thursday, November 21st 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Mon Dec 09 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use chess::{board::Board, definitions::NumberOf, moves::Move, pieces::Piece};

use crate::{
    psqt::{Psqt, MG_VALUE},
    score::{Score, ScoreType},
    ttable::TranspositionTableEntry,
};

// similar setup to Rustic https://rustic-chess.org/search/ordering/mvv_lva.html
// MVV-LVA (Most Valuable Victim - Least Valuable Attacker) is a heuristic used to order captures.
// MVV_LVA[victim][attacker] = victim_value - attacker_value
const MVV_LVA: [[ScoreType; NumberOf::PIECE_TYPES + 1]; NumberOf::PIECE_TYPES + 1] = [
    [0, 0, 0, 0, 0, 0, 0],             // victim K, attacker K, Q, R, B, N, P, None
    [500, 510, 520, 530, 540, 550, 0], // victim Q, attacker K, Q, R, B, N, P, None
    [400, 410, 420, 430, 440, 450, 0], // victim R, attacker K, Q, R, B, N, P, None
    [300, 310, 320, 330, 340, 350, 0], // victim B, attacker K, Q, R, B, N, P, None
    [200, 210, 220, 230, 240, 250, 0], // victim N, attacker K, Q, R, B, N, P, None
    [100, 110, 120, 130, 140, 150, 0], // victim P, attacker K, Q, R, B, N, P, None
    [0, 0, 0, 0, 0, 0, 0],             // victim None, attacker K, Q, R, B, N, P, None
];

/// Provides static evaluation of a given chess position.
pub struct Evaluation {
    psqt: Psqt,
}

impl Default for Evaluation {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluation {
    pub fn new() -> Self {
        Evaluation { psqt: Psqt::new() }
    }

    /// Evaluates the given position.
    ///
    /// # Arguments
    ///
    /// - `board`: The [`Board`] to evaluate.
    pub(crate) fn evaluate_position(&self, board: &Board) -> Score {
        self.psqt.evaluate(board)
    }

    /// Scores a move for ordering. This will return the _negative_ score of
    /// the move so that if you sort moves by their score, the best move will
    /// be first (at index 0).
    ///
    /// # Arguments
    ///
    /// - `mv`: The move to score.
    /// - `tt_entry`: The transposition table entry for the current position.
    ///
    /// # Returns
    ///
    /// The score of the move.
    pub(crate) fn score_move_for_ordering(
        mv: &Move,
        tt_entry: &Option<TranspositionTableEntry>,
    ) -> Score {
        if tt_entry.is_some_and(|tt| *mv == tt.board_move) {
            return Score::new(ScoreType::MIN);
        }
        let mut score = Score::new(0);

        // MVV-LVA for captures
        if mv.is_en_passant_capture() || mv.captured_piece().is_some() {
            // safe to unwrap because we know it's a capture
            // TODO: Tune/adjust the victim multiplier. Roughly we scale so that PxQ is worth the most
            // We roughly scale the value of the victim by 40 so that the max value is 40 *800 = 32000, which is still less than a TT match
            score += 34 * Evaluation::piece_value(mv.captured_piece().unwrap())
                - Evaluation::piece_value(mv.piece());
        }

        // negate the score to get the best move first
        -score
    }

    pub(crate) fn piece_value(piece: Piece) -> ScoreType {
        MG_VALUE[piece as usize]
    }
}

#[cfg(test)]
mod tests {
    use chess::{
        moves::{self, Move},
        pieces::Piece,
        square::Square,
    };

    use crate::{evaluation::Evaluation, score::Score};

    #[test]
    fn score_moves() {
        let from = Square::from_square_index(0);
        let to = Square::from_square_index(1);
        let mut mv = Move::new(
            &from,
            &to,
            moves::MoveDescriptor::None,
            Piece::Pawn,
            Some(Piece::Queen),
            None,
        );

        // note that these scores are for ordering, so they are negated
        assert_eq!(
            -Evaluation::score_move_for_ordering(&mv, &None),
            Score::new(550)
        );

        mv = Move::new(
            &from,
            &to,
            moves::MoveDescriptor::None,
            Piece::Bishop,
            Some(Piece::Rook),
            None,
        );

        assert_eq!(
            -Evaluation::score_move_for_ordering(&mv, &None),
            Score::new(430)
        );

        mv = Move::new(
            &from,
            &to,
            moves::MoveDescriptor::None,
            Piece::Knight,
            Some(Piece::Pawn),
            None,
        );

        assert_eq!(
            -Evaluation::score_move_for_ordering(&mv, &None),
            Score::new(140)
        );
    }
}
