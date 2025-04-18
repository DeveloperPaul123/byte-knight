use chess::moves::Move;

use crate::defs::{MAX_DEPTH, MAX_KILLERS_PER_PLY};

pub struct KillerMovesTable {
    table: [[Option<Move>; MAX_KILLERS_PER_PLY as usize]; MAX_DEPTH as usize],
}

impl KillerMovesTable {
    pub(crate) fn new() -> Self {
        let table = [[None; MAX_KILLERS_PER_PLY as usize]; MAX_DEPTH as usize];
        Self { table }
    }

    pub(crate) fn get(&self, depth: u8) -> &[Option<Move>] {
        assert!(depth < MAX_DEPTH, "Depth is out of bounds");
        &self.table[depth as usize][..]
    }

    fn get_mut(&mut self, depth: u8) -> &mut [Option<Move>] {
        assert!(depth < MAX_DEPTH, "Depth is out of bounds");
        &mut self.table[depth as usize][..]
    }

    pub(crate) fn update(&mut self, ply: u8, mv: Move) {
        assert!(ply < MAX_DEPTH, "Depth is out of bounds");
        let current_killers = &mut self.get_mut(ply);
        // first killer should not match the new move
        if !current_killers[0].is_some_and(|killer_mv| killer_mv == mv) {
            current_killers.swap(0, 1);
            self.table[ply as usize][0] = Some(mv);
        }
    }

    pub(crate) fn clear(&mut self) {
        for item in self.table.as_flattened_mut() {
            *item = None;
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
    use crate::defs::MAX_DEPTH;

    use super::KillerMovesTable;

    #[test]
    fn initialize_killers_table() {
        let killers_table: KillerMovesTable = Default::default();
        for i in 0..MAX_DEPTH {
            let killers = killers_table.get(i);
            assert_eq!(killers.len(), 2);
            assert!(killers.iter().all(|mv| *mv == None));
        }
    }
}
