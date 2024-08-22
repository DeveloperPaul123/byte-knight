use std::iter::zip;

use crate::definitions::{CastlingAvailability, SPACE};
use crate::fen::FenError;

use super::definitions::{NumberOf, Side};
use super::fen;
use super::{bitboard::Bitboard, pieces::Piece};

pub struct Board {
    piece_bitboards: [[Bitboard; NumberOf::PIECE_TYPES]; NumberOf::SIDES],
    half_move_clock: u32,
    full_move_number: u32,
    side_to_move: Side,
    en_passant_square: Option<u8>,
    castling_rights: u8,
}

// Initializations
impl Board {
    fn new() -> Self {
        Board {
            piece_bitboards: [[Bitboard::default(); NumberOf::PIECE_TYPES]; NumberOf::SIDES],
            half_move_clock: 0,
            full_move_number: 1,
            side_to_move: Side::White,
            en_passant_square: None,
            castling_rights: CastlingAvailability::NONE,
        }
    }

    /// Create a new board with the default starting position.
    pub fn default_board() -> Board {
        let mut board = Board::new();
        // Set up the board with the starting position
        // White pieces
        board.initialize_piece_bbs(Side::White);
        // Black pieces
        board.initialize_piece_bbs(Side::Black);
        board.set_en_passant_square(None);
        board.set_half_move_clock(0);
        board.set_full_move_number(1);
        board.set_side_to_move(Side::White);
        board.set_castling_rights(CastlingAvailability::ALL);
        return board;
    }

    /// Create a new board from a FEN string.
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

    /// Convert the board to a FEN string.
    pub fn to_fen(self) -> String {
        let mut fen = String::new();
        // Piece placement
        fen.push_str(&fen::piece_placement_to_fen(&self));
        fen.push(SPACE);
        // Active color
        fen.push_str(&fen::active_color_to_fen(&self));
        fen.push(SPACE);
        // Castling availability
        fen.push_str(&fen::castling_availability_to_fen(&self));
        fen.push(SPACE);
        // En passant target square
        fen.push_str(&fen::en_passant_target_square_to_fen(&self));
        fen.push(SPACE);
        // Halfmove clock
        fen.push_str(&fen::halfmove_clock_to_fen(&self));
        fen.push(SPACE);
        // Fullmove number
        fen.push_str(&fen::fullmove_number_to_fen(&self));

        return fen;
    }

    /// Initialize bitboards for a given side
    fn initialize_piece_bbs(&mut self, side: Side) {
        // Set up the board with the starting position
        match side {
            Side::White => self.initialize_white_bbs(),
            Side::Black => self.initialize_black_bbs(),
            _ => panic!("Invalid side"),
        }
    }

    /// Initialize bitboard for all white pieces
    fn initialize_white_bbs(&mut self) {
        let index = Side::White as usize;
        // Set up the board with the starting position
        self.piece_bitboards[index][Piece::Pawn as usize] = Bitboard::new(0xFF00);
        self.piece_bitboards[index][Piece::Knight as usize] = Bitboard::new(0x42);
        self.piece_bitboards[index][Piece::Bishop as usize] = Bitboard::new(0x24);
        self.piece_bitboards[index][Piece::Rook as usize] = Bitboard::new(0x81);
        self.piece_bitboards[index][Piece::Queen as usize] = Bitboard::new(0x8);
        self.piece_bitboards[index][Piece::King as usize] = Bitboard::new(0x10);
    }

    /// Initialize bitboard for all black pieces
    fn initialize_black_bbs(&mut self) {
        let index = Side::Black as usize;
        // Set up the board with the starting position
        self.piece_bitboards[index][Piece::Pawn as usize] = Bitboard::new(0xFF000000000000);
        self.piece_bitboards[index][Piece::Knight as usize] = Bitboard::new(0x4200000000000000);
        self.piece_bitboards[index][Piece::Bishop as usize] = Bitboard::new(0x2400000000000000);
        self.piece_bitboards[index][Piece::Rook as usize] = Bitboard::new(0x8100000000000000);
        self.piece_bitboards[index][Piece::Queen as usize] = Bitboard::new(0x800000000000000);
        self.piece_bitboards[index][Piece::King as usize] = Bitboard::new(0x1000000000000000);
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
            white_pieces |= self.piece_bitboards[Side::White as usize][piece_type];
        }
        return white_pieces;
    }

    pub fn black_pieces(&self) -> Bitboard {
        let mut black_pieces = Bitboard::default();
        for piece_type in 0..NumberOf::PIECE_TYPES {
            black_pieces |= self.piece_bitboards[Side::Black as usize][piece_type];
        }
        return black_pieces;
    }

    pub fn piece_bitboard(&self, piece: Piece, side: Side) -> &Bitboard {
        return &self.piece_bitboards[side as usize][piece as usize];
    }

    pub(crate) fn set_piece_square(&mut self, piece: usize, side: usize, square: usize) {
        self.piece_bitboards[side][piece].set_square(square);
    }

    pub fn piece_on_square(&self, square: usize) -> Option<(Piece, usize)> {
        for piece in 0..NumberOf::PIECE_TYPES {
            for side in 0..NumberOf::SIDES {
                if self.piece_bitboards[side][piece].is_square_occupied(square) {
                    return Some((Piece::try_from(piece as u8).unwrap(), side));
                }
            }
        }
        return None;
    }

    pub(crate) fn set_side_to_move(&mut self, side: Side) {
        self.side_to_move = side;
    }

    pub fn side_to_move(&self) -> Side {
        return self.side_to_move;
    }

    pub(crate) fn set_en_passant_square(&mut self, square: Option<u8>) {
        // TODO: Check if square is valid
        self.en_passant_square = square;
    }

    pub fn en_passant_square(&self) -> Option<u8> {
        return self.en_passant_square;
    }

    pub(crate) fn set_half_move_clock(&mut self, half_move_clock: u32) {
        self.half_move_clock = half_move_clock;
    }

    pub fn half_move_clock(&self) -> u32 {
        return self.half_move_clock;
    }

    pub(crate) fn set_full_move_number(&mut self, full_move_number: u32) {
        self.full_move_number = full_move_number;
    }

    pub fn full_move_number(&self) -> u32 {
        return self.full_move_number;
    }

    pub(crate) fn set_castling_rights(&mut self, castling_rights: u8) {
        self.castling_rights = castling_rights;
    }

    pub fn castling_rights(&self) -> u8 {
        return self.castling_rights;
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn test_default_board() {
        let board = Board::default_board();
        assert_eq!(board.all_pieces(), 0xFFFF00000000FFFF);
    }
}
