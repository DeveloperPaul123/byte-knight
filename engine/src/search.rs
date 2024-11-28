/*
 * search.rs
 * Part of the byte-knight project
 * Created Date: Thursday, November 21st 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Thu Nov 28 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::{
    fmt::Display,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use chess::{board::Board, move_generation::MoveGenerator, move_list::MoveList, moves::Move};
use itertools::Itertools;
use uci_parser::{UciInfo, UciResponse, UciSearchOptions};

use crate::{
    evaluation::Evaluation,
    score::Score,
    tt_table::{EntryFlag, TranspositionTable, TranspositionTableEntry},
};

const MAX_DEPTH: u8 = 128;

/// Result for a search.
#[derive(Clone, Copy, Debug)]
pub struct SearchResult {
    pub score: Score,
    pub best_move: Option<Move>,
    pub nodes: u64,
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

/// Input parameters for the search.
#[derive(Clone, Debug)]
pub struct SearchParameters {
    pub max_depth: u8,
    pub start_time: Instant,
    pub soft_timeout: Duration,
    pub hard_timeout: Duration,
    pub max_nodes: u64,
}

impl Default for SearchParameters {
    fn default() -> Self {
        SearchParameters {
            max_depth: MAX_DEPTH,
            start_time: Instant::now(),
            soft_timeout: Duration::MAX,
            hard_timeout: Duration::MAX,
            max_nodes: u64::MAX,
        }
    }
}

impl SearchParameters {
    /// Creates a new set of search parameters from the UCI options and the current board.
    pub fn new(uci_options: &UciSearchOptions, board: &Board) -> Self {
        let mut params = Self::default();
        if let Some(depth) = uci_options.depth {
            params.max_depth = depth as u8;
        }

        if let Some(nodes) = uci_options.nodes {
            params.max_nodes = nodes as u64;
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

pub struct Search<'search_lifetime> {
    transposition_table: &'search_lifetime mut TranspositionTable,
    move_gen: MoveGenerator,
    nodes: u64,
    parameters: SearchParameters,
    eval: Evaluation,
    stop_flag: Option<Arc<AtomicBool>>,
}

impl<'a> Search<'a> {
    pub fn new(parameters: &SearchParameters, tt_table: &'a mut TranspositionTable) -> Self {
        Search {
            transposition_table: tt_table,
            move_gen: MoveGenerator::new(),
            nodes: 0,
            parameters: parameters.clone(),
            eval: Evaluation::new(),
            stop_flag: None,
        }
    }

    /// Search for the best move in the given board state. This will output
    /// UCI info lines as it searches.
    ///
    /// # Arguments
    ///
    /// - `board` - The current board state.
    /// - `stop_flag` - An optional flag to stop the search.
    ///
    /// # Returns
    ///
    /// The best move found.
    pub fn search(
        &mut self,
        board: &mut Board,
        stop_flag: Option<Arc<AtomicBool>>,
    ) -> SearchResult {
        self.stop_flag = stop_flag;

        let info = UciInfo::default().string(format!("searching {}", self.parameters));
        let message = UciResponse::info(info);
        println!("{}", message);

        let result = self.iterative_deepening(board);
        // search ended, reset our node count
        self.nodes = 0;
        result
    }

    fn should_stop_searching(&self) -> bool {
        self.parameters.start_time.elapsed() >= self.parameters.hard_timeout // hard timeout
        || self.nodes >= self.parameters.max_nodes // node limit reached
        || self.stop_flag.as_ref().is_some_and(|f| f.load(Ordering::Relaxed)) // stop flag set
    }

    fn send_info(
        &self,
        depth: u8,
        nodes: u64,
        score: Score,
        nps: f32,
        time: u64,
        best_move: Option<Move>,
    ) {
        // create UciInfo and print it
        let info = UciInfo::new()
            .depth(depth)
            .nodes(nodes)
            .score(score)
            .nps(nps.trunc())
            .time(time)
            .pv(best_move.map(|m| m.to_long_algebraic()));
        let message = UciResponse::info(info);
        println!("{}", message);
    }

    fn iterative_deepening(&mut self, board: &mut Board) -> SearchResult {
        // initialize the best result
        let mut best_result = SearchResult::default();
        let mut move_list = MoveList::new();
        self.move_gen.generate_legal_moves(board, &mut move_list);
        if !move_list.is_empty() {
            best_result.best_move = Some(*move_list.at(0).unwrap())
        }

        while self.parameters.start_time.elapsed() <= self.parameters.soft_timeout
            && best_result.depth <= self.parameters.max_depth
        {
            // search the tree, starting at the current depth (starts at 1)
            let score = self.negamax(
                board,
                best_result.depth as i64,
                0,
                -Score::INF,
                Score::INF,
                self.parameters.max_depth as i64,
            );

            // check stop conditions
            if self.should_stop_searching() {
                // we have to stop searching now, use the best result we have
                // no score update
                break;
            }

            // update the best result
            best_result.score = score;
            // pull best move from the transposition table
            best_result.best_move = self
                .transposition_table
                .get_entry(board.zobrist_hash())
                .map(|e| e.board_move);

            // send UCI info
            self.send_info(
                best_result.depth,
                self.nodes,
                best_result.score,
                (self.nodes as f32 / self.parameters.start_time.elapsed().as_secs_f32()).trunc(),
                self.parameters.start_time.elapsed().as_millis() as u64,
                best_result.best_move,
            );

            // increment depth for next iteration
            best_result.depth += 1;
        }

        // update total nodes for the current search
        best_result.nodes = self.nodes;

        // send final info line
        self.send_info(
            best_result.depth,
            self.nodes,
            best_result.score,
            (self.nodes as f32 / self.parameters.start_time.elapsed().as_secs_f32()).trunc(),
            self.parameters.start_time.elapsed().as_millis() as u64,
            best_result.best_move,
        );

        // return our best result so far
        best_result
    }

    fn negamax(
        &mut self,
        board: &mut Board,
        depth: i64,
        ply: i64,
        mut alpha: Score,
        mut beta: Score,
        _max_depth: i64,
    ) -> Score {
        // increment node count
        self.nodes += 1;

        // get the tt entry
        let tt_entry = self.transposition_table.get_entry(board.zobrist_hash());
        if let Some(tt) = tt_entry {
            if tt.depth >= depth as u8 {
                match tt.flag {
                    EntryFlag::Exact => {
                        return tt.score;
                    }
                    EntryFlag::LowerBound => {
                        alpha = alpha.max(tt.score);
                    }
                    EntryFlag::UpperBound => {
                        beta = beta.min(tt.score);
                    }
                }
                if alpha >= beta {
                    return tt.score;
                }
            }
        }

        let alpha_original = alpha;
        if depth == 0 {
            return self.quiescence(board, ply, alpha, beta);
        }

        // get all legal moves
        let mut move_list = MoveList::new();
        self.move_gen.generate_legal_moves(board, &mut move_list);

        // do we have moves?
        if move_list.is_empty() {
            if board.is_in_check(&self.move_gen) {
                return -Score::MATE + ply;
            } else {
                return Score::DRAW;
            }
        }

        // sort moves by MVV/LVA
        let sorted_moves = move_list
            .iter()
            .sorted_by_cached_key(|mv| Evaluation::score_move_for_ordering(mv, &tt_entry))
            .collect_vec();

        // initialize best move and best score
        // we ensured we have moves earlier
        let mut best_move = sorted_moves[0];
        // really "bad" initial score
        let mut best_score = -Score::INF;

        // loop through all moves
        for mv in sorted_moves {
            // make the move
            board.make_move_unchecked(mv).unwrap();
            // is it a draw?
            let score = // recursive call and lower depth, higher ply and negated alpha and beta (swapped)
            -self.negamax(board, depth - 1, ply + 1, -beta, -alpha, _max_depth);

            // undo the move
            board.unmake_move().unwrap();

            // check the results
            if score > best_score {
                // we improved, so update the score and best move
                best_score = score;
                best_move = mv;

                // update alpha
                alpha = alpha.max(best_score);
                if alpha >= beta {
                    break;
                }
            }

            // do we need to stop searching?
            if self.should_stop_searching() {
                break;
            }
        }

        // did we fail high or low or did we find the exact score?
        let flag = if best_score <= alpha_original {
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

    /// Implements [quiescence search](https://www.chessprogramming.org/Quiescence_Search).
    /// We use this to avoid the horizon effect. The idea is to evaluate quiet moves where there are no tactical moves to make.
    ///
    /// # Arguments
    ///
    /// - `board` - The current board state.
    /// - `ply` - The current ply.
    /// - `alpha` - The current alpha value.
    /// - `beta` - The current beta value.
    ///
    /// # Returns
    ///
    /// The score of the position.
    ///
    fn quiescence(&mut self, board: &mut Board, _ply: i64, mut alpha: Score, beta: Score) -> Score {
        let standing_eval = self.eval.evaluate_position(board);
        if standing_eval >= beta {
            return beta;
        }

        alpha = alpha.max(standing_eval);

        let mut move_list = MoveList::new();
        self.move_gen.generate_legal_moves(board, &mut move_list);

        // we only want captures here
        let captures = move_list
            .iter()
            .filter(|mv: &&Move| mv.captured_piece().is_some())
            .collect_vec();

        // no captures
        if captures.is_empty() {
            return standing_eval;
        }

        let tt_move = self.transposition_table.get_entry(board.zobrist_hash());
        let sorted_moves = captures
            .into_iter()
            .sorted_by_cached_key(|mv| Evaluation::score_move_for_ordering(mv, &tt_move));
        let mut best = standing_eval;

        for mv in sorted_moves {
            board.make_move_unchecked(mv).unwrap();
            let score = if board.is_draw() {
                Score::DRAW
            } else {
                let eval = -self.quiescence(board, _ply + 1, -beta, -alpha);
                self.nodes += 1;
                eval
            };
            board.unmake_move().unwrap();

            if score > best {
                best = score;

                if score > alpha {
                    alpha = score;
                }

                if score >= beta {
                    break;
                }
            }

            if self.should_stop_searching() {
                break;
            }
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chess::board::Board;

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

        let mut tt = Default::default();

        let mut search = Search::new(&config, &mut tt);
        let res = search.search(&mut board.clone(), None);
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

        let mut tt = Default::default();

        let mut search = Search::new(&config, &mut tt);
        let res = search.search(&mut board, None);

        assert_eq!(res.best_move.unwrap().to_long_algebraic(), "b8a8")
    }

    #[test]
    fn stalemate() {
        let fen = "k7/8/KQ6/8/8/8/8/8 b - - 0 1";
        let mut board = Board::from_fen(fen).unwrap();
        let config = SearchParameters::default();

        let mut tt = Default::default();

        let mut search = Search::new(&config, &mut tt);
        let res = search.search(&mut board, None);
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

        let mut tt = Default::default();

        let mut search = Search::new(&config, &mut tt);
        let res = search.search(&mut board, None);

        assert!(res.best_move.is_some());
        assert!(config.start_time.elapsed() <= config.hard_timeout);
    }

    #[test]
    fn starting_position() {
        let mut board = Board::default_board();
        let config = SearchParameters {
            max_depth: 8,
            ..Default::default()
        };

        let mut tt = Default::default();

        let mut search = Search::new(&config, &mut tt);
        let res = search.search(&mut board, None);
        assert!(res.best_move.is_some());
        println!("{}", res.best_move.unwrap().to_long_algebraic());
    }

    #[test]
    fn no_time() {
        let mut board = Board::from_fen("8/7p/5p2/2K1qp2/7P/8/6k1/4q3 w - - 1 2").unwrap();
        let config = SearchParameters {
            soft_timeout: Duration::from_millis(0),
            hard_timeout: Duration::from_millis(0),
            ..Default::default()
        };

        let mut tt = Default::default();
        let mut search = Search::new(&config, &mut tt);
        let res = search.search(&mut board, None);
        assert!(res.best_move.is_some());
        println!("{}", res.best_move.unwrap().to_long_algebraic());
    }
}
