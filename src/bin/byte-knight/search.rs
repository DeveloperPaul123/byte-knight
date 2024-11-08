use std::{
    fmt::Display,
    i64::MAX,
    time::{Duration, Instant},
};

use byte_board::{
    board::Board,
    definitions::DEFAULT_FEN,
    move_generation::MoveGenerator,
    move_list::MoveList,
    moves::{Move, MoveType},
};
use itertools::Itertools;
use uci_parser::{UciInfo, UciResponse, UciSearchOptions};

use crate::{
    evaluation::Evaluation,
    score::Score,
    tt_table::{EntryFlag, TranspositionTable, TranspositionTableEntry},
};

const MAX_DEPTH: u8 = 128;

#[derive(Clone, Copy, Debug)]
pub(crate) struct SearchResult {
    pub score: Score,
    pub best_move: Option<Move>,
    pub nodes: u128,
    pub depth: u8,
}

impl Default for SearchResult {
    fn default() -> SearchResult {
        SearchResult {
            score: -Score::INF,
            best_move: None,
            nodes: 0,
            depth: 1,
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

impl Display for SearchParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "max depth {} start_time {:?} soft_timeout {:?} hard_timeout {:?}",
            self.max_depth, self.start_time, self.soft_timeout, self.hard_timeout
        )
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
    parameters: SearchParameters,
}

impl Default for Search {
    fn default() -> Self {
        Search::new(SearchParameters::default())
    }
}

impl Search {
    pub fn new(parameters: SearchParameters) -> Self {
        Search {
            transposition_table: TranspositionTable::from_size_in_mb(64),
            move_gen: MoveGenerator::new(),
            nodes: 0,
            parameters,
        }
    }

    pub(crate) fn search(self: &mut Self, board: &mut Board) -> SearchResult {
        let info = UciInfo::default().string(format!("searching {}", self.parameters));
        let message = UciResponse::info(info);
        println!("{}", message);

        let result = self.iterative_deepening(board);
        self.nodes = 0;
        result
    }

    fn should_stop_searching(self: &Self) -> bool {
        self.parameters.start_time.elapsed() >= self.parameters.hard_timeout
    }

    fn iterative_deepening(self: &mut Self, board: &mut Board) -> SearchResult {
        // initialize the best result
        let mut best_result = SearchResult::default();

        while self.parameters.start_time.elapsed() < self.parameters.soft_timeout
            && best_result.depth <= self.parameters.max_depth
        {
            let score = self.negamax(
                board,
                best_result.depth as i64,
                0,
                -Score::INF,
                Score::INF,
                self.parameters.max_depth as i64,
            );

            if self.should_stop_searching() {
                // we have to stop searching now, use the best result we have
                // no score update
                break;
            }

            best_result.score = score;
            best_result.best_move = self
                .transposition_table
                .get_entry(board.zobrist_hash())
                .map(|e| e.board_move);

            // create UciInfo and print it
            let info = UciInfo::new()
                .depth(best_result.depth)
                .nodes(self.nodes)
                .score(best_result.score)
                .nps(
                    (self.nodes as f32 / self.parameters.start_time.elapsed().as_secs_f32())
                        .trunc(),
                )
                .time(self.parameters.start_time.elapsed().as_millis() as u64)
                .pv(best_result.best_move.map(|m| m.to_long_algebraic()));
            let message = UciResponse::info(info);
            println!("{}", message);

            // increment depth for next move
            best_result.depth += 1;
        }

        // update total nodes for the current search
        best_result.nodes = self.nodes;
        // return our best result so far
        best_result
    }

