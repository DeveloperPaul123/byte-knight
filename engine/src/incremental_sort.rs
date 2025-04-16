use chess::moves::Move;

use crate::move_order::MoveOrder;

struct OrderedMove {
    pub(crate) order: MoveOrder,
    pub(crate) mv: Move,
}

impl PartialEq for OrderedMove {
    fn eq(&self, other: &Self) -> bool {
        (self.order == other.order)
            .then(|| {
                // If orders are equal, compare moves
                self.mv == other.mv
            })
            .unwrap()
    }
}

impl Eq for OrderedMove {}

impl PartialOrd for OrderedMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

pub(crate) struct IncrementalSort {
    data: Vec<OrderedMove>,
    current_index: usize,
}

impl IncrementalSort {
    pub(crate) fn new(move_list: Vec<(MoveOrder, Move)>) -> Self {
        let data = move_list
            .into_iter()
            .map(|(order, mv)| OrderedMove { order, mv })
            .collect();
        Self {
            data,
            current_index: 0,
        }
    }

    pub(crate) fn has_next(&self) -> bool {
        self.current_index < self.data.len()
    }

    pub(crate) fn next(&mut self) -> Option<Move> {
        if !self.has_next() {
            return None;
        }

        // Find the index of the maximum score among remaining elements
        let mut max_idx = self.current_index;
        for i in self.current_index..self.data.len() {
            // This comparison ensures we get the HIGHEST scores first
            // MoveOrdering sorts in descending order
            if self.data[i] < self.data[max_idx] {
                max_idx = i;
            }
        }

        // Swap the max element with the current position
        if max_idx != self.current_index {
            self.data.swap(self.current_index, max_idx);
        }

        // Move the index forward and return the current max item
        let current = self.current_index;
        self.current_index += 1;

        // Take the item at the current index
        Some(std::mem::replace(&mut self.data[current].mv, unsafe {
            std::mem::zeroed()
        }))
    }
}

impl Iterator for IncrementalSort {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[cfg(test)]
mod tests {
    use chess::{
        board::Board,
        move_generation::MoveGenerator,
        move_list::MoveList,
        moves::{Move, MoveType},
        pieces::Piece,
    };

    use crate::{incremental_sort::IncrementalSort, move_order::MoveOrder};

    use super::OrderedMove;

    #[test]
    fn verify_ordered_move_order() {
        let mut moves = vec![
            OrderedMove {
                order: MoveOrder::Capture(Piece::Bishop, Piece::Rook),
                mv: Move::default(),
            },
            OrderedMove {
                order: MoveOrder::Capture(Piece::Queen, Piece::Pawn),
                mv: Move::default(),
            },
            OrderedMove {
                order: MoveOrder::Quiet(15),
                mv: Move::default(),
            },
            OrderedMove {
                order: MoveOrder::Quiet(14),
                mv: Move::default(),
            },
            OrderedMove {
                order: MoveOrder::TtMove,
                mv: Move::default(),
            },
        ];

        moves.sort();

        let expected_order = vec![
            MoveOrder::TtMove,
            MoveOrder::Capture(Piece::Queen, Piece::Pawn),
            MoveOrder::Capture(Piece::Bishop, Piece::Rook),
            MoveOrder::Quiet(15),
            MoveOrder::Quiet(14),
        ];

        for (i, mv) in moves.iter().enumerate() {
            assert_eq!(mv.order, expected_order[i]);
        }
    }

    #[test]
    fn test_incremental_sort() {
        let move_gen = MoveGenerator::new();
        let mut move_list = MoveList::new();
        let board = Board::default_board();

        // generate moves
        move_gen.generate_moves(&board, &mut move_list, MoveType::All);

        let test_moves = vec![
            (MoveOrder::Quiet(15), *move_list.at(5).unwrap()),
            (MoveOrder::Quiet(14), *move_list.at(0).unwrap()),
            (
                MoveOrder::Capture(Piece::Bishop, Piece::Rook),
                *move_list.at(1).unwrap(),
            ),
            (MoveOrder::TtMove, *move_list.at(3).unwrap()),
            (
                MoveOrder::Capture(Piece::Queen, Piece::Pawn),
                *move_list.at(2).unwrap(),
            ),
        ];

        let incremental_sort = IncrementalSort::new(test_moves);

        // check if has next
        assert!(incremental_sort.has_next());

        let expected_move_order = vec![
            move_list.at(3).unwrap(),
            move_list.at(2).unwrap(),
            move_list.at(1).unwrap(),
            move_list.at(5).unwrap(),
            move_list.at(0).unwrap(),
        ];

        for e_mv in expected_move_order.iter() {
            println!("e_mv: {}", e_mv.to_long_algebraic());
        }

        for (i, mv) in incremental_sort.enumerate() {
            println!(
                "mv: {} expected {}",
                mv.to_long_algebraic(),
                expected_move_order[i].to_long_algebraic()
            );
            assert_eq!(mv, *expected_move_order[i]);
        }
    }
}
