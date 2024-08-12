use chess::Board;

use super::ChessEngine;

struct ByteKnight {
    /// The current board state
    board: Board
}

impl ChessEngine for ByteKnight {
    fn think(&self, board: &Board) -> Option<chess::ChessMove> {
        // TODO
        return None;
    }
}
