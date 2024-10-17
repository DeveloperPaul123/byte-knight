use crate::{board_state::BoardState, definitions::MAX_MOVES};

pub(crate) struct BoardHistory {
    board_states: Vec<BoardState>,
}

impl Clone for BoardHistory {
    fn clone(&self) -> Self {
        Self {
            board_states: self.board_states.clone(),
        }
    }
}

impl BoardHistory {
    pub fn new() -> Self {
        BoardHistory {
            board_states: Vec::with_capacity(MAX_MOVES),
        }
    }

    pub fn push(&mut self, board_state: BoardState) {
        self.board_states.push(board_state);
    }

    pub fn pop(&mut self) -> Option<BoardState> {
        self.board_states.pop()
    }
}
