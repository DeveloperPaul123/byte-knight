use super::Timer;
use chess::Board;

pub trait ChessEngine {
    fn think(&self, board: &Board, timer: &Timer) -> Option<chess::ChessMove>;
}
