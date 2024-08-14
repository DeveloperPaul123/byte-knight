use chess::Board;
use super::Timer;

pub trait ChessEngine {
    fn think(&self, board: &Board, timer: &Timer) -> Option<chess::ChessMove>;
}
