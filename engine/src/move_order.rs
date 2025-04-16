use std::cmp::Ordering;

use chess::{moves::Move, pieces::Piece, side::Side};

use crate::{
    evaluation::Evaluation, hce_values::ByteKnightValues, history_table, score::LargeScoreType,
};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Default)]
pub enum MoveOrder {
    #[default]
    TtMove,
    Capture(Piece, Piece),
    Quiet(LargeScoreType),
}

impl PartialOrd for MoveOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MoveOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // move from the transposition table comes first
            (MoveOrder::TtMove, MoveOrder::TtMove) => Ordering::Equal,
            (MoveOrder::TtMove, _) => Ordering::Less,
            (_, MoveOrder::TtMove) => Ordering::Greater,

            // captures come next, according to MVV/LVA
            (
                MoveOrder::Capture(left_victim, left_attacker),
                MoveOrder::Capture(right_victim, right_attacker),
            ) => {
                let left_value =
                    Evaluation::<ByteKnightValues>::mvv_lva(*left_victim, *left_attacker);
                let right_value =
                    Evaluation::<ByteKnightValues>::mvv_lva(*right_victim, *right_attacker);
                return right_value.cmp(&left_value);
            }
            (MoveOrder::Capture(_, _), _) => Ordering::Less,
            (_, MoveOrder::Capture(_, _)) => Ordering::Greater,

            // quiet moves come last, according to their score
            (MoveOrder::Quiet(left_score), MoveOrder::Quiet(right_score)) => {
                return right_score.cmp(&left_score);
            }
        }
    }
}

impl MoveOrder {
    /// Classify moves for move ordering  purposes.
    pub fn classify(
        stm: Side,
        mv: &Move,
        tt_move: &Option<Move>,
        history_table: &history_table::HistoryTable,
    ) -> Self {
        if tt_move.is_some_and(|tt| *mv == tt) {
            return Self::TtMove;
        }

        if mv.is_capture() {
            let victim = mv.captured_piece().expect("Capture move without victim");
            let attacker = mv.piece();
            return Self::Capture(victim, attacker);
        }

        let score = history_table.get(stm, mv.piece(), mv.to());
        Self::Quiet(score)
    }
}

#[cfg(test)]
mod tests {
    use chess::{board::Board, move_generation::MoveGenerator, move_list::MoveList, moves::Move};
    use itertools::Itertools;

    use crate::{
        move_order::MoveOrder,
        score::Score,
        ttable::{EntryFlag, TranspositionTable, TranspositionTableEntry},
    };

    #[test]
    fn verify_move_ordering() {
        let mut tt = TranspositionTable::from_capacity(10);
        let mut history_table = crate::history_table::HistoryTable::new();

        let move_gen = MoveGenerator::new();
        let mut move_list = MoveList::new();
        let board =
            Board::from_fen("rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKB1R w KQkq - 0 1").unwrap();
        move_gen.generate_legal_moves(&board, &mut move_list);

        assert!(move_list.len() >= 6);
        let depth = 3i32;
        let first_mv = move_list.at(4).unwrap();
        tt.store_entry(TranspositionTableEntry::new(
            board.zobrist_hash(),
            depth as u8,
            Score::new(1234),
            EntryFlag::Exact,
            *first_mv,
        ));

        let second_mv = move_list.at(2).unwrap();
        history_table.update(
            board.side_to_move(),
            second_mv.piece(),
            second_mv.to(),
            300 * depth - 250,
        );
        let tt_entry = tt.get_entry(board.zobrist_hash()).unwrap();
        let tt_move = tt_entry.board_move;
        // sort the moves
        let moves = move_list
            .iter()
            .sorted_by_key(|mv| {
                MoveOrder::classify(board.side_to_move(), mv, &Some(tt_move), &history_table)
            })
            .collect::<Vec<&Move>>();

        assert!(moves.len() >= 6);
        assert_eq!(moves[0], &tt_move);
        // check the order of the moves
    }
}
