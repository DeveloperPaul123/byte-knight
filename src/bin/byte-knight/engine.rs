use byte_board::{board::Board, moves::Move};

use crate::search::SearchParameters;

use super::search;

pub struct ByteKnight {}

impl ByteKnight {
    pub fn new() -> ByteKnight {
        ByteKnight {}
    }

    pub(crate) fn think(
        &mut self,
        board: &mut Board,
        search_params: &SearchParameters,
    ) -> Option<Move> {
        println!("Searching with params: {}", search_params);
        let mut search = search::Search::new(*search_params);
        let result = search.search(board);
        result.best_move
    }
}

impl Default for ByteKnight {
    fn default() -> Self {
        ByteKnight::new()
    }
}
