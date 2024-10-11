use crate::{board::Board, move_generation::MoveGenerator, move_list::MoveList, moves::MoveType};
use anyhow::Result;

#[cfg_attr(not(debug_assertions), inline(always))]
#[cfg_attr(debug_assertions, inline(never))]
pub fn perft(
    board: &mut Board,
    move_gen: &MoveGenerator,
    depth: usize,
    print_moves: bool,
) -> Result<u64> {
    if depth == 0 {
        return Ok(1);
    }

    let mut nodes = 0;
    let mut move_list = MoveList::new();
    move_gen.generate_moves(&board, &mut move_list, MoveType::All);
    println!("generated {} moves", move_list.len());
    for mv in move_list.iter() {
        if !board.make_move(mv, &move_gen).is_err() {
            if print_moves {
                println!("{}", mv);
            }
            nodes += perft(board, move_gen, depth - 1, print_moves)?;
            board.unmake_move()?;
        }
    }

    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use crate::{
        board::Board, definitions::Side, move_generation::MoveGenerator, moves::Move, perft::perft,
    };
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
    }

    #[test]
    fn multi_depth_non_standard_positions() {
        let move_gen = MoveGenerator::new();
        {
            let mut board =
                Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")
                    .unwrap();
            let total_moves = perft(&mut board, &move_gen, 3, false).unwrap();
            // TODO: This is VERY broken
            assert_eq!(total_moves, 62379);
        }
    }
}
