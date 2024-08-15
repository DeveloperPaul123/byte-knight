use chess::{Board, Piece, ALL_PIECES};
use chess_board_helpers::is_in_checkmate;

use super::chess_board_helpers::{self, piece_for_color};

struct Evaluation;

impl Evaluation {
    pub fn evaluate_position(board: &Board) -> i64 {
        if is_in_checkmate(board) {
            return if board.side_to_move() == chess::Color::White {
                i64::MIN
            } else {
                i64::MAX
            };
        }
        let mut sum: i64 = 0;
        for piece in ALL_PIECES.iter() {
            let black_bb = piece_for_color(board, piece, chess::Color::Black);
            let white_bb = piece_for_color(board, piece, chess::Color::White);
            let piece_value = match piece {
                Piece::Pawn => 1,
                Piece::Knight => 3,
                Piece::Bishop => 3,
                Piece::Rook => 5,
                Piece::Queen => 9,
                Piece::King => 0,
            };
            sum += (black_bb.popcnt() as i64 - white_bb.popcnt() as i64) * piece_value;
        }
        return sum;
    }
}
