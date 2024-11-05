use byte_board::{board::Board, moves::Move};

use super::{search, Timer};

pub struct ByteKnight {
    search: search::Search,
}

impl ByteKnight {
    pub fn new() -> ByteKnight {
        ByteKnight {
            search: search::Search::new(),
        }
    }

    pub(crate) fn think(&mut self, board: &mut Board, _: &Timer) -> Option<Move> {
        let _best_score = self.search.search(board);
        // TODO: Print out information about the search
        Some(self.search.best_move)
    }
}

impl Default for ByteKnight {
    fn default() -> Self {
        ByteKnight::new()
    }
}
