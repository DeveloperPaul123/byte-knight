use crate::move_order::MoveOrder;
use chess::moves::Move;

#[derive(Debug, Clone)]
pub(crate) struct OrderedMove {
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

#[cfg(test)]
mod tests {
    use chess::{moves::Move, pieces::Piece};

    use crate::move_order::MoveOrder;

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
}
