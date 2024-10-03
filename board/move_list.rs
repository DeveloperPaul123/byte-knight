use crate::{
    definitions::{MAX_MOVES, MAX_MOVE_LIST_SIZE},
    moves::Move,
};

pub struct MoveList {
    size: usize,
    moves: [Move; MAX_MOVE_LIST_SIZE],
}

impl MoveList {
    pub fn new() -> Self {
        MoveList {
            size: 0,
            moves: [Move::default(); MAX_MOVE_LIST_SIZE],
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, mv: Move) {
        if self.size >= MAX_MOVES {
            panic!("MoveList is full");
        }
        self.moves[self.size] = mv;
        self.size += 1;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves.iter().take(self.size)
    }

    pub fn clear(&mut self) {
        self.size = 0;
    }
}
