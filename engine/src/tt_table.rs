/*
 * tt_table.rs
 * Part of the byte-knight project
 * Created Date: Thursday, November 21st 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Fri Nov 29 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use chess::{board::Board, moves::Move};

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
    pub depth: u8,
    pub score: Score,
    pub flag: EntryFlag,
    pub board_move: Move,
}

impl TranspositionTableEntry {
    pub fn new(
        board: &Board,
        depth: u8,
        score: Score,
        flag: EntryFlag,
        mv: Move,
    ) -> TranspositionTableEntry {
        TranspositionTableEntry {
            zobrist: board.zobrist_hash(),
            depth,
            score,
            flag,
            board_move: mv,
        }
    }
}

/// A transposition table used to store the results of previous searches.
pub struct TranspositionTable<const TRACK_STATS: bool> {
    table: Vec<Option<TranspositionTableEntry>>,
    pub(crate) collisions: usize,
    pub(crate) accesses: usize,
    pub(crate) hits: usize,
}

pub const MAX_TABLE_SIZE_MB: usize = 1024;
pub const MIN_TABLE_SIZE_MB: usize = 16;
const DEFAULT_TABLE_SIZE_MB: usize = MIN_TABLE_SIZE_MB;

impl<const TRACK_STATS: bool> Default for TranspositionTable<TRACK_STATS> {
    fn default() -> Self {
        Self::from_size_in_mb(DEFAULT_TABLE_SIZE_MB)
    }
}

impl<const TRACK_STATS: bool> TranspositionTable<TRACK_STATS> {
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
        if TRACK_STATS {
            self.accesses += 1;
        }

        let index = self.get_index(zobrist);
        let entry = self.table[index];

        if TRACK_STATS {
            if entry.is_some() {
                self.hits += 1;
            }
        }
        entry
    }

    pub(crate) fn store_entry(&mut self, entry: TranspositionTableEntry) {
        let index = self.get_index(entry.zobrist);
        if TRACK_STATS {
            let old = self.table[index];
            if let Some(old) = old {
                if old.zobrist == entry.zobrist {
                    self.collisions += 1;
                }
            }
        }
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
