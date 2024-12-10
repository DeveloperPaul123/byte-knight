/*
 * evaluation.rs
 * Part of the byte-knight project
 * Created Date: Thursday, November 21st 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Tue Dec 10 2024
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
    score::{MoveOrderScoreType, Score},
    ttable::TranspositionTableEntry,
};

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
    ) -> MoveOrderScoreType {
        if tt_entry.is_some_and(|tt| *mv == tt.board_move) {
            return MoveOrderScoreType::MIN;
        }
        let mut score = 0;

        // MVV-LVA for captures
        if mv.is_en_passant_capture() || mv.captured_piece().is_some() {
            // safe to unwrap because we know it's a capture
            score += Self::mvv_lva(mv.captured_piece().unwrap(), mv.piece());
        }

        // negate the score to get the best move first
        -score
    }

    fn mvv_lva(captured: Piece, capturing: Piece) -> MoveOrderScoreType {
        let can_capture = captured != Piece::King && captured != Piece::None;
        ((can_capture as MoveOrderScoreType)
            * (25 * Evaluation::piece_value(captured) - Evaluation::piece_value(capturing)))
            << 16
    }

    pub(crate) fn piece_value(piece: Piece) -> MoveOrderScoreType {
        match piece {
            Piece::King => 0,
            Piece::Queen => 5,
            Piece::Rook => 4,
            Piece::Bishop => 3,
            Piece::Knight => 2,
            Piece::Pawn => 1,
            Piece::None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use chess::{
        moves::{self, Move},
        pieces::{Piece, ALL_PIECES, PIECE_SHORT_NAMES},
        side::Side,
        square::Square,
    };

    use crate::{evaluation::Evaluation, score::MoveOrderScoreType};

    #[test]
    fn mvv_lva_scaling() {
        for captured in ALL_PIECES {
            for capturing in ALL_PIECES {
                let score = Evaluation::mvv_lva(captured, capturing);
                println!(
                    "{} x {} -> {}",
                    PIECE_SHORT_NAMES[capturing as usize],
                    PIECE_SHORT_NAMES[captured as usize],
                    score
                );
                assert!((score as i64) < (MoveOrderScoreType::MIN as i64).abs());
            }
        }
    }

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
            -Evaluation::score_move_for_ordering(&mv, &None),
            Evaluation::mvv_lva(mv.captured_piece().unwrap(), mv.piece())
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
            Evaluation::mvv_lva(mv.captured_piece().unwrap(), mv.piece())
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
            Evaluation::mvv_lva(mv.captured_piece().unwrap(), mv.piece())
        );
    }
}
