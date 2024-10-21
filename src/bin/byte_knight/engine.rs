use byte_board::{board::Board, moves::Move};

use super::{search, ChessEngine, Timer};

pub struct ByteKnight {
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

impl Default for ByteKnight {
    fn default() -> Self {
        ByteKnight::new()
    }
}

impl ChessEngine for ByteKnight {
    fn think(&mut self, board: &mut Board, timer: &Timer) -> Option<Move> {
        let _best_score = self.search.search(board);
        // TODO: Print out information about the search
        Some(self.search.best_move)
    }
}
