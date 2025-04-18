use chess::moves::Move;

use crate::defs::MAX_DEPTH;

pub struct KillerMovesTable {
    table: [Option<Move>; MAX_DEPTH as usize],
}

impl KillerMovesTable {
    pub(crate) fn new() -> Self {
        let table = [None; MAX_DEPTH as usize];
        Self { table }
    }

    pub(crate) fn get(&self, depth: u8) -> Option<Move> {
        assert!(depth < MAX_DEPTH, "Depth is out of bounds");
        self.table[depth as usize]
    }

    pub(crate) fn update(&mut self, ply: u8, mv: Move) {
        assert!(ply < MAX_DEPTH, "Depth is out of bounds");
        self.table[ply as usize] = Some(mv);
    }

    pub(crate) fn clear(&mut self) {
        for depth in 0..MAX_DEPTH {
            self.table[depth as usize] = None;
        }
    }
}

impl Default for KillerMovesTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::KillerMovesTable;

    #[test]
    fn initialize_killers_table() {
        let killers_table: KillerMovesTable = Default::default();
        assert_eq!(killers_table.get(0), None);
    }
}
