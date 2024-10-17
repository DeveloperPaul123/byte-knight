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

use std::backtrace;
use std::iter::zip;

use crate::board_state::BoardState;
use crate::definitions::{CastlingAvailability, SPACE};
use crate::fen::FenError;
use crate::move_generation::MoveGenerator;
use crate::move_history::BoardHistory;
use crate::move_list::MoveList;
use crate::moves::{Move, MoveType};
use crate::square::Square;
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

impl Clone for Board {
    fn clone(&self) -> Self {
        Self {
            piece_bitboards: self.piece_bitboards.clone(),
            history: self.history.clone(),
            state: self.state.clone(),
            zobrist_values: self.zobrist_values.clone(),
        }
    }
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

    pub(crate) fn mut_piece_bitboard(&mut self, piece: Piece, side: Side) -> &mut Bitboard {
        return &mut self.piece_bitboards[side as usize][piece as usize];
    }

    pub(crate) fn set_piece_square(&mut self, piece: usize, side: usize, square: u8) {
        self.piece_bitboards[side][piece].set_square(square);
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

    pub(crate) fn set_half_move_clock(&mut self, half_move_clock: u32) {
        self.state.half_move_clock = half_move_clock;
    }

    pub(crate) fn set_full_move_number(&mut self, full_move_number: u32) {
        self.state.full_move_number = full_move_number;
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
    pub fn to_fen(&self) -> String {
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

    /// Find what piece is on a given square.
    ///
    /// Returns an optional tuple of the piece and the side that the piece belongs to.
    pub fn piece_on_square(&self, square: u8) -> Option<(Piece, Side)> {
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

    /// Returns the side to move of this [`Board`].
    pub fn side_to_move(&self) -> Side {
        return self.state.side_to_move;
    }

    pub fn en_passant_square(&self) -> Option<u8> {
        return self.state.en_passant_square;
    }

    pub fn half_move_clock(&self) -> u32 {
        return self.state.half_move_clock;
    }

    pub fn full_move_number(&self) -> u32 {
        return self.state.full_move_number;
    }

    pub fn castling_rights(&self) -> u8 {
        return self.state.castling_rights;
    }

    pub fn zobrist_hash(&self) -> u64 {
        return self.state.zobrist_hash;
    }

    pub fn is_square_on_rank(square: u8, rank: u8) -> bool {
        let (_, rnk) = square::from_square(square);
        return rnk == rank;
    }

    pub fn is_square_empty(&self, square: &Square) -> bool {
        return !self
            .all_pieces()
            .is_square_occupied(square.to_square_index());
    }

    pub fn can_castle_kingside(&self, side: Side) -> bool {
        let castling_rights = self.castling_rights();
        return match side {
            Side::White => castling_rights & CastlingAvailability::WHITE_KINGSIDE != 0,
            Side::Black => castling_rights & CastlingAvailability::BLACK_KINGSIDE != 0,
            Side::Both => panic!("Cannot check if both sides can castle kingside"),
        };
    }

    pub fn can_castle_queenside(&self, side: Side) -> bool {
        let castling_rights = self.castling_rights();
        return match side {
            Side::White => castling_rights & CastlingAvailability::WHITE_QUEENSIDE != 0,
            Side::Black => castling_rights & CastlingAvailability::BLACK_QUEENSIDE != 0,
            Side::Both => panic!("Cannot check if both sides can castle queenside"),
        };
    }

    pub fn is_in_check(&self, move_gen: &MoveGenerator) -> bool {
        // pseudo legal check
        // check if we are in check
        // get the kings location and check if that square is attacked by the opponent
        let mut king_bb = self
            .piece_bitboard(Piece::King, self.side_to_move())
            .clone();
        let king_square = bitboard_helpers::next_bit(&mut king_bb) as u8;
        return move_gen.is_square_attacked(
            self,
            &Square::from_square_index(king_square),
            Side::opposite(self.side_to_move()),
        );
    }

    /// Get the color of the piece on a given square.
    ///
    /// Returns `Some(Side)` if the square is occupied, otherwise `None`.
    pub fn color_on(&self, square: u8) -> Option<Side> {
        let white_pieces = self.white_pieces();
        let black_pieces = self.black_pieces();
        if white_pieces.is_square_occupied(square) {
            Some(Side::White)
        } else if black_pieces.is_square_occupied(square) {
            Some(Side::Black)
        } else {
            None
        }
    }

    pub fn is_stalemate(&self, move_gen: &MoveGenerator) -> bool {
        // if the side to move is in check, it's not stalemate
        if self.is_in_check(move_gen) {
            return false;
        }

        // if the side to move has no legal moves, it's stalemate
        let mut move_list = MoveList::new();
        move_gen.generate_moves(self, &mut move_list, MoveType::All);
        return move_list.len() == 0;
    }

    pub fn is_legal(&self, mv: &Move, move_gen: &MoveGenerator) -> bool {
        // check if a move is legal without altering the current board state
        let mut board_copy = self.clone();
        board_copy.make_move(mv, move_gen).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::{File, Rank, Squares, DEFAULT_FEN},
        move_generation::MoveGenerator,
        move_list::{self, MoveList},
        moves::{Move, MoveDescriptor, MoveType},
        square::Square,
    };

    use super::*;

    #[test]
    fn test_default_board() {
        let board = Board::default_board();
        assert_eq!(board.all_pieces(), 0xFFFF00000000FFFF);
        assert_eq!(board.to_fen(), DEFAULT_FEN);
    }

    #[test]
    fn make_and_unmake_move_changes_hash() {
        static FEN: &str = "6nr/pp3p1p/k1p5/8/1QN5/2P1P3/4KPqP/8 b - - 5 26";
        let move_gen = MoveGenerator::new();
        let mut move_list = MoveList::new();
        let mut board = Board::from_fen(FEN).unwrap();
        let hash = board.zobrist_hash();

        move_gen.generate_moves(&board, &mut move_list, MoveType::All);

        for mv in move_list.iter() {
            let mv_ok = board.make_move(mv, &move_gen);
            if mv_ok.is_ok() {
                // legal move, check that the new hash is different
                let move_hash = board.zobrist_hash();
                assert_ne!(hash, move_hash);
                // undo the move
                let undo_result = board.unmake_move();
                assert!(undo_result.is_ok());
                // check that the hash is back to the original value
                assert_eq!(hash, board.zobrist_hash());
            }
        }
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

    #[test]
    fn check_square_is_empty() {
        let board = Board::default_board();
        // all of these squares should be empty.
        for rank in (Rank::R3 as u8)..=(Rank::R6 as u8) {
            for file in (File::A as u8)..=(File::H as u8) {
                let square = square::to_square_object(file, rank);
                assert!(board.is_square_empty(&square));
            }
        }
    }

    #[test]
    fn properly_undo_piece_promotion() {
        let move_gen = MoveGenerator::new();
        let mut move_list = MoveList::new();
        let mut board =
            Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        move_gen.generate_moves(&board, &mut move_list, MoveType::All);

        let initial_mv = move_list
            .iter()
            .find(|mv| mv.to_long_algebraic() == "d7c8q")
            .unwrap()
            .clone();

        let mut queen_bb = board.piece_bitboard(Piece::Queen, Side::White).clone();
        assert_eq!(queen_bb.number_of_occupied_squares(), 1);
        assert_eq!(queen_bb, Bitboard::from_square(Squares::D1));

        let mv_ok = board.make_move(&initial_mv, &move_gen);
        assert!(mv_ok.is_ok());

        queen_bb = board.piece_bitboard(Piece::Queen, Side::White).clone();
        assert_eq!(queen_bb.number_of_occupied_squares(), 2);
        let mut compare_bb = Bitboard::from_square(Squares::D1);
        compare_bb.set_square(Squares::C8);
        assert_eq!(queen_bb, compare_bb);

        let undo_result = board.unmake_move();
        assert!(undo_result.is_ok());

        queen_bb = board.piece_bitboard(Piece::Queen, Side::White).clone();
        assert_eq!(queen_bb.number_of_occupied_squares(), 1);
        assert_eq!(queen_bb, Bitboard::from_square(Squares::D1));
    }

    #[test]
    fn make_move_updates_piece_boards() {
        let move_gen = MoveGenerator::new();
        let mut move_list = MoveList::new();
        let mut board =
            Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        move_gen.generate_moves(&board, &mut move_list, MoveType::All);

        let initial_mv = move_list
            .iter()
            .find(|mv| mv.to_long_algebraic() == "d7c8q")
            .unwrap()
            .clone();

        let next_move = move_list
            .iter()
            .find(|mv| mv.to_long_algebraic() == "d7c8r")
            .unwrap()
            .clone();

        let mut mv_ok = board.make_move(&initial_mv, &move_gen);
        assert!(mv_ok.is_ok());

        // generate moves again
        move_list.clear();
        move_gen.generate_moves(&board, &mut move_list, MoveType::All);
        let mut node_count = 0;
        for mv in move_list.iter() {
            println!("trying move {}", mv.to_long_algebraic());
            mv_ok = board.make_move(mv, &move_gen);
            if mv_ok.is_ok() {
                node_count += 1;
                let undo_result = board.unmake_move();
                assert!(undo_result.is_ok());
            }
        }

        assert_eq!(node_count, 31);

        println!("\n{}\n{}", board.to_fen(), board.board_state());
        println!(
            "queen before:\n{}",
            board.piece_bitboard(Piece::Queen, Side::White)
        );

        let initial_move_undo_result = board.unmake_move();
        assert!(initial_move_undo_result.is_ok());

        println!("\n{}\n{}", board.to_fen(), board.board_state());
        println!(
            "queen after:\n{}",
            board.piece_bitboard(Piece::Queen, Side::White)
        );

        mv_ok = board.make_move(&next_move, &move_gen);
        assert!(mv_ok.is_ok());

        println!(
            "rook after move:\n{}",
            board.piece_bitboard(Piece::Rook, Side::White)
        );

        move_list.clear();
        move_gen.generate_moves(&board, &mut move_list, MoveType::All);
        node_count = 0;

        for mv in move_list.iter() {
            mv_ok = board.make_move(mv, &move_gen);
            if mv_ok.is_ok() {
                println!("{} 1", mv.to_long_algebraic());
                node_count += 1;
                let undo_result = board.unmake_move();
                assert!(undo_result.is_ok());
            }
        }

        assert_eq!(node_count, 31);
    }

    #[test]
    fn make_move_and_undo_move() {
        let move_gen = MoveGenerator::new();
        let mut move_list = MoveList::new();
        let mut board =
            Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

        move_gen.generate_moves(&board, &mut move_list, MoveType::All);

        let first_mv = move_list
            .iter()
            .find(|mv| mv.to_long_algebraic() == "b1d2")
            .unwrap();
        let second_mv = move_list
            .iter()
            .find(|mv| mv.to_long_algebraic() == "b1a3")
            .unwrap();

        println!("{}\n{}", board.to_fen(), board.board_state());
        let mut mv_ok = board.make_move(first_mv, &move_gen);
        assert!(mv_ok.is_ok());
        println!("{}\n{}", board.to_fen(), board.board_state());
        // undo the move
        let mut undo_ok = board.unmake_move();
        assert!(undo_ok.is_ok());
        println!("{}\n{}", board.to_fen(), board.board_state());

        // make the second move
        mv_ok = board.make_move(second_mv, &move_gen);
        assert!(mv_ok.is_ok());
        // undo the move
        undo_ok = board.unmake_move();
        assert!(undo_ok.is_ok());
    }
}
