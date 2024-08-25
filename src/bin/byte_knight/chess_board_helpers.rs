use chess::{BitBoard, Board, Piece};

pub fn is_in_checkmate(board: &Board) -> bool {
    return false;
}

pub fn piece_for_color(board: &Board, piece: &Piece, color: chess::Color) -> BitBoard {
    return board.pieces(*piece) & board.color_combined(color);
}
