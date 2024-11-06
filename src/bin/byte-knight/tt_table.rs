use byte_board::moves::Move;

#[derive(Clone, Copy)]
pub(crate) struct TranspositionTableEntry {
    zobrist: u64,
    depth: i64,
    score: i64,
    flag: i64,
    board_move: Move,
}

impl TranspositionTableEntry {
    pub fn new() -> TranspositionTableEntry {
        TranspositionTableEntry {
            zobrist: 0,
            depth: 0,
            score: 0,
            flag: 0,
            board_move: Move::default(),
        }
    }
}

static TRANSPOSITION_TABLE_SIZE: usize = 1_048_576;
pub(crate) struct TranspositionTable {
    table: Vec<Option<TranspositionTableEntry>>,
}

impl TranspositionTable {
    pub(crate) fn new() -> TranspositionTable {
        TranspositionTable {
            table: Vec::with_capacity(TRANSPOSITION_TABLE_SIZE),
        }
    }

    pub(crate) fn get_entry(self: &Self, zobrist: u64) -> Option<TranspositionTableEntry> {
        let index = zobrist as usize % TRANSPOSITION_TABLE_SIZE;
        self.table[index]
    }
}
