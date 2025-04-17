use crate::ordered_move::OrderedMove;
use chess::moves::Move;

pub(crate) struct IncrementalSort<'s> {
    data: &'s mut [OrderedMove],
    current_index: usize,
}

impl<'s> IncrementalSort<'s> {
    pub(crate) fn new(move_list: &'s mut Vec<OrderedMove>) -> Self {
        Self {
            data: move_list.as_mut_slice(),
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
        Some(self.data[current].mv)
    }
}

impl<'s> Iterator for IncrementalSort<'s> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.data.len() - self.current_index;
        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.data.len() - self.current_index
    }
}

#[cfg(test)]
mod tests {
    use chess::{
        board::Board, move_generation::MoveGenerator, move_list::MoveList, moves::MoveType,
        pieces::Piece,
    };

    use crate::{
        incremental_sort::IncrementalSort, move_order::MoveOrder, ordered_move::OrderedMove,
    };

    #[test]
    fn test_incremental_sort() {
        let move_gen = MoveGenerator::new();
        let mut move_list = MoveList::new();
        let board = Board::default_board();

        // generate moves
        move_gen.generate_moves(&board, &mut move_list, MoveType::All);

        let mut test_moves = vec![
            OrderedMove {
                order: MoveOrder::Quiet(14),
                mv: *move_list.at(0).unwrap(),
            },
            OrderedMove {
                order: MoveOrder::Quiet(15),
                mv: *move_list.at(5).unwrap(),
            },
            OrderedMove {
                order: MoveOrder::Capture(Piece::Bishop, Piece::Rook),
                mv: *move_list.at(1).unwrap(),
            },
            OrderedMove {
                order: MoveOrder::TtMove,
                mv: *move_list.at(3).unwrap(),
            },
            OrderedMove {
                order: MoveOrder::Capture(Piece::Queen, Piece::Pawn),
                mv: *move_list.at(2).unwrap(),
            },
        ];

        let print_test_moves = |msg: &str, mvs: &[OrderedMove]| {
            println!("{}:", msg);
            for i in 0..mvs.len() {
                println!("test_moves[{}]: {:?}", i, mvs[i]);
            }
        };

        print_test_moves("before sorting", &test_moves);

        let incremental_sort = IncrementalSort::new(&mut test_moves);

        // check if has next
        assert!(incremental_sort.has_next());

        let expected_move_order = vec![
            move_list.at(3).unwrap(),
            move_list.at(2).unwrap(),
            move_list.at(1).unwrap(),
            move_list.at(5).unwrap(),
            move_list.at(0).unwrap(),
        ];

        // loop through the iterator
        for (i, mv) in incremental_sort.enumerate() {
            assert_eq!(mv, *expected_move_order[i]);
        }

        print_test_moves("after sorting", &test_moves);
        // check if the vector has been sorted
        for i in 0..test_moves.len() {
            assert_eq!(test_moves[i].mv, *expected_move_order[i]);
        }
    }
}
