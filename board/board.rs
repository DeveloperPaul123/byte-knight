/*
 * board.rs
 * Part of the byte-knight project
 * Created Date: Wednesday, August 21st 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified:
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::iter::zip;

use crate::board_state::BoardState;
use crate::definitions::{CastlingAvailability, SPACE};
use crate::fen::FenError;
use crate::move_history::BoardHistory;
use crate::zobrist::{ZobristHash, ZobristRandomValues};
use crate::{bitboard_helpers, square};

use super::definitions::{NumberOf, Side};
use super::fen;
use super::{bitboard::Bitboard, pieces::Piece};

pub struct Board {
    piece_bitboards: [[Bitboard; NumberOf::PIECE_TYPES]; NumberOf::SIDES],
    pub(crate) history: BoardHistory,
    state: BoardState,
    zobrist_values: ZobristRandomValues,
}

// Private methods
impl Board {
    fn new() -> Self {
        Board {
            piece_bitboards: [[Bitboard::default(); NumberOf::PIECE_TYPES]; NumberOf::SIDES],
            history: BoardHistory::new(),
            state: BoardState::new(),
            zobrist_values: ZobristRandomValues::new(),
        }
    }

    pub(crate) fn initialize(&mut self) {
        self.state.zobrist_hash = self.initialize_zobrist_hash();
    }

    fn initialize_zobrist_hash(&self) -> ZobristHash {
        // create the initial zobrist hash based on the starting position
        // for each piece on the board, get the corresponding zobrist value and xor it with the hash
        // for each side to move, xor the hash with the zobrist value for the side
        // for each castling right, xor the hash with the zobrist value for the castling right
        // for the en passant square, xor the hash with the zobrist value for the en passant square
        // Initialize the zobrist hash to 0
        let mut zobrist_hash = ZobristHash::default();

        // XOR the zobrist values for each piece on the board
        for side in 0..NumberOf::SIDES {
            for piece in 0..NumberOf::PIECE_TYPES {
                let mut bitboard = self.piece_bitboards[side][piece].clone();

                while bitboard != 0 {
                    let square = bitboard_helpers::next_bit(&mut bitboard);
                    zobrist_hash ^=
                        self.zobrist_values
                            .get_piece_value(piece, side, square as usize);
                }
            }
        }

        // XOR the zobrist value for the side to move
        zobrist_hash ^= self
            .zobrist_values
            .get_side_value(self.side_to_move() as usize);

        // XOR the zobrist values for castling rights
        zobrist_hash ^= self
            .zobrist_values
            .get_castling_value(self.castling_rights() as usize);

        // XOR the zobrist value for the en passant square, if any
        zobrist_hash ^= self
            .zobrist_values
            .get_en_passant_value(self.state.en_passant_square);

        zobrist_hash
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

// Public API
impl Board {
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
        board.set_zobrist_hash(board.initialize_zobrist_hash());
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

        // the parser initializes most of the board state, but we need to set the zobrist hash
        // initializing the board will handle initializing anything that isn't set by the FEN parser
        board.initialize();

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

    /// Returns the all pieces of this [`Board`].
    /// This is also known as the occupancy bitboard.
    pub fn all_pieces(&self) -> Bitboard {
        let mut all_pieces = Bitboard::default();
        for piece_type in 0..NumberOf::PIECE_TYPES {
            for side in 0..NumberOf::SIDES {
                all_pieces |= self.piece_bitboards[side][piece_type];
            }
        }
        return all_pieces;
    }

    /// Returns all the pieces of a given side in a single [`Bitboard`].
    pub fn pieces(&self, side: Side) -> Bitboard {
        let mut pieces = Bitboard::default();
        for piece_type in 0..NumberOf::PIECE_TYPES {
            pieces |= self.piece_bitboards[side as usize][piece_type];
        }
        return pieces;
    }

    /// Returns the white pieces of this [`Board`] in a single [`Bitboard`].
    pub fn white_pieces(&self) -> Bitboard {
        return self.pieces(Side::White);
    }

    /// Returns the black pieces of this [`Board`] in a single [`Bitboard`].
    pub fn black_pieces(&self) -> Bitboard {
        return self.pieces(Side::Black);
    }

    /// Returns the bitboard for a specific piece and side.
    pub fn piece_bitboard(&self, piece: Piece, side: Side) -> &Bitboard {
        return &self.piece_bitboards[side as usize][piece as usize];
    }

    pub(crate) fn mut_piece_bitboard(&mut self, piece: Piece, side: Side) -> &mut Bitboard {
        return &mut self.piece_bitboards[side as usize][piece as usize];
    }

    pub(crate) fn set_piece_square(&mut self, piece: usize, side: usize, square: usize) {
        self.piece_bitboards[side][piece].set_square(square);
    }

    /// Find what piece is on a given square.
    ///
    /// Returns an optional tuple of the piece and the side that the piece belongs to.
    pub fn piece_on_square(&self, square: usize) -> Option<(Piece, Side)> {
        for piece in 0..NumberOf::PIECE_TYPES {
            for side in 0..NumberOf::SIDES {
                if self.piece_bitboards[side][piece].is_square_occupied(square) {
                    return Some((
                        Piece::try_from(piece as u8).unwrap(),
                        Side::try_from(side as u8).unwrap(),
                    ));
                }
            }
        }
        return None;
    }

    /// Sets the side to move and updates the zobrist hash.
    pub(crate) fn set_side_to_move(&mut self, side: Side) {
        // undo the current side to move in the hash
        self.state.zobrist_hash ^= self
            .zobrist_values
            .get_side_value(self.state.side_to_move as usize);
        // set the new side to move
        self.state.side_to_move = side;
        // update zobrist hash with the new side to move
        self.state.zobrist_hash ^= self
            .zobrist_values
            .get_side_value(self.state.side_to_move as usize);
    }

    /// Returns the side to move of this [`Board`].
    pub fn side_to_move(&self) -> Side {
        return self.state.side_to_move;
    }

    /// Set the en passant square and update the zobrist hash.
    pub(crate) fn set_en_passant_square(&mut self, square: Option<u8>) {
        self.state.zobrist_hash ^= self
            .zobrist_values
            .get_en_passant_value(self.state.en_passant_square);
        self.state.en_passant_square = square;
        self.state.zobrist_hash ^= self
            .zobrist_values
            .get_en_passant_value(self.state.en_passant_square);
    }

    pub fn en_passant_square(&self) -> Option<u8> {
        return self.state.en_passant_square;
    }

    pub(crate) fn set_half_move_clock(&mut self, half_move_clock: u32) {
        self.state.half_move_clock = half_move_clock;
    }

    pub fn half_move_clock(&self) -> u32 {
        return self.state.half_move_clock;
    }

    pub(crate) fn set_full_move_number(&mut self, full_move_number: u32) {
        self.state.full_move_number = full_move_number;
    }

    pub fn full_move_number(&self) -> u32 {
        return self.state.full_move_number;
    }

    pub(crate) fn set_castling_rights(&mut self, castling_rights: u8) {
        self.state.zobrist_hash ^= self
            .zobrist_values
            .get_castling_value(self.state.castling_rights as usize);
        self.state.castling_rights = castling_rights;
        self.state.zobrist_hash ^= self
            .zobrist_values
            .get_castling_value(self.state.castling_rights as usize);
    }

    pub fn castling_rights(&self) -> u8 {
        return self.state.castling_rights;
    }

    pub fn zobrist_hash(&self) -> u64 {
        return self.state.zobrist_hash;
    }

    pub(crate) fn update_zobrist_hash_for_piece(&mut self, square: u8, piece: Piece, side: Side) {
        self.state.zobrist_hash ^=
            self.zobrist_values
                .get_piece_value(piece as usize, side as usize, square as usize);
    }

    fn set_zobrist_hash(&mut self, hash: u64) {
        self.state.zobrist_hash = hash;
    }

    pub(crate) fn board_state(&self) -> &BoardState {
        return &self.state;
    }

    pub(crate) fn set_board_state(&mut self, state: BoardState) {
        self.state = state;
    }

    pub fn is_square_on_rank(square: u8, rank: u8) -> bool {
        let (_, rnk) = square::from_square(square);
        return rnk == rank;
    }
}

#[cfg(test)]
mod board_tests {
    use crate::{
        definitions::{File, Rank, Squares},
        moves::Move,
        square::Square,
    };

    use super::*;

    #[test]
    fn test_default_board() {
        let board = Board::default_board();
        assert_eq!(board.all_pieces(), 0xFFFF00000000FFFF);
    }

    #[test]
    fn make_and_unmake_move_changes_hash() {
        static FEN: &str = "6nr/pp3p1p/k1p5/8/1QN5/2P1P3/4KPqP/8 b - - 5 26";

        let mut board = Board::from_fen(FEN).unwrap();
        let hash = board.zobrist_hash();
        let chess_move = Move::new(
            &Square::new(File::F, Rank::R7),
            &Square::new(File::F, Rank::R5),
            0,
            Piece::Pawn,
            None,
        );
        let mut mv_ok = board.make_move(&chess_move);
        assert!(mv_ok.is_ok());
        assert_ne!(hash, board.zobrist_hash());
        let move_hash = board.zobrist_hash();
        mv_ok = board.unmake_move();
        assert!(mv_ok.is_ok());
        let unmake_hash = board.zobrist_hash();
        assert_ne!(unmake_hash, move_hash);
        assert_eq!(unmake_hash, hash);
    }

    #[test]
    fn make_move_updates_castling_rights() {
        // TODO
    }

    #[test]
    fn check_square_on_rank() {
        assert!(Board::is_square_on_rank(Squares::A1, Rank::R1 as u8));
        assert!(!Board::is_square_on_rank(Squares::A1, Rank::R2 as u8));
        assert!(Board::is_square_on_rank(Squares::C5, Rank::R5 as u8));
    }
}
