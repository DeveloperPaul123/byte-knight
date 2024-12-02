/*
 * tt_table.rs
 * Part of the byte-knight project
 * Created Date: Thursday, November 21st 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Mon Dec 02 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use chess::moves::Move;

use crate::score::Score;

const BYTES_PER_MB: usize = 1024 * 1024;

#[derive(Clone, Copy, PartialEq)]
pub enum EntryFlag {
    Exact,
    LowerBound,
    UpperBound,
}

/// A transposition table entry.
#[derive(Clone, Copy)]
pub(crate) struct TranspositionTableEntry {
    pub zobrist: u64,
    pub score: Score,
    pub board_move: Move,
    pub depth: u8,
    pub flag: EntryFlag,
}

impl TranspositionTableEntry {
    #[allow(dead_code)]
    pub fn new(
        zobrist: u64,
        depth: u8,
        score: Score,
        flag: EntryFlag,
        mv: Move,
    ) -> TranspositionTableEntry {
        TranspositionTableEntry {
            zobrist,
            depth,
            score,
            flag,
            board_move: mv,
        }
    }
}

/// A transposition table used to store the results of previous searches.
pub struct TranspositionTable {
    table: Vec<Option<TranspositionTableEntry>>,
    pub(crate) collisions: usize,
    pub(crate) accesses: usize,
    pub(crate) hits: usize,
}

pub const MAX_TABLE_SIZE_MB: usize = 1024;
pub const MIN_TABLE_SIZE_MB: usize = 16;
const DEFAULT_TABLE_SIZE_MB: usize = MIN_TABLE_SIZE_MB;

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::from_size_in_mb(DEFAULT_TABLE_SIZE_MB)
    }
}

impl TranspositionTable {
    pub(crate) fn from_capacity(capacity: usize) -> Self {
        Self {
            table: vec![None; capacity],
            collisions: 0,
            accesses: 0,
            hits: 0,
        }
    }

    pub(crate) fn from_size_in_mb(mb: usize) -> Self {
        let capacity = mb * BYTES_PER_MB / std::mem::size_of::<TranspositionTableEntry>();
        Self::from_capacity(capacity)
    }

    fn get_index(&self, zobrist: u64) -> usize {
        zobrist as usize % self.table.len()
    }

    pub(crate) fn get_entry(&mut self, zobrist: u64) -> Option<TranspositionTableEntry> {
        let index = self.get_index(zobrist);
        self.table[index]
    }

    pub(crate) fn store_entry(&mut self, entry: TranspositionTableEntry) {
        let index = self.get_index(entry.zobrist);
        self.table[index] = Some(entry);
    }

    pub(crate) fn clear(&mut self) {
        self.table.iter_mut().for_each(|element| {
            *element = None;
        });

        // reset stats as well
        self.collisions = 0;
        self.accesses = 0;
        self.hits = 0;
    }

    pub(crate) fn fullness(&self) -> f64 {
        (self.table.iter().filter(|entry| entry.is_some()).count() as f64 / self.table.len() as f64)
            * 100_f64
    }

    pub(crate) fn size(&self) -> usize {
        self.table.len()
    }
}

#[cfg(test)]
mod tests {
    use chess::{
        moves::{Move, MoveDescriptor},
        pieces::Piece,
        square::Square,
    };

    use crate::score::Score;

    use super::{EntryFlag, TranspositionTable, TranspositionTableEntry};

    #[test]
    fn store_and_retrieve() {
        let mut tt = TranspositionTable::from_capacity(1000);
        let hash1 = 123452341999_u64;
        let hash2 = 2423498723999_u64;
        let mv1 = Move::new(
            &Square::from_square_index(3),
            &Square::from_square_index(4),
            MoveDescriptor::Castle,
            Piece::Knight,
            None,
            None,
        );
        let mv2 = Move::new(
            &Square::from_square_index(7),
            &Square::from_square_index(10),
            MoveDescriptor::Castle,
            Piece::Knight,
            None,
            None,
        );
        tt.store_entry(TranspositionTableEntry::new(
            hash1,
            3,
            Score::new(-123),
            EntryFlag::Exact,
            mv1,
        ));
        tt.store_entry(TranspositionTableEntry::new(
            hash2,
            3,
            Score::new(123),
            EntryFlag::Exact,
            mv2,
        ));

        // TODO(PT) - If we every implement buckets in our ttable, this should be used to check for store/retrieve working correctly at the same index
        // let stored_entry1 = tt.get_entry(hash1);
        // assert!(stored_entry1.is_some());
        // assert_eq!(stored_entry1.unwrap().board_move, mv1);
        let stored_entry2 = tt.get_entry(hash2);
        assert!(stored_entry2.is_some());
        assert_eq!(stored_entry2.unwrap().board_move, mv2);
    }
}
