use crate::{
    board::Board,
    move_generation::MoveGenerator,
    move_list::MoveList,
    moves::{Move, MoveType},
};
use anyhow::{bail, Result};

pub struct SplitPerftResult {
    pub mv: Move,
    pub nodes: u64,
}

#[cfg_attr(not(debug_assertions), inline(always))]
#[cfg_attr(debug_assertions, inline(never))]
pub fn split_perft(
    board: &mut Board,
    move_gen: &MoveGenerator,
    depth: usize,
    print_moves: bool,
) -> Result<Vec<SplitPerftResult>> {
    let mut move_list = MoveList::new();
    move_gen.generate_legal_moves(&board, &mut move_list);

    let mut results = Vec::new();
    for mv in move_list.iter() {
        // first ensure the move is legal
        if print_moves {
            println!("mv - {}", mv.to_long_algebraic());
        }
        let move_res = board.make_move(mv, &move_gen);
        if move_res.is_err() {
            bail!("move failed: {:?}", move_res);
        }
        if print_moves {
            println!("node - {}", mv.to_long_algebraic());
        }

        let nodes = if depth > 1 {
            perft(board, move_gen, depth - 1, print_moves)?
        } else {
            1
        };
        if print_moves {
            println!("---");
        }
        board.unmake_move()?;
        results.push(SplitPerftResult { mv: *mv, nodes });
    }

    // sort results alphabetically
    results.sort_by(|a, b| a.mv.to_long_algebraic().cmp(&b.mv.to_long_algebraic()));

    Ok(results)
}

