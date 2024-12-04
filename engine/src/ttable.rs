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

use crate::score::Score;
use chess::moves::Move;

const BYTES_PER_MB: usize = 1024 * 1024;
/// How many positions do we store per index in the TT
const POSITIONS_PER_INDEX: usize = 2;

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

/// Given "word", produce an integer in the range [0, p) without division.
/// Alternative to modulo operation.
/// See <https://github.com/ozgrakkurt/fastrange-rs/blob/master/src/lib.rs>
const fn fast_range_64(word: u64, p: u64) -> u64 {
    ((word as u128 * p as u128) >> 64) as u64
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
        fast_range_64(zobrist, self.table.len() as u64) as usize
    }

    pub(crate) fn get_entry(&mut self, zobrist: u64) -> Option<TranspositionTableEntry> {
        let index = self.get_index(zobrist);
        let max_index = index + POSITIONS_PER_INDEX;

        let slice = if max_index > self.table.len() {
            // wrap around to beginning
            &self.table[index..]
                .iter()
                .chain(&self.table[..max_index % self.table.len()])
                .collect::<Vec<_>>()
        } else {
            &self.table[index..max_index].iter().collect()
        };

        // find the first entry that matches the zobrist key
        slice.iter().find_map(|entry| {
            if let Some(entry) = entry {
                if entry.zobrist == zobrist {
                    Some(*entry)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    /// Store an entry into the hash table.
    pub(crate) fn store_entry(&mut self, entry: TranspositionTableEntry) {
        let index = self.get_index(entry.zobrist);
        let old = self.table[index];
        if let Some(old) = old {
            if old.zobrist != entry.zobrist {
                // index is already occupied by a different position whose index collides with the current position
                // find the next index that we can use
                let mut stored = false;
                let mut delta = 1;
                while !stored && delta < POSITIONS_PER_INDEX {
                    let idx = (index + delta) % self.table.len();
                    let stored_value = self.table[idx];

                    match stored_value {
                        Some(e) => {
                            if e.zobrist == entry.zobrist {
                                self.table[idx] = Some(entry);
                                stored = true;
                                break;
                            }
                        }
                        None => {
                            self.table[idx] = Some(entry);
                            stored = true;
                            break;
                        }
                    }

                    delta += 1;
                }

                if !stored {
                    // What do we do here? All positions are full, and we don't have a matching position
                    // For now when in doubt, just overwrite
                    self.table[index] = Some(entry);
                }
            }
        } else {
            // the index is not occupied
            self.table[index] = Some(entry);
        }
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
    use super::{EntryFlag, TranspositionTable, TranspositionTableEntry};
    use crate::score::Score;
    use chess::{
        moves::{Move, MoveDescriptor},
        pieces::Piece,
        square::Square,
    };
    use itertools::Itertools;
    use rand::Rng;
    use std::collections::HashMap;

    #[test]
    fn get_index() {
        let tt = TranspositionTable::from_size_in_mb(32);
        let mut rng = rand::thread_rng();
        let random_numbers: Vec<u64> = (0..tt.size()).map(|_| rng.gen::<u64>()).collect();
        let min = random_numbers.iter().min().unwrap();
        let max = random_numbers.iter().max().unwrap();
        println!("min/max random number: {}/{}", min, max);
        println!("Table size: {}", tt.size());
        let mut index_histogram: HashMap<usize, usize> = HashMap::new();
        random_numbers.iter().for_each(|&num| {
            let index = tt.get_index(num);
            assert!(index < tt.size());
            *index_histogram.entry(index).or_insert(0) += 1;
        });

        // make sure that the distribution is roughly uniform
        let min = index_histogram.values().min().unwrap();
        let max = index_histogram.values().max().unwrap();
        let mean = index_histogram.values().sum::<usize>() as f64 / index_histogram.len() as f64;
        let count = index_histogram.len();

        println!("Min: {}, Max: {}, Mean: {}, Len: {}", min, max, mean, count);
        let unique_keys = random_numbers.iter().unique().count();
        println!("Unique keys: {}", unique_keys);
        let collision_rate = (1.0 - (count as f64 / unique_keys as f64)) * 100.0;
        println!("Collision rate: {}", collision_rate);
    }

    #[test]
    fn store_and_retrieve() {
        let mut tt = TranspositionTable::from_capacity(1000);
        // these 2 hashes will collide at index 999
        let hash1 = 1234523413999_u64;
        let hash2 = 2423498723999_u64;
        let hash3 = 1466267341999_u64;
        let mv1 = Move::new(
            &Square::from_square_index(3),
            &Square::from_square_index(4),
            MoveDescriptor::None,
            Piece::Knight,
            None,
            None,
        );
        let mv2 = Move::new(
            &Square::from_square_index(7),
            &Square::from_square_index(10),
            MoveDescriptor::None,
            Piece::Knight,
            None,
            None,
        );

        let mv3 = Move::new(
            &Square::from_square_index(4),
            &Square::from_square_index(10),
            MoveDescriptor::None,
            Piece::Bishop,
            Some(Piece::Pawn),
            None,
        );

        tt.store_entry(TranspositionTableEntry::new(
            hash1,
            3,
            Score::new(-123),
            EntryFlag::Exact,
            mv1,
        ));

        let stored_entry1 = tt.get_entry(hash1);
        assert!(stored_entry1.is_some());
        assert_eq!(stored_entry1.unwrap().board_move, mv1);

        tt.store_entry(TranspositionTableEntry::new(
            hash2,
            3,
            Score::new(123),
            EntryFlag::Exact,
            mv2,
        ));

        let stored_entry1 = tt.get_entry(hash1);
        assert!(stored_entry1.is_some());
        assert_eq!(stored_entry1.unwrap().board_move, mv1);
        let mut stored_entry2 = tt.get_entry(hash2);
        assert!(stored_entry2.is_some());
        assert_eq!(stored_entry2.unwrap().board_move, mv2);

        tt.store_entry(TranspositionTableEntry::new(
            hash3,
            3,
            Score::new(123),
            EntryFlag::Exact,
            mv3,
        ));

        // this should have overridden entry 1
        let stored_entry3 = tt.get_entry(hash3);
        assert!(stored_entry3.is_some());
        assert_eq!(stored_entry3.unwrap().board_move, mv3);

        stored_entry2 = tt.get_entry(hash2);
        assert!(stored_entry2.is_some());
        assert_eq!(stored_entry2.unwrap().board_move, mv2);

        tt.store_entry(TranspositionTableEntry::new(
            hash3,
            3,
            Score::new(123),
            EntryFlag::Exact,
            mv3,
        ));

        let stored_entry3 = tt.get_entry(hash3);
        assert!(stored_entry3.is_some());
        assert_eq!(stored_entry3.unwrap().board_move, mv3);
    }
}
