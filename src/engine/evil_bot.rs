use chess::{Board, ChessMove, MoveGen, Piece};

use crate::engine::ChessEngine;
use super::Timer;

pub struct EvilBot;

impl EvilBot {
    fn piece_value(piece: Piece) -> u8 {
        match piece {
            Piece::Pawn => 1,
            Piece::Knight => 3,
            Piece::Bishop => 3,
            Piece::Rook => 5,
            Piece::Queen => 9,
            Piece::King => 0, // We don't want to prioritize capturing the king
        }
    }
}

impl ChessEngine for EvilBot
{
    fn think(self: &Self, board: &Board, _: &Timer) -> Option<ChessMove> {
        MoveGen::new_legal(board)
            .filter(|&m| board.piece_on(m.get_dest()).is_some())
            .max_by(|&a, &b| {
                let a_value = EvilBot::piece_value(board.piece_on(a.get_dest()).unwrap());
                let b_value = EvilBot::piece_value(board.piece_on(b.get_dest()).unwrap());
                a_value.cmp(&b_value)
            })
    }
}
