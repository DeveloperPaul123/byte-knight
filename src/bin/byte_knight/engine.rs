use chess::{Board, ChessMove};

use super::{search, ChessEngine, Timer};

struct ByteKnight {
    /// The current board state
    board: Board,
    search: search::Search,
}

impl ByteKnight {
    pub fn new() -> ByteKnight {
        ByteKnight {
            board: Board::default(),
            search: search::Search::new(),
        }
    }
}

impl ChessEngine for ByteKnight {
    fn think(&self, board: &Board, timer: &Timer) -> Option<ChessMove> {
        // TODO
        return None;
    }
}
