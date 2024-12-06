/*
 * evaluation.rs
 * Part of the byte-knight project
 * Created Date: Thursday, November 21st 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Mon Dec 02 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use chess::{board::Board, definitions::NumberOf, moves::Move, pieces::Piece, side::Side};

use crate::{
    history_table,
    psqt::Psqt,
    score::{Score, ScoreType},
    ttable::TranspositionTableEntry,
};

// similar setup to Rustic https://rustic-chess.org/search/ordering/mvv_lva.html
// MVV-LVA (Most Valuable Victim - Least Valuable Attacker) is a heuristic used to order captures.
// MVV_LVA[victim][attacker] = victim_value - attacker_value
const MVV_LVA: [[ScoreType; NumberOf::PIECE_TYPES + 1]; NumberOf::PIECE_TYPES + 1] = [
    [0, 0, 0, 0, 0, 0, 0],             // victim K, attacker K, Q, R, B, N, P, None
    [30000, 30100, 30200, 30300, 30400, 30500, 0], // victim Q, attacker K, Q, R, B, N, P, None
    [28000, 28100, 28200, 28300, 28400, 28500, 0], // victim R, attacker K, Q, R, B, N, P, None
    [26000, 26100, 26200, 26300, 26400, 26500, 0], // victim B, attacker K, Q, R, B, N, P, None
    [23000, 23100, 23200, 23300, 23400, 23500, 0], // victim N, attacker K, Q, R, B, N, P, None
    [15000, 16000, 17000, 18000, 19000, 20000, 0], // victim P, attacker K, Q, R, B, N, P, None
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
        stm: Side,
        mv: &Move,
        tt_entry: &Option<TranspositionTableEntry>,
        history_table: &history_table::HistoryTable,
    ) -> Score {
        if tt_entry.is_some_and(|tt| *mv == tt.board_move) {
            return Score::new(ScoreType::MIN);
        }
        let mut score = Score::new(0);

        if mv.is_quiet() {
            // add the score from the history table
            score += history_table.get(stm, mv.piece(), mv.to());
        } else if mv.captured_piece().is_some() {
            // score by MVV-LVA
            score +=
                MVV_LVA[mv.captured_piece().unwrap_or(Piece::None) as usize][mv.piece() as usize];
        }

        // negate the score to get the best move first
        -score
    }
}

#[cfg(test)]
mod tests {
    use chess::{
        moves::{self, Move},
        pieces::Piece,
        side::Side,
        square::Square,
    };

    use crate::{evaluation::Evaluation, history_table, score::Score};

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
        let side = Side::Black;
        let history_table = Default::default();
        // note that these scores are for ordering, so they are negated
        assert_eq!(
            -Evaluation::score_move_for_ordering(side, &mv, &None, &history_table),
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
            -Evaluation::score_move_for_ordering(side, &mv, &None, &history_table),
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
            -Evaluation::score_move_for_ordering(side, &mv, &None, &history_table),
            Score::new(140)
        );
    }
}
