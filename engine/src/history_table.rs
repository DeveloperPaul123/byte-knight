use chess::{definitions::NumberOf, pieces::Piece, side::Side};

use crate::score::{Score, ScoreType};

pub struct HistoryTable {
    table: [[[Score; NumberOf::SQUARES]; NumberOf::PIECE_TYPES]; NumberOf::SIDES],
}

impl HistoryTable {
    pub(crate) fn new() -> Self {
        let table =
            [[[Default::default(); NumberOf::SQUARES]; NumberOf::PIECE_TYPES]; NumberOf::SIDES];
        Self { table }
    }

    pub(crate) fn get(&self, side: Side, piece: Piece, square: u8) -> Score {
        assert!(side != Side::Both, "Side cannot be Both");
        self.table[side as usize][piece as usize][square as usize]
    }

    pub(crate) fn update(&mut self, side: Side, piece: Piece, square: u8, bonus: ScoreType) {
        assert!(side != Side::Both, "Side cannot be Both");
        let current_score = self.table[side as usize][piece as usize][square as usize];
        let clamped_bonus = (current_score.0 + bonus).clamp(-Score::MAX_HISTORY, Score::MAX_HISTORY);
        // history gravity formula <https://www.chessprogramming.org/History_Heuristic>
        let bonus = clamped_bonus - current_score.0 * clamped_bonus.abs() / Score::MAX_HISTORY;
        self.table[side as usize][piece as usize][square as usize] += bonus;
    }

    pub(crate) fn clear(&mut self) {
        for side in 0..NumberOf::SIDES {
            for piece_type in 0..NumberOf::PIECE_TYPES {
                for square in 0..NumberOf::SQUARES {
                    self.table[side][piece_type][square] = Default::default();
                }
            }
        }
    }
}

impl Default for HistoryTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use chess::{definitions::Squares, pieces::Piece, side::Side};

    use crate::score::Score;

    use super::HistoryTable;

    #[test]
    fn initialize_history_table() {
        let history_table = HistoryTable::new();
        // loop through all sides, piece types, and squares
        for side in 0..2 {
            for piece_type in 0..6 {
                for square in 0..64 {
                    assert_eq!(
                        history_table.table[side][piece_type][square],
                        Default::default()
                    );
                }
            }
        }
    }

    #[test]
    fn store_and_read() {
        let mut history_table = HistoryTable::new();
        let side = Side::Black;
        let piece = Piece::Pawn;
        let square = Squares::A1;
        let score = Score::new(37);
        history_table.update(side, piece, square, score);
        assert_eq!(history_table.get(side, piece, square), score);
        history_table.update(side, piece, square, score);
        assert_eq!(history_table.get(side, piece, square), score + score);
    }
}
