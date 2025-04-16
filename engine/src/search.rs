/*
 * search.rs
 * Part of the byte-knight project
 * Created Date: Thursday, November 21st 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Wed Apr 16 2025
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::{
    fmt::Display,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use chess::{
    board::Board, move_generation::MoveGenerator, move_list::MoveList, moves::Move, pieces::Piece,
};
use itertools::Itertools;
use uci_parser::{UciInfo, UciResponse, UciSearchOptions};

use crate::{
    aspiration_window::AspirationWindow,
    defs::MAX_DEPTH,
    evaluation::ByteKnightEvaluation,
    history_table::HistoryTable,
    incremental_sort::IncrementalSort,
    move_order::MoveOrder,
    node_types::{NodeType, NonPvNode, PvNode, RootNode},
    score::{LargeScoreType, Score, ScoreType},
    traits::Eval,
    ttable::{self, TranspositionTableEntry},
    tuneable::{
        IIR_DEPTH_REDUCTION, IIR_MIN_DEPTH, MAX_RFP_DEPTH, NMP_DEPTH_REDUCTION, NMP_MIN_DEPTH,
        RFP_MARGIN,
    },
};
use ttable::TranspositionTable;

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
    history_table: &'search_lifetime mut HistoryTable,
    move_gen: MoveGenerator,
    nodes: u64,
    parameters: SearchParameters,
    eval: ByteKnightEvaluation,
    stop_flag: Option<Arc<AtomicBool>>,
}

impl<'a> Search<'a> {
    pub fn new(
        parameters: &SearchParameters,
        ttable: &'a mut TranspositionTable,
        history_table: &'a mut HistoryTable,
    ) -> Self {
        Search {
            transposition_table: ttable,
            history_table,
            move_gen: MoveGenerator::new(),
            nodes: 0,
            parameters: parameters.clone(),
            eval: ByteKnightEvaluation::default(),
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
            || self.stop_flag.as_ref().is_some_and(|f| f.load(Ordering::Relaxed))
        // stop flag set
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

        'deepening: while self.parameters.start_time.elapsed() <= self.parameters.soft_timeout
            && best_result.depth <= self.parameters.max_depth
        {
            // create an aspiration window around the best result so far
            let mut aspiration_window =
                AspirationWindow::around(best_result.score, best_result.depth as ScoreType);

            let mut score: Score;
            'aspiration_window: loop {
                // search the tree, starting at the current depth (starts at 1)
                score = self.negamax::<RootNode>(
                    board,
                    best_result.depth as ScoreType,
                    0,
                    aspiration_window.alpha(),
                    aspiration_window.beta(),
                );

                if aspiration_window.failed_low(score) {
                    // fail low, widen the window
                    aspiration_window.widen_down(score, best_result.depth as ScoreType);
                } else if aspiration_window.failed_high(score) {
                    // fail high, widen the window
                    aspiration_window.widen_up(score, best_result.depth as ScoreType);
                } else {
                    // we have a valid score, break the loop
                    break 'aspiration_window;
                }

                // check stop conditions
                if self.should_stop_searching() {
                    // we have to stop searching now, use the best result we have
                    // no score update
                    break 'deepening;
                }
            }

            // update the best result
            best_result.score = score;
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

        // return our best result so far
        best_result
    }

    fn negamax<Node>(
        &mut self,
        board: &mut Board,
        mut depth: ScoreType,
        ply: ScoreType,
        alpha: Score,
        beta: Score,
    ) -> Score
    where
        Node: NodeType,
    {
        // increment node count
        self.nodes += 1;
        let alpha_original = alpha;
        let mut alpha_use = alpha;

        if depth == 0 {
            return self.quiescence::<Node>(board, alpha, beta);
        }

        // Transposition Table Cutoffs: https://www.chessprogramming.org/Transposition_Table#Transposition_Table_Cutoffs
        // Check if we have a transposition table entry and if we can return early
        let tt_move =
            match self
                .transposition_table
                .probe::<Node>(depth, board.zobrist_hash(), alpha, beta)
            {
                ttable::ProbeResult::CutOff(entry) => {
                    // we have a cutoff, so return the score, but only in a non-PV node
                    self.nodes += 1;
                    if !Node::PV {
                        return entry.score;
                    }
                    Some(entry.board_move)
                }
                ttable::ProbeResult::Hit(entry) => Some(entry.board_move),
                ttable::ProbeResult::Empty => None,
            };

        // Internal Iterative Reductions: https://www.chessprogramming.org/Internal_Iterative_Reductions
        // If no tt entry was found, searching it will be very costly, so we reduce the depth. This is
        // working under the assumption that the position is likely not important.
        if tt_move.is_none() && depth >= IIR_MIN_DEPTH {
            depth -= IIR_DEPTH_REDUCTION;
        }

        // can we prune the current node with something other than TT?
        if let Some(score) = self.pruned_score::<Node>(board, depth, ply, beta) {
            return score;
        }

        // get all legal moves
        let mut move_list = MoveList::new();
        self.move_gen.generate_legal_moves(board, &mut move_list);

        // do we have moves?
        if move_list.is_empty() {
            return if board.is_in_check(&self.move_gen) {
                -Score::MATE + ply
            } else {
                Score::DRAW
            };
        }

        // sort moves by MVV/LVA
        let sorted_moves = move_list
            .iter()
            .map(|mv| {
                (
                    MoveOrder::classify(board.side_to_move(), &mv, &tt_move, &self.history_table),
                    *mv,
                )
            })
            .collect::<Vec<(MoveOrder, Move)>>();
        let move_iter = IncrementalSort::new(sorted_moves.clone());

        // initialize best move and best score
        // we ensured we have moves earlier
        // let mut best_move = Some(*sorted_moves[0]);

        // really "bad" initial score
        let mut best_score = -Score::INF;
        let mut best_move = None;

        // loop through all moves
        // TODO(PT): Not a fan of this clone() call, but we needed it (for now) for the history malus update later on.
        // This will likely be a non-issue once we implement a move picker
        for (i, mv) in move_iter.into_iter().enumerate() {
            // make the move
            board.make_move_unchecked(&mv).unwrap();
            let score : Score =
                // Principal Variation Search (PVS)
                if Node::PV && i == 0 {
                    -self.negamax::<PvNode>(board, depth - 1, ply + 1, -beta, -alpha_use)
                } else {
                    // search with a null window
                    let temp_score = -self.negamax::<NonPvNode>(board, depth - 1, ply + 1, -alpha_use - 1, -alpha_use);
                    // if it fails, we need to do a full re-search
                    if temp_score > alpha_use && temp_score < beta {
                        -self.negamax::<NonPvNode>(board, depth - 1, ply + 1, -beta, -alpha_use)
                    }
                    else {
                        temp_score
                    }
                };

            // undo the move
            board.unmake_move().unwrap();

            // check the results
            if score > best_score {
                // we improved, so update the score and best move
                best_score = score;
                best_move = Some(mv);

                // update alpha
                alpha_use = alpha_use.max(best_score);
                if alpha_use >= beta {
                    // update history table for quiets
                    if mv.is_quiet() {
                        // calculate history bonus
                        let bonus = 300 * depth - 250;
                        self.history_table.update(
                            board.side_to_move(),
                            mv.piece(),
                            mv.to(),
                            bonus as LargeScoreType,
                        );

                        // apply a penalty to all quiets searched so far
                        for (_, mv) in sorted_moves.iter().take(i).filter(|(_, mv)| mv.is_quiet()) {
                            self.history_table.update(
                                board.side_to_move(),
                                mv.piece(),
                                mv.to(),
                                -bonus as LargeScoreType,
                            );
                        }
                    }
                    break;
                }
            }

            // do we need to stop searching?
            if self.should_stop_searching() {
                break;
            }
        }

        // store the best move in the transposition table
        let flag = if best_score <= alpha_original {
            ttable::EntryFlag::UpperBound
        } else if best_score >= beta {
            ttable::EntryFlag::LowerBound
        } else {
            ttable::EntryFlag::Exact
        };

        self.transposition_table
            .store_entry(TranspositionTableEntry::new(
                board.zobrist_hash(),
                depth as u8,
                best_score,
                flag,
                best_move.unwrap(),
            ));

        best_score
    }

    /// Checks to see if the current node can be pruned. If it can, returns the score. Otherwise returns None.
    ///
    /// # Arguments
    ///
    /// - `board` - The current board state.
    /// - `depth` - The current depth.
    /// - `beta` - The current beta value.
    ///
    /// # Returns
    ///
    /// The score of the position if it can be pruned, otherwise None.
    fn pruned_score<Node: NodeType>(
        &mut self,
        board: &Board,
        depth: ScoreType,
        ply: ScoreType,
        beta: Score,
    ) -> Option<Score> {
        // no pruning if we are in check or if we are in a PV node
        if board.is_in_check(&self.move_gen) || Node::PV {
            return None;
        }

        let static_eval = self.eval.eval(board);
        // Reverse futility pruning
        // https://cosmo.tardis.ac/files/2023-02-20-viri-wiki.html
        // https://www.chessprogramming.org/Reverse_Futility_Pruning
        // If the static evaluation is very high and beats beta by a depth-dependent margin, we can prune the move.
        if depth <= MAX_RFP_DEPTH && static_eval - RFP_MARGIN * depth > beta {
            return Some(static_eval);
        }

        /*
        Null move pruning
        https://www.chessprogramming.org/Null_Move_Pruning
        https://cosmo.tardis.ac/files/2023-02-20-viri-wiki.html
        Give the opponent a free move. If they cannot improve their position (beat beta)
        then prune the tree as our advantage is too great to bother searching further.
        */

        // Are we left with more than just kings and pawns?
        let sufficient_material = (board.all_pieces()
            ^ board.piece_kind_bitboard(Piece::King)
            ^ board.piece_kind_bitboard(Piece::Pawn))
        .number_of_occupied_squares()
            > 0;
        // was the last move null?
        let last_move_was_null = board.last_move().is_some_and(|mv| mv.is_null_move());

        if !last_move_was_null
            && depth >= NMP_MIN_DEPTH
            && static_eval >= beta
            && sufficient_material
        {
            let null_move_depth = depth - NMP_DEPTH_REDUCTION - 1;
            let mut null_board = board.clone();
            null_board.null_move();
            let null_score =
                -self.negamax::<Node>(&mut null_board, null_move_depth, ply + 1, -beta, -beta + 1);
            null_board.unmake_move().unwrap();
            if null_score >= beta {
                return Some(null_score);
            }
        }

        None
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
    fn quiescence<Node: NodeType>(
        &mut self,
        board: &mut Board,
        alpha: Score,
        beta: Score,
    ) -> Score {
        let standing_eval = self.eval.eval(board);
        if standing_eval >= beta {
            return beta;
        }
        let mut alpha_use: Score = alpha.max(standing_eval);

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

        // Transposition Table Cutoffs: https://www.chessprogramming.org/Transposition_Table#Transposition_Table_Cutoffs
        // Check if we have a transposition table entry and if we can return early
        let tt_move =
            match self
                .transposition_table
                .probe::<Node>(0, board.zobrist_hash(), alpha_use, beta)
            {
                ttable::ProbeResult::CutOff(entry) => {
                    // we have a cutoff, so return the score, but only in a non-PV node
                    if !Node::PV {
                        return entry.score;
                    }
                    Some(entry.board_move)
                }
                ttable::ProbeResult::Hit(entry) => Some(entry.board_move),
                ttable::ProbeResult::Empty => None,
            };

        // sort moves by MVV/LVA
        let sorted_moves = captures
            .into_iter()
            .map(|mv| {
                (
                    MoveOrder::classify(board.side_to_move(), &mv, &tt_move, &self.history_table),
                    *mv,
                )
            })
            .collect::<Vec<(MoveOrder, Move)>>();
        let move_iter = IncrementalSort::new(sorted_moves);

        let mut best = standing_eval;
        let mut best_move = tt_move;
        let original_alpha = alpha_use;

        for mv in move_iter.into_iter() {
            board.make_move_unchecked(&mv).unwrap();
            let score = if board.is_draw() {
                Score::DRAW
            } else {
                let eval = -self.quiescence::<Node>(board, -beta, -alpha_use);
                self.nodes += 1;
                eval
            };
            board.unmake_move().unwrap();

            if score > best {
                best = score;
                best_move = Some(mv);

                if score >= beta {
                    break;
                }
                if score > alpha_use {
                    alpha_use = score;
                }
            }

            if self.should_stop_searching() {
                break;
            }
        }

        if best_move.is_some() {
            // store the best move in the transposition table
            let flag = if best <= original_alpha {
                ttable::EntryFlag::UpperBound
            } else if best >= beta {
                ttable::EntryFlag::LowerBound
            } else {
                ttable::EntryFlag::Exact
            };

            self.transposition_table
                .store_entry(TranspositionTableEntry::new(
                    board.zobrist_hash(),
                    0u8,
                    best,
                    flag,
                    best_move.unwrap(),
                ));
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chess::{board::Board, pieces::ALL_PIECES};

    use crate::{
        evaluation::ByteKnightEvaluation,
        score::Score,
        search::{Search, SearchParameters},
        ttable::TranspositionTable,
    };

    use super::LargeScoreType;

    #[test]
    fn white_mate_in_1() {
        let fen = "k7/8/KQ6/8/8/8/8/8 w - - 0 1";
        let board = Board::from_fen(fen).unwrap();
        let config = SearchParameters {
            max_depth: 2,
            ..Default::default()
        };

        let mut ttable = TranspositionTable::default();
        let mut history_table = Default::default();
        let mut search = Search::new(&config, &mut ttable, &mut history_table);
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

        let mut ttable = Default::default();
        let mut history_table = Default::default();
        let mut search = Search::new(&config, &mut ttable, &mut history_table);
        let res = search.search(&mut board, None);

        assert_eq!(res.best_move.unwrap().to_long_algebraic(), "b8a8")
    }

    #[test]
    fn stalemate() {
        let fen = "k7/8/KQ6/8/8/8/8/8 b - - 0 1";
        let mut board = Board::from_fen(fen).unwrap();
        let config = SearchParameters::default();

        let mut ttable = Default::default();
        let mut history_table = Default::default();
        let mut search = Search::new(&config, &mut ttable, &mut history_table);
        let res = search.search(&mut board, None);
        assert!(res.best_move.is_none());
        assert_eq!(res.score, Score::DRAW);
    }

    #[test]
    #[ignore = "Timing on this is not consistent when instrumentation is enabled"]
    fn do_not_exceed_time() {
        let mut board = Board::default_board();
        let config = SearchParameters {
            soft_timeout: Duration::from_millis(100),
            hard_timeout: Duration::from_millis(1000),
            ..Default::default()
        };

        let mut ttable = Default::default();
        let mut history_table = Default::default();
        let mut search = Search::new(&config, &mut ttable, &mut history_table);
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

        let mut ttable = Default::default();
        let mut history_table = Default::default();
        let mut search = Search::new(&config, &mut ttable, &mut history_table);
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

        let mut ttable = Default::default();
        let mut history_table = Default::default();
        let mut search = Search::new(&config, &mut ttable, &mut history_table);
        let res = search.search(&mut board, None);
        assert!(res.best_move.is_some());
        println!("{}", res.best_move.unwrap().to_long_algebraic());
    }

    const TEST_FENS: [&str; 25] = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        "4k3/8/8/8/8/8/8/4K2R w K - 0 1",
        "4k3/8/8/8/8/8/8/R3K3 w Q - 0 1",
        "4k2r/8/8/8/8/8/8/4K3 w k - 0 1",
        "r3k3/8/8/8/8/8/8/4K3 w q - 0 1",
        "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1",
        "r3k2r/8/8/8/8/8/8/4K3 w kq - 0 1",
        "8/8/8/8/8/8/6k1/4K2R w K - 0 1",
        "8/8/8/8/8/8/1k6/R3K3 w Q - 0 1",
        "4k2r/6K1/8/8/8/8/8/8 w k - 0 1",
        "r3k3/1K6/8/8/8/8/8/8 w q - 0 1",
        "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
        "r3k2r/8/8/8/8/8/8/1R2K2R w Kkq - 0 1",
        "r3k2r/8/8/8/8/8/8/2R1K2R w Kkq - 0 1",
        "r3k2r/8/8/8/8/8/8/R3K1R1 w Qkq - 0 1",
        "1r2k2r/8/8/8/8/8/8/R3K2R w KQk - 0 1",
        "2r1k2r/8/8/8/8/8/8/R3K2R w KQk - 0 1",
        "r3k1r1/8/8/8/8/8/8/R3K2R w KQq - 0 1",
        "4k3/8/8/8/8/8/8/4K2R b K - 0 1",
        "4k3/8/8/8/8/8/8/R3K3 b Q - 0 1",
        "4k2r/8/8/8/8/8/8/4K3 b k - 0 1",
        "r3k3/8/8/8/8/8/8/4K3 b q - 0 1",
        "4k3/8/8/8/8/8/8/R3K2R b KQ - 0 1",
        "r3k2r/8/8/8/8/8/8/4K3 b kq - 0 1",
    ];

    #[test]
    fn quiets_ordered_after_captures() {
        let config = SearchParameters {
            max_depth: 6,
            ..Default::default()
        };

        let mut min_mvv_lva = LargeScoreType::MAX;
        let mut max_mvv_lva = LargeScoreType::MIN;
        for capturing in ALL_PIECES {
            for captured in ALL_PIECES.iter().filter(|p| !p.is_king() && !p.is_none()) {
                let mvv_lva = ByteKnightEvaluation::mvv_lva(*captured, capturing);
                if mvv_lva < min_mvv_lva {
                    min_mvv_lva = mvv_lva;
                }
                if mvv_lva > max_mvv_lva {
                    max_mvv_lva = mvv_lva;
                }
            }
        }

        for fen in TEST_FENS {
            let mut board = Board::from_fen(fen).unwrap();

            let mut ttable = Default::default();
            let mut history_table = Default::default();
            let mut search = Search::new(&config, &mut ttable, &mut history_table);
            let res = search.search(&mut board, None);

            assert!(res.best_move.is_some());

            let side = board.side_to_move();
            let mut max_history = LargeScoreType::MIN;
            for piece in ALL_PIECES {
                for square in 0..64 {
                    let score = history_table.get(side, piece, square);
                    if score > max_history {
                        max_history = score;
                    }
                }
            }

            println!("max history: {:5}", max_history);
            println!("min/max mvv-lva: {}, {}", min_mvv_lva, max_mvv_lva);
            assert!(max_history < min_mvv_lva);
        }
    }
}
