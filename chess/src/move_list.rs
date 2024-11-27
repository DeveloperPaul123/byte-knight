/*
 * move_list.rs
 * Part of the byte-knight project
 * Created Date: Monday, November 25th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Tue Nov 26 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use arrayvec::ArrayVec;

use crate::{definitions::MAX_MOVE_LIST_SIZE, moves::Move};

/// A list of moves used in move generation. This is a fixed-size list that can hold up to 218 moves.
/// If more moves are added, the program will panic.
pub struct MoveList {
    moves: ArrayVec<Move, MAX_MOVE_LIST_SIZE>,
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveList {
    /// Create a new [MoveList] with a capacity of 218 moves.
    /// Note that no default assignment is done for the moves in the list.
    /// The intention is for the items in the [`MoveList`] to be overwritten.
    pub fn new() -> Self {
        MoveList {
            moves: ArrayVec::new(),
        }
    }

    /// Returns the number of moves in the list.
    pub fn len(&self) -> usize {
        self.moves.len()
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    /// Push a move to the list. If the list is full, the program will panic.
    /// This is done to avoid the overhead of returning a Result.
    pub fn push(&mut self, mv: Move) {
        let overflow = self.moves.try_push(mv);
        if overflow.is_err() {
            panic!("MoveList is full");
        }
    }

    /// Get an iterator to the moves in the list.
    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves.iter()
    }

    /// Get the move at the given index. Returns None if the index is out of bounds.
    pub fn at(&self, index: usize) -> Option<&Move> {
        self.moves.get(index)
    }

    /// Clear the list of moves.
    pub fn clear(&mut self) {
        self.moves.clear();
    }
}
