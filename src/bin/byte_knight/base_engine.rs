use byte_board::{board::Board, moves::Move};

use super::Timer;

pub trait ChessEngine {
    fn think(&mut self, board: &mut Board, timer: &Timer) -> Option<Move>;
}
