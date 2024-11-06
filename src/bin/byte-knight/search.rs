use std::{
    fmt::Display,
    i64::MAX,
    time::{Duration, Instant},
};

use byte_board::{
    board::Board,
    move_generation::MoveGenerator,
    move_list::MoveList,
    moves::{Move, MoveType},
};
use uci_parser::UciSearchOptions;

use crate::{evaluation::Evaluation, score::Score, tt_table::TranspositionTable};

const MAX_DEPTH: u8 = 128;

#[derive(Clone, Copy, Debug)]
pub(crate) struct SearchResult {
    pub score: Score,
    pub best_move: Option<Move>,
    pub nodes: u128,
    pub depth: u8,
}

impl SearchResult {
    pub fn new() -> SearchResult {
        SearchResult {
            score: Score::INF,
            best_move: None,
            nodes: 0,
            depth: 0,
        }
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "score {} nodes {} depth {} bestmove {}",
            self.score,
            self.nodes,
            self.depth,
            self.best_move
                .map(|m| m.to_long_algebraic())
                .unwrap_or_else(|| "none".to_string())
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct SearchParameters {
    pub max_depth: u8,
    pub start_time: Instant,
    pub soft_timeout: Duration,
    pub hard_timeout: Duration,
}

impl SearchParameters {
    pub fn default() -> SearchParameters {
        SearchParameters {
            max_depth: MAX_DEPTH,
            start_time: Instant::now(),
            soft_timeout: Duration::MAX,
            hard_timeout: Duration::MAX,
        }
    }

    pub fn new(uci_options: &UciSearchOptions, board: &Board) -> Self {
        let mut params = Self::default();
        if let Some(depth) = uci_options.depth {
            params.max_depth = depth as u8;
        }

        if let Some(time) = uci_options.movetime {
            params.soft_timeout = time;
            params.hard_timeout = time;
        } else {
            let (time, increment) = if board.side_to_move().is_white() {
                (uci_options.wtime, uci_options.winc)
            } else {
                (uci_options.btime, uci_options.binc)
            };

            // do we have valid time
            if let Some(time) = time {
                // TODO: How can we tune these params?
                let inc = increment.unwrap_or(Duration::ZERO) / 2;
                params.soft_timeout = time / 20 + inc;
                params.hard_timeout = time / 5 + inc;
            }
        }

        params
    }
}

impl Default for SearchParameters {
    fn default() -> SearchParameters {
        SearchParameters::default()
    }
}

pub(crate) struct Search {
    transposition_table: TranspositionTable,
    move_gen: MoveGenerator,
    nodes: u128,
}

impl Default for Search {
    fn default() -> Self {
        Search::new()
    }
}

impl Search {
    pub fn new() -> Self {
        Search {
            transposition_table: TranspositionTable::new(),
            move_gen: MoveGenerator::new(),
            nodes: 0,
        }
    }

    pub(crate) fn search(
        self: &mut Self,
        board: &mut Board,
        search_params: &SearchParameters,
    ) -> SearchResult {
        // basic negamax search
        self.iterative_deepening(board, search_params)
    }

    fn iterative_deepening(
        self: &mut Self,
        board: &mut Board,
        params: &SearchParameters,
    ) -> SearchResult {
        // initialize the best result
        let mut best_result = SearchResult::new();
        let best_move = Move::default();

        while params.start_time.elapsed() < params.soft_timeout
            && best_result.depth <= params.max_depth
        {
            let score = self.negamax(
                board,
                params.max_depth as i64,
                0,
                -Score::INF,
                Score::INF,
                &mut best_result,
                params.max_depth as i64,
            );

            best_result.score = score;
            best_result.depth += 1;
            best_result.best_move = if best_move.is_valid() {
                Some(best_move)
            } else {
                None
            };

            println!("info {}", best_result);
        }

        best_result.nodes = self.nodes;
        best_result.depth += 1;
        best_result
    }

    fn negamax(
        self: &mut Self,
        board: &mut Board,
        depth: i64,
        ply: i64,
        alpha: Score,
        beta: Score,
        results: &mut SearchResult,
        max_depth: i64,
    ) -> Score {
        self.nodes += 1;

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
                let score =
                    -self.negamax(board, depth - 1, ply + 1, -beta, -alpha, results, max_depth);
                board.unmake_move().unwrap();

                if score >= best_score {
                    best_score = score;
                    if depth == max_depth {
                        results.best_move = Some(*mv);
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

#[cfg(test)]
mod tests {
    use byte_board::board::Board;

    use crate::search::SearchParameters;

    #[test]
    fn test_white_mate_in_1() {
        let fen = "k7/8/KQ6/8/8/8/8/8 w - - 0 1";
        let board = Board::from_fen(fen).unwrap();
        let config = SearchParameters {
            max_depth: 2,
            ..Default::default()
        };

        let mut search = super::Search::new();
        let res = search.search(&mut board.clone(), &config);
        // b6a7
        assert_eq!(
            res.best_move.unwrap().to_long_algebraic(),
            "b6a7".to_string()
        );
    }
}
