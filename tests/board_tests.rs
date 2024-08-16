use byte_board::board::Board;

#[test]
fn test_board_construction() {
    let board = Board::default_board();
    assert_eq!(board.all_pieces(), 0xFFFF00000000FFFF);
}

#[test]
fn construct_board_from_fen_string() {
    let board_result = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    assert!(board_result.is_ok());
    let board = board_result.unwrap();
    assert_eq!(board.all_pieces(), 0xFFFF00000000FFFF);
}
