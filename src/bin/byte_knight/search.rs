use byte_board::{
    board::Board,
    move_generation::MoveGenerator,
    move_list::MoveList,
    moves::{Move, MoveType},
};

use crate::{evaluation::Evaluation, score::Score};

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
    pub best_move: Move,
    move_gen: MoveGenerator,
    depth: i64,
}

impl Search {
    pub fn new() -> Self {
        Search {
            transposition_table: TranspositionTable::new(),
            best_move: Move::default(),
            move_gen: MoveGenerator::new(),
            depth: 3,
        }
    }

    pub(crate) fn search(self: &mut Self, board: &mut Board) -> Score {
        // basic negamax search
        self.negamax(board, self.depth, -Score::INF, Score::INF)
    }

    fn negamax(self: &mut Self, board: &mut Board, depth: i64, alpha: Score, beta: Score) -> Score {
        if board.is_draw(&self.move_gen) {
            return Score::DRAW;
        }

        if depth == 0 {
            return Evaluation::evaluate_position(board, &self.move_gen);
        }

        let mut move_list = MoveList::new();
        // search the tree
        self.move_gen
            .generate_moves(board, &mut move_list, MoveType::All);
        let mut best_score = -Score::INF;
        for mv in move_list.iter() {
            if board.make_move(mv, &self.move_gen).is_ok() {
                let score = -self.negamax(board, depth - 1, -beta, -alpha);
                board.unmake_move().unwrap();

                if score >= best_score {
                    best_score = score;
                    if depth == self.depth {
                        self.best_move = *mv;
                    }
                }

                let new_alpha = alpha.max(best_score);
                if new_alpha >= beta {
                    break;
                }
            }
        }

        best_score
    }
}
