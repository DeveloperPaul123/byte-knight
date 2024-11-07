use byte_board::{board::Board, moves::Move};

use crate::score::Score;

const BYTES_PER_MB: usize = 1024 * 1024;
const DEFAULT_CAPACITY: usize = 64;

#[derive(Clone, Copy, PartialEq)]
pub enum EntryFlag {
    Exact,
    LowerBound,
    UpperBound,
}

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

pub(crate) struct TranspositionTable {
    table: Vec<Option<TranspositionTableEntry>>,
}

impl TranspositionTable {
    pub(crate) fn new() -> TranspositionTable {
        Self::from_capacity(DEFAULT_CAPACITY)
    }

    pub(crate) fn from_capacity(capacity: usize) -> Self {
        Self {
            table: vec![None; capacity],
        }
    }

    pub(crate) fn from_size_in_mb(mb: usize) -> Self {
        let capacity = mb * BYTES_PER_MB / std::mem::size_of::<TranspositionTableEntry>();
        Self::from_capacity(capacity)
    }

    fn get_index(self: &Self, zobrist: u64) -> usize {
        zobrist as usize % self.table.len()
    }

    pub(crate) fn get_entry(self: &Self, zobrist: u64) -> Option<TranspositionTableEntry> {
        let index = self.get_index(zobrist);
        self.table[index]
    }

    pub(crate) fn store_entry(self: &mut Self, entry: TranspositionTableEntry) {
        let index = self.get_index(entry.zobrist);
        self.table[index] = Some(entry);
    }
}