#[cfg_attr(not(debug_assertions), inline(always))]
#[cfg_attr(debug_assertions, inline(never))]
pub fn perft(
    board: &mut Board,
    move_gen: &MoveGenerator,
    depth: usize,
    print_moves: bool,
) -> Result<u64> {
    let mut nodes = 0;
    let mut move_list = MoveList::new();
    move_gen.generate_legal_moves(&board, &mut move_list);

    if print_moves {
        for mv in move_list.iter() {
            println!("{}", mv);
        }
    }

    if depth == 1 {
        // bulk counting
        return Ok(move_list.len() as u64);
    }

    for mv in move_list.iter() {
        let result = board.make_move(mv, &move_gen);
        // this is unexpected as we are generating legal moves
        // if this happens, it is likely a bug in the move generator
        if result.is_err() {
            println!("current board: {}", board.to_fen());
            println!("current move: {}", mv);
            bail!("move failed ({}): {:?}", depth, result);
        }
        nodes += perft(board, move_gen, depth - 1, print_moves)?;
        board.unmake_move()?;
    }

    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, move_generation::MoveGenerator, perft::perft, side::Side};
    #[test]
    fn default_board() {
        let mut board = Board::default_board();
        let move_gen = MoveGenerator::new();
        assert!(board.en_passant_square().is_none());
        let result = perft(&mut board, &move_gen, 1, false).unwrap();
        assert_eq!(result, 20);
    }

    #[test]
    fn single_depth_non_standard_positions() {
        // test positions taken from https://gist.github.com/peterellisjones/8c46c28141c162d1d8a0f0badbc9cff9
        let move_gen = MoveGenerator::new();
        {
            let mut board = Board::from_fen("r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2").unwrap();
            assert_eq!(board.side_to_move(), Side::Black);
            assert!(board.is_in_check(&move_gen));
            let total_moves = perft(&mut board, &move_gen, 1, false).unwrap();
            assert_eq!(total_moves, 8);
        }

        {
            let mut board = Board::from_fen("8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0 3").unwrap();
            assert_eq!(board.side_to_move(), Side::Black);
            assert!(board.is_in_check(&move_gen));
            assert!(board.en_passant_square().is_some());
            let total_moves = perft(&mut board, &move_gen, 1, false).unwrap();
            assert_eq!(total_moves, 8);
        }

        {
            let mut board =
                Board::from_fen("r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 2 2")
                    .unwrap();
            let total_moves = perft(&mut board, &move_gen, 1, false).unwrap();
            assert_eq!(total_moves, 19);
        }

        {
            let mut board = Board::from_fen(
                "r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQkq - 3 2",
            )
            .unwrap();
            let total_moves = perft(&mut board, &move_gen, 1, false).unwrap();
            assert_eq!(total_moves, 5);
        }

        {
            let mut board =
                Board::from_fen("2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2")
                    .unwrap();
            let total_moves = perft(&mut board, &move_gen, 1, false).unwrap();
            assert_eq!(total_moves, 44);
        }

        {
            let mut board =
                Board::from_fen("rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w KQ - 3 9")
                    .unwrap();
            let total_moves = perft(&mut board, &move_gen, 1, false).unwrap();
            assert_eq!(total_moves, 39);
        }

        {
            let mut board = Board::from_fen("2r5/3pk3/8/2P5/8/2K5/8/8 w - - 5 4").unwrap();
            let total_moves = perft(&mut board, &move_gen, 1, false).unwrap();
            assert_eq!(total_moves, 9);
        }

        {
            let mut board =
                Board::from_fen("rnQq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQKR1r b Q - 1 8")
                    .unwrap();
            let total_moves = perft(&mut board, &move_gen, 1, false).unwrap();
            assert_eq!(total_moves, 32);
        }
    }

    #[test]
    fn multi_depth_non_standard_positions_2() {
        let move_gen = MoveGenerator::new();
        let mut board = Board::from_fen("3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1").unwrap();
        let total_moves = perft(&mut board, &move_gen, 6, false).unwrap();
        assert_eq!(total_moves, 1134888);
    }

    #[test]
    fn multi_depth_non_standard_positions() {
        let move_gen = MoveGenerator::new();
        {
            let mut board =
                Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")
                    .unwrap();
            let total_moves_depth1 = perft(&mut board, &move_gen, 1, false).unwrap();
            assert_eq!(total_moves_depth1, 44);
            let total_moves_depth2 = perft(&mut board, &move_gen, 2, false).unwrap();
            assert_eq!(total_moves_depth2, 1486);
            let total_moves = perft(&mut board, &move_gen, 3, false).unwrap();
            // TODO: This is VERY broken
            assert_eq!(total_moves, 62379);
        }

        {
            let mut board = Board::from_fen(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            )
            .unwrap();
            let total_moves = perft(&mut board, &move_gen, 3, false).unwrap();
            assert_eq!(total_moves, 89890);
        }

        {
            let mut board = Board::from_fen("8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1").unwrap();
            let total_moves = perft(&mut board, &move_gen, 6, false).unwrap();
            assert_eq!(total_moves, 1015133);
        }

        {
            let mut board = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1").unwrap();
            let total_moves = perft(&mut board, &move_gen, 6, false).unwrap();
            assert_eq!(total_moves, 1440467);
        }

        {
            let mut board = Board::from_fen("5k2/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();
            let total_moves = perft(&mut board, &move_gen, 6, false).unwrap();
            assert_eq!(total_moves, 661072);
        }

        {
            let mut board = Board::from_fen("3k4/8/8/8/8/8/8/R3K3 w Q - 0 1").unwrap();
            let total_moves = perft(&mut board, &move_gen, 6, false).unwrap();
            assert_eq!(total_moves, 803711);
        }

        {
            let mut board = Board::from_fen("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1").unwrap();
            let total_moves = perft(&mut board, &move_gen, 4, false).unwrap();
            assert_eq!(total_moves, 1274206);
        }

        {
            let mut board = Board::from_fen("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1").unwrap();
            let total_moves = perft(&mut board, &move_gen, 4, false).unwrap();
            assert_eq!(total_moves, 1720476);
        }

        {
            let mut board = Board::from_fen("2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1").unwrap();
            let total_moves = perft(&mut board, &move_gen, 6, false).unwrap();
            assert_eq!(total_moves, 3821001);
        }

        {
            let mut board = Board::from_fen("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1").unwrap();
            let total_moves = perft(&mut board, &move_gen, 5, false).unwrap();
            assert_eq!(total_moves, 1004658);
        }

        {
            let mut board = Board::from_fen("4k3/1P6/8/8/8/8/K7/8 w - - 0 1").unwrap();
            let total_moves = perft(&mut board, &move_gen, 6, false).unwrap();
            assert_eq!(total_moves, 217342);
        }

        {
            let mut board = Board::from_fen("8/P1k5/K7/8/8/8/8/8 w - - 0 1").unwrap();
            let total_moves = perft(&mut board, &move_gen, 6, false).unwrap();
            assert_eq!(total_moves, 92683);
        }

        {
            let mut board = Board::from_fen("K1k5/8/P7/8/8/8/8/8 w - - 0 1").unwrap();
            let total_moves = perft(&mut board, &move_gen, 6, false).unwrap();
            assert_eq!(total_moves, 2217);
        }

        {
            let mut board = Board::from_fen("8/k1P5/8/1K6/8/8/8/8 w - - 0 1").unwrap();
            let total_moves = perft(&mut board, &move_gen, 7, false).unwrap();
            assert_eq!(total_moves, 567584);
        }

        {
            let mut board = Board::from_fen("8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1").unwrap();
            let total_moves = perft(&mut board, &move_gen, 4, false).unwrap();
            assert_eq!(total_moves, 23527);
        }
    }
}
