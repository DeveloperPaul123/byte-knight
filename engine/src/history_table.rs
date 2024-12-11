use chess::{
    definitions::NumberOf,
    pieces::{Piece, PIECE_NAMES},
    side::Side,
};

use crate::score::MoveOrderScoreType;

pub struct HistoryTable {
    table: [[[MoveOrderScoreType; NumberOf::SQUARES]; NumberOf::PIECE_TYPES]; NumberOf::SIDES],
}

impl HistoryTable {
    pub(crate) fn new() -> Self {
        let table =
            [[[Default::default(); NumberOf::SQUARES]; NumberOf::PIECE_TYPES]; NumberOf::SIDES];
        Self { table }
    }

    pub(crate) fn get(&self, side: Side, piece: Piece, square: u8) -> MoveOrderScoreType {
        assert!(side != Side::Both, "Side cannot be Both");
        self.table[side as usize][piece as usize][square as usize]
    }

    pub(crate) fn update(
        &mut self,
        side: Side,
        piece: Piece,
        square: u8,
        bonus: MoveOrderScoreType,
    ) {
        assert!(side != Side::Both, "Side cannot be Both");
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

    pub(crate) fn print_for_side(&self, side: Side) {
        for (piece_type, piece_name) in PIECE_NAMES.iter().enumerate() {
            println!("{} - {}", piece_name, side);
            // print from white's perspective
            for rank in (0..=NumberOf::RANKS - 1).rev() {
                print!("|");
                for file in 0..NumberOf::FILES {
                    let square = file + rank * NumberOf::FILES;
                    print!("{:5} ", self.table[side as usize][piece_type][square]);
                }
                println!("|");
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
    use super::HistoryTable;
    use chess::{definitions::Squares, pieces::Piece, side::Side};

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
        let score = 37;
        history_table.update(side, piece, square, score);
        assert_eq!(history_table.get(side, piece, square), score);
        history_table.update(side, piece, square, score);
        assert_eq!(history_table.get(side, piece, square), score + score);
    }
}