    fn negamax(
        self: &mut Self,
        board: &Board,
        depth: i64,
        ply: i64,
        mut alpha: Score,
        beta: Score,
        max_depth: i64,
    ) -> Score {
        self.nodes += 1;

        if depth == 0 {
            return self.quiescence(board, ply, alpha, beta);
        }

        // get all legal moves
        let mut move_list = MoveList::new();
        // search the tree
        self.move_gen.generate_legal_moves(board, &mut move_list);

        // do we have moves?
        if move_list.len() == 0 {
            if board.is_in_check(&self.move_gen) {
                return Score::new(-Score::MATE.0 + ply);
            } else {
                return Score::new(0);
            }
        }

        // get the tt entry
        let tt_entry = self.transposition_table.get_entry(board.zobrist_hash());
        match tt_entry {
            Some(tt) => {
                if tt.zobrist == board.zobrist_hash()
                    && tt.depth >= depth as u8
                    && (tt.flag == EntryFlag::Exact
                        || tt.flag == EntryFlag::LowerBound && tt.score >= beta
                        || tt.flag == EntryFlag::UpperBound && tt.score <= alpha)
                {
                    return tt.score;
                }
            }
            None => {} // do nothing
        }

        let mut best_move = move_list.at(0).unwrap();
        let mut best_score = -Score::INF;

        // sort moves
        let sorted_moves = move_list
            .iter()
            .sorted_by_cached_key(|mv| Evaluation::score_moves_for_ordering(mv, &tt_entry));

        // loop through all moves
        for mv in sorted_moves {
            let mut new_board = board.clone();

            if new_board.make_move_unchecked(mv).is_ok() {
                let score = if new_board.is_draw() {
                    Score::DRAW
                } else {
                    -self.negamax(&mut new_board, depth - 1, ply + 1, -beta, -alpha, max_depth)
                };

                if score > best_score {
                    best_score = score;
                    if depth == max_depth {
                        best_move = mv;
                    }
                }

                alpha = alpha.max(best_score);
                if alpha >= beta {
                    break;
                }
            }

            if self.should_stop_searching() {
                break;
            }
        }

        // did we fail high or low or did we find the exact score?
        let flag = if best_score <= alpha {
            EntryFlag::UpperBound
        } else if best_score >= beta {
            EntryFlag::LowerBound
        } else {
            EntryFlag::Exact
        };

        // save move to transposition table
        self.transposition_table
            .store_entry(TranspositionTableEntry::new(
                board,
                depth as u8,
                best_score,
                flag,
                *best_move,
            ));
        best_score
    }

    fn quiescence(
        self: &mut Self,
        board: &Board,
        ply: i64,
        mut alpha: Score,
        beta: Score,
    ) -> Score {
        let standing_eval = Evaluation::evaluate_position(board, &self.move_gen);
        if standing_eval >= beta {
            return beta;
        }

        alpha = alpha.max(standing_eval);

        let mut move_list = MoveList::new();
        self.move_gen.generate_legal_moves(board, &mut move_list);

        let captures = move_list
            .iter()
            .filter(|mv| mv.captured_piece().is_some())
            .collect_vec();

        if captures.len() == 0 {
            return standing_eval;
        }

        self.nodes += 1;

        let tt_move = self.transposition_table.get_entry(board.zobrist_hash());
        let sorted_moves = captures
            .iter()
            .sorted_by_cached_key(|mv| Evaluation::score_moves_for_ordering(*mv, &tt_move));

        let mut best_score = standing_eval;

        for mv in sorted_moves {
            let mut new_board = board.clone();
            new_board.make_move_unchecked(mv).unwrap();
            let score;

            if new_board.is_draw() {
                return Score::DRAW;
            } else {
                score = -self.quiescence(&mut new_board, ply + 1, -beta, -alpha);
            }

            if score > best_score {
                best_score = score;
                if best_score > alpha {
                    alpha = best_score;
                }

                if best_score >= beta {
                    break;
                }
            }

            if self.should_stop_searching() {
                break;
            }
        }

        best_score
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use byte_board::board::Board;

    use crate::{
        score::Score,
        search::{Search, SearchParameters},
    };

    #[test]
    fn white_mate_in_1() {
        let fen = "k7/8/KQ6/8/8/8/8/8 w - - 0 1";
        let board = Board::from_fen(fen).unwrap();
        let config = SearchParameters {
            max_depth: 2,
            ..Default::default()
        };

        let mut search = Search::new(config);
        let res = search.search(&mut board.clone());
        // b6a7
        assert_eq!(
            res.best_move.unwrap().to_long_algebraic(),
            "b6a7".to_string()
        );
    }

    #[test]
    fn black_mated_in_1() {
        let fen = "1k6/8/KQ6/2Q5/8/8/8/8 b - - 0 1";
        let mut board = Board::from_fen(fen).unwrap();
        let config = SearchParameters {
            max_depth: 3,
            ..Default::default()
        };

        let mut search = Search::new(config);
        let res = search.search(&mut board);

        assert_eq!(res.best_move.unwrap().to_long_algebraic(), "b8a8")
    }

    #[test]
    fn stalemate() {
        let fen = "k7/8/KQ6/8/8/8/8/8 b - - 0 1";
        let mut board = Board::from_fen(&fen).unwrap();
        let config = SearchParameters::default();

        let mut search = Search::new(config);
        let res = search.search(&mut board);
        assert!(res.best_move.is_none());
        assert_eq!(res.score, Score::DRAW);
    }

    #[test]
    fn do_not_exceed_time() {
        let mut board = Board::default_board();
        let config = SearchParameters {
            soft_timeout: Duration::from_millis(100),
            hard_timeout: Duration::from_millis(1000),
            ..Default::default()
        };

        let mut search = Search::new(config);
        let res = search.search(&mut board);

        assert!(res.best_move.is_some());
        assert!(config.start_time.elapsed() <= config.hard_timeout);
    }
}
