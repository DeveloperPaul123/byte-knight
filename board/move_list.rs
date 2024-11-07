use arrayvec::ArrayVec;

use crate::{definitions::MAX_MOVE_LIST_SIZE, moves::Move};

pub struct MoveList {
    moves: ArrayVec<Move, MAX_MOVE_LIST_SIZE>,
}

impl MoveList {
    pub fn new() -> Self {
        MoveList {
            moves: ArrayVec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn push(&mut self, mv: Move) {
        let overflow = self.moves.try_push(mv);
        if overflow.is_err() {
            panic!("MoveList is full");
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves.iter()
    }

    pub fn at(&self, index: usize) -> Option<&Move> {
        self.moves.get(index)
    }

    pub fn clear(&mut self) {
        self.moves.clear();
    }
}
