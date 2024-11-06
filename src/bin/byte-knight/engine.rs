use byte_board::{board::Board, moves::Move};

use crate::search::SearchParameters;

use super::search;

pub struct ByteKnight {
    search: search::Search,
}

impl ByteKnight {
    pub fn new() -> ByteKnight {
        ByteKnight {
            search: search::Search::new(),
        }
    }

    pub(crate) fn think(
        &mut self,
        board: &mut Board,
        search_params: &SearchParameters,
    ) -> Option<Move> {
        let result = self.search.search(board, &search_params);
        // TODO: Print out information about the search
        result.best_move
    }
}

impl Default for ByteKnight {
    fn default() -> Self {
        ByteKnight::new()
    }
}
