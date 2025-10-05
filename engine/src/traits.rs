use chess::{pieces::Piece, side::Side};

use crate::score::Score;

pub trait Eval<Board> {
    fn eval(&self, board: &Board) -> Score;
}

pub trait EvalValues {
    type ReturnScore;
    fn psqt(&self, square: u8, piece: Piece, side: Side) -> Self::ReturnScore;
    fn passed_pawn_bonus(&self) -> Self::ReturnScore;
}
