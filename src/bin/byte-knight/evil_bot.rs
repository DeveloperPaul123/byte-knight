use byte_board::{
    board::Board,
    move_generation::MoveGenerator,
    move_list::MoveList,
    moves::{Move, MoveType},
    pieces::Piece,
};

use crate::ChessEngine;

use super::Timer;

pub struct EvilBot {
    move_gen: MoveGenerator,
}

impl EvilBot {
    fn piece_value(piece: Piece) -> u8 {
        match piece {
            Piece::Pawn => 1,
            Piece::Knight => 3,
            Piece::Bishop => 3,
            Piece::Rook => 5,
            Piece::Queen => 9,
            Piece::King => 0, // We don't want to prioritize capturing the king
            Piece::None => 0,
        }
    }
}

impl Default for EvilBot {
    fn default() -> Self {
        EvilBot {
            move_gen: MoveGenerator::new(),
        }
    }
}

impl ChessEngine for EvilBot {
    fn think(self: &mut Self, board: &mut Board, _: &Timer) -> Option<Move> {
        let mut move_list = MoveList::new();
        self.move_gen
            .generate_moves(board, &mut move_list, MoveType::All);
        let result = move_list
            .iter()
            .filter(|&m| board.is_legal(m, &self.move_gen))
            .max_by(|&a, &b| {
                let a_value = EvilBot::piece_value(a.captured_piece().unwrap_or(Piece::None));
                let b_value = EvilBot::piece_value(b.captured_piece().unwrap_or(Piece::None));
                a_value.cmp(&b_value)
            });

        match result {
            Some(mv) => Some(mv.clone()),
            None => None,
        }
    }
}
