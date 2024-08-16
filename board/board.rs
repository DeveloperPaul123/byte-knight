use std::iter::zip;

use crate::fen::FenError;

use super::definitions::{NumberOf, Side};
use super::fen;
use super::{bitboard::Bitboard, pieces::Pieces};

pub struct Board {
    piece_bitboards: [[Bitboard; NumberOf::PIECE_TYPES]; NumberOf::SIDES],
    half_move_clock: u32,
    full_move_number: u32,
}

/// Initializations
impl Board {
    fn new() -> Self {
        Board {
            piece_bitboards: [[Bitboard::default(); NumberOf::PIECE_TYPES]; NumberOf::SIDES],
            half_move_clock: 0,
            full_move_number: 1,
        }
    }

    /// Create a new board with the default starting position.
    pub fn default_board() -> Board {
        let mut board = Board::new();
        // Set up the board with the starting position
        // White pieces
        board.initialize_piece_bbs(Side::WHITE);
        // Black pieces
        board.initialize_piece_bbs(Side::BLACK);

        return board;
    }

    pub fn from_fen(fen: &str) -> Result<Board, FenError> {
        let mut board = Board::new();

        // parse the FEN string
        let fen_parts = fen::split_fen_string(fen);
        match fen_parts {
            Ok(parts) => {
                let fen_part_parsers = fen::FEN_PART_PARSERS;
                for (part, parser) in zip(parts, fen_part_parsers) {
                    parser(&mut board, &part)?;
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
        return Ok(board);
    }

    /// Initialize bitboards for a given side
    fn initialize_piece_bbs(&mut self, side: usize) {
        // Set up the board with the starting position
        match side {
            Side::WHITE => self.initialize_white_bbs(),
            Side::BLACK => self.initialize_black_bbs(),
            _ => panic!("Invalid side"),
        }
    }

    /// Initialize bitboard for all white pieces
    fn initialize_white_bbs(&mut self) {
        // Set up the board with the starting position
        self.piece_bitboards[Side::WHITE][Pieces::PAWN as usize] = Bitboard::new(0xFF00);
        self.piece_bitboards[Side::WHITE][Pieces::KNIGHT as usize] = Bitboard::new(0x42);
        self.piece_bitboards[Side::WHITE][Pieces::BISHOP as usize] = Bitboard::new(0x24);
        self.piece_bitboards[Side::WHITE][Pieces::ROOK as usize] = Bitboard::new(0x81);
        self.piece_bitboards[Side::WHITE][Pieces::QUEEN as usize] = Bitboard::new(0x8);
        self.piece_bitboards[Side::WHITE][Pieces::KING as usize] = Bitboard::new(0x10);
    }

    /// Initialize bitboard for all black pieces
    fn initialize_black_bbs(&mut self) {
        // Set up the board with the starting position
        self.piece_bitboards[Side::BLACK][Pieces::PAWN as usize] = Bitboard::new(0xFF000000000000);
        self.piece_bitboards[Side::BLACK][Pieces::KNIGHT as usize] =
            Bitboard::new(0x4200000000000000);
        self.piece_bitboards[Side::BLACK][Pieces::BISHOP as usize] =
            Bitboard::new(0x2400000000000000);
        self.piece_bitboards[Side::BLACK][Pieces::ROOK as usize] =
            Bitboard::new(0x8100000000000000);
        self.piece_bitboards[Side::BLACK][Pieces::QUEEN as usize] =
            Bitboard::new(0x800000000000000);
        self.piece_bitboards[Side::BLACK][Pieces::KING as usize] =
            Bitboard::new(0x1000000000000000);
    }
}

/// Piece operations
impl Board {
    pub fn all_pieces(&self) -> Bitboard {
        let mut all_pieces = Bitboard::default();
        for piece_type in 0..NumberOf::PIECE_TYPES {
            for side in 0..NumberOf::SIDES {
                all_pieces |= self.piece_bitboards[side][piece_type];
            }
        }
        return all_pieces;
    }

    pub fn white_pieces(&self) -> Bitboard {
        let mut white_pieces = Bitboard::default();
        for piece_type in 0..NumberOf::PIECE_TYPES {
            white_pieces |= self.piece_bitboards[Side::WHITE][piece_type];
        }
        return white_pieces;
    }

    pub fn black_pieces(&self) -> Bitboard {
        let mut black_pieces = Bitboard::default();
        for piece_type in 0..NumberOf::PIECE_TYPES {
            black_pieces |= self.piece_bitboards[Side::BLACK][piece_type];
        }
        return black_pieces;
    }

    pub fn piece_bitboard(&self, piece: usize, side: usize) -> &Bitboard {
        return &self.piece_bitboards[side][piece];
    }

    pub(crate) fn set_piece_square(&mut self, piece: usize, side: usize, square: usize) {
        self.piece_bitboards[side][piece].set_square(square);
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn test_default_board() {
        let board = Board::default_board();
        let all_pieces = board.all_pieces();
        let black = board.black_pieces();
        let white = board.white_pieces();
        println!("black: \n\n{}", black);
        println!("white: \n\n{}", white);
        println!("all: \n\n{}", all_pieces);
        assert_eq!(board.all_pieces(), 0xFFFF00000000FFFF);
    }
}
