use byte_board::{
    board::Board,
    move_generation::MoveGenerator,
    move_list::MoveList,
    moves::{Move, MoveType},
};
use itertools::Itertools;

#[derive(Clone, Copy)]
struct TranspositionTableEntry {
    zobrist: u64,
    depth: i64,
    score: i64,
    flag: i64,
    board_move: Move,
}

impl TranspositionTableEntry {
    pub fn new() -> TranspositionTableEntry {
        TranspositionTableEntry {
            zobrist: 0,
            depth: 0,
            score: 0,
            flag: 0,
            board_move: Move::default(),
        }
    }
}

static TRANSPOSITION_TABLE_SIZE: usize = 1_048_576;

struct TranspositionTable {
    table: Vec<TranspositionTableEntry>,
}

impl TranspositionTable {
    fn new() -> TranspositionTable {
        TranspositionTable {
            table: Vec::with_capacity(TRANSPOSITION_TABLE_SIZE),
        }
    }

    pub fn get_entry(self: &Self, zobrist: u64) -> TranspositionTableEntry {
        let index = zobrist as usize % TRANSPOSITION_TABLE_SIZE;
        return self.table[index];
    }
}

pub(crate) struct Search {
    transposition_table: TranspositionTable,
    best_move: Move,
    move_gen: MoveGenerator,
    move_list: MoveList,
}

impl Search {
    pub fn new() -> Self {
        Search {
            transposition_table: TranspositionTable::new(),
            best_move: Move::default(),
            move_gen: MoveGenerator::new(),
            move_list: MoveList::new(),
        }
    }
    fn quiesce(self: &Self, board: &Board, depth: i64, ply: i64, alpha: i64, beta: i64) -> i64 {
        todo!("Implement quiece search");
    }

    pub(crate) fn search(
        self: &mut Self,
        board: &mut Board,
        mut depth: i64,
        ply: i64,
        mut alpha: i64,
        mut beta: i64,
        allow_null_move: bool,
    ) -> i64 {
        let quiesce_search = depth <= 0;
        let not_root = ply > 0;
        let is_in_check = board.is_in_check(&self.move_gen);
        let not_principle_variation = beta - alpha == 1;
        let can_prune = false;

        if not_root && board.is_stalemate(&self.move_gen) {
            return 0;
        } else {
            let zobrist = board.zobrist_hash();
            let tt_entry = self.transposition_table.get_entry(zobrist);
            if is_in_check {
                depth += 1;
            }

            if tt_entry.zobrist == zobrist && not_root && tt_entry.depth >= depth {
                // 0 = exact, -1 = lower bound, 1 = upper bound
                if tt_entry.flag == 0 {
                    return tt_entry.score;
                } else if tt_entry.flag == 1 {
                    alpha = alpha.max(tt_entry.score);
                } else if tt_entry.flag == 2 {
                    beta = beta.min(tt_entry.score);
                }
                if alpha >= beta {
                    return tt_entry.score;
                }
            }

            let best_score = -9_999_999;
            let move_score_index = 0;

            if quiesce_search {
                return self.quiesce(board, depth, ply, alpha, beta);
            } else if !is_in_check && not_principle_variation {
                // reverse futility pruning
                // TODO: Implement reverse futility pruning
            }

            self.move_list.clear();
            self.move_gen
                .generate_moves(board, &mut self.move_list, MoveType::All);
            let sorted_moves = self.move_list.iter().sorted_by_key(|m| {
                let dest_color = board.color_on(m.to());
                if tt_entry.board_move == **m {
                    return 9_000_000;
                } else if m.captured_piece().is_some() {
                    let capture_piece = m.captured_piece().unwrap();
                    let move_piece = m.piece();
                    return 1_000_000 * ((capture_piece as i64) - (move_piece as i64));
                } else if m.promotion_piece().is_some() {
                    return 10_000;
                } else {
                    return 0;
                }
            });

            if !quiesce_search && self.move_list.len() == 0 {
                return if is_in_check { -100_000 + ply } else { 0 };
            } else {
                let starting_alpha = alpha;
                let moves_searched = 0;

                for mv in sorted_moves {
                    // TODO: futility pruning
                    let result = board.make_move(mv, &self.move_gen);
                    if result.is_err() {
                        // TODO: illegal move
                        continue;
                    }
                }
            }
            return 0;
        }
    }
}
