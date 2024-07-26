use chess::Board;

pub trait ChessEngine {
    fn think(&self, board: &Board) -> Option<chess::ChessMove>;
}
