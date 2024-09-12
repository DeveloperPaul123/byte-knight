use crate::{
    definitions::{MAX_MOVES, MAX_MOVE_LIST_SIZE},
    moves::Move,
};

pub struct MoveList {
    index: usize,
    moves: [Move; MAX_MOVE_LIST_SIZE],
}

impl MoveList {
    pub fn new() -> Self {
        MoveList {
            index: 0,
            moves: [Move::default(); MAX_MOVE_LIST_SIZE],
        }
    }

    pub fn push(&mut self, mv: Move) {
        if self.index >= MAX_MOVES {
            panic!("MoveList is full");
        }
        self.moves[self.index] = mv;
        self.index += 1;
    }

    pub fn pop(&mut self) -> Option<Move> {
        if self.index == 0 {
            return None;
        }
        self.index -= 1;
        return Some(self.moves[self.index]);
    }
}
