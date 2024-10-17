use byte_board::{board::Board, moves::Move};

use super::{search, ChessEngine, Timer};

struct ByteKnight {
    /// The current board state
    board: Board,
    search: search::Search,
}

impl ByteKnight {
    pub fn new() -> ByteKnight {
        ByteKnight {
            board: Board::default_board(),
            search: search::Search::new(),
        }
    }
}

impl ChessEngine for ByteKnight {
    fn think(&self, board: &mut Board, timer: &Timer) -> Option<Move> {
        // TODO
        return None;
    }
}
