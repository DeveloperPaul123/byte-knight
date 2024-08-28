use chess::CastleRights;

use crate::{
    board::Board,
    definitions::{CastlingAvailability, File, Rank, Side, Squares},
    moves::Move,
    pieces::Piece,
    square::{self, to_square},
};

impl Board {
    pub fn make_move(&mut self, mv: &Move) -> Result<(), &'static str> {
        let mut current_state = self.board_state().clone();
        current_state.next_move = mv.clone();
        // update history before modifying the current state
        self.history.push(current_state);

        let from = mv.from();
        let to: u8 = mv.to();
        let piece = mv.piece();
        let captured_piece = mv.captured_piece();

        let us = self.side_to_move();
        let them = Side::opposite(us);
        let can_castle = self.castling_rights() > 0;
        let update_zobrist_hash = true;

        if captured_piece.is_some() {
            let cap = captured_piece.unwrap();
            // remove the captured piece
            self.remove_piece(them, cap, to, update_zobrist_hash);
            // reset half move clock
            self.set_half_move_clock(0);
            //check for need to update castling rights
            if cap == Piece::Rook {
                // TODO: determine what castling rights need to be updated
            }
        }

        if piece == Piece::Pawn {
            self.remove_piece(us, piece, from, update_zobrist_hash);
            // take into account the promotion piece if any
            let piece_to_add = if mv.is_promotion() {
                mv.promotion_piece().unwrap()
            } else {
                piece
            };
            self.add_piece(us, piece_to_add, to, update_zobrist_hash);

            if mv.is_en_passant_capture() {
                // remove the captured pawn
                // TODO: Is ^8 right?
                self.remove_piece(them, Piece::Pawn, to ^ 8, update_zobrist_hash);
            }
            // check if this is a double pawn push
            // if so, set the en passant square
            if mv.is_pawn_two_up() {
                // get the en passant square from the new move
                // TODO: Is this right?
                self.set_en_passant_square(Some(to ^ 8));
            }
        } else {
            // just move the piece
            self.move_piece(us, piece, from, to, update_zobrist_hash)
        }

        // todo:update the castling rights
        if can_castle && (piece == Piece::King || piece == Piece::Rook) {
            // TODO: update castling rights
            self.set_castling_rights(
                self.castling_rights() & !(get_casting_right_to_remove(us, from)),
            );
        }

        if mv.is_castle() {
            // Handle castling, note that we've already moved the piece in question, which in this case would be the king.
            // So now we need to move the rook to the correct square.
            match to {
                Squares::G1 => self.move_piece(
                    us,
                    Piece::Rook,
                    Squares::H1,
                    Squares::F1,
                    update_zobrist_hash,
                ),
                Squares::C1 => self.move_piece(
                    us,
                    Piece::Rook,
                    Squares::A1,
                    Squares::D1,
                    update_zobrist_hash,
                ),
                Squares::G8 => self.move_piece(
                    us,
                    Piece::Rook,
                    Squares::H8,
                    Squares::F8,
                    update_zobrist_hash,
                ),
                Squares::C8 => self.move_piece(
                    us,
                    Piece::Rook,
                    Squares::A8,
                    Squares::D8,
                    update_zobrist_hash,
                ),
                _ => panic!("Invalid castling move"),
            }
        }

        self.switch_side();

        if us == Side::Black {
            self.set_full_move_number(self.half_move_clock() + 1);
        }

        // pseudo legal check
        // check if we are in check
        // TODO: Implement this; we will need to check if the king is in check
        // To do this, we need to check if the king is attacked by the opponent's pieces
        // We don't have attack tables yet, so we'll need to implement them

        return Ok(());
    }

    pub fn unmake_move(&mut self) -> Result<(), &'static str> {
        let maybe_state = self.history.pop();
        if maybe_state.is_none() {
            return Err("No move to unmake");
        }
        let update_zobrist_hash = false;
        let state = maybe_state.unwrap();
        self.set_board_state(state);

        // restore the board state
        let us = self.side_to_move();
        let them = Side::opposite(us);
        // this is move that we're unmaking
        let chess_move = state.next_move;

        let from = chess_move.from();
        let to = chess_move.to();
        let piece = chess_move.piece();
        let captured_piece = chess_move.captured_piece();
        let promoted_piece = chess_move.promotion_piece();
        if promoted_piece.is_some() {
            // remove the promoted piece
            // TODO: Don't update the zobrist hash here
            self.remove_piece(them, promoted_piece.unwrap(), to, update_zobrist_hash);
            // put the pawn back
            self.add_piece(us, Piece::Pawn, from, update_zobrist_hash);
        } else {
            self.undo_move(us, piece, from, to, update_zobrist_hash);
        }

        if chess_move.is_castle() {
            // TODO: Undo castling
        }

        if captured_piece.is_some() {
            // TODO: Restore the captured piece to the board
            self.add_piece(them, captured_piece.unwrap(), to, update_zobrist_hash);
        }

        if chess_move.is_en_passant_capture() {
            // why does ^ 8 work here?
            self.add_piece(them, Piece::Pawn, to ^ 8, update_zobrist_hash);
        }

        return Ok(());
    }

    fn undo_move(&mut self, side: Side, piece: Piece, from: u8, to: u8, update_zobrist_hash: bool) {
        self.remove_piece(side, piece, to, update_zobrist_hash);
        self.add_piece(side, piece, from, update_zobrist_hash);
    }

    /// Add a piece to the board for a given side and square. Will also update the zobrist hash.
    fn add_piece(&mut self, side: Side, piece: Piece, square: u8, update_zobrist_hash: bool) {
        let bb = self.mut_piece_bitboard(piece, side);
        bb.set_square(square as usize);
        if update_zobrist_hash {
            self.update_zobrist_hash_for_piece(square, piece, side)
        }
    }

    /// Remove a piece from the board for a given side and square. Will also update the zobrist hash.
    fn remove_piece(&mut self, side: Side, piece: Piece, square: u8, update_zobrist_hash: bool) {
        let bb = self.mut_piece_bitboard(piece, side);
        bb.clear_square(square as usize);
        if update_zobrist_hash {
            self.update_zobrist_hash_for_piece(square, piece, side)
        }
    }

    /// Move a piece from one square to another. Will update the zobrist hash for the
    ///  removal and addition of the piece for the to and from square.
    fn move_piece(
        &mut self,
        side: Side,
        piece: Piece,
        from: u8,
        to: u8,
        update_zobrist_hash: bool,
    ) {
        self.remove_piece(side, piece, from, update_zobrist_hash);
        self.add_piece(side, piece, to, update_zobrist_hash);
    }

    /// Switch the side to move, and update the zobrist hash (see [Board::set_side_to_move]).
    fn switch_side(&mut self) {
        self.set_side_to_move(Side::opposite(self.side_to_move()));
    }
}

fn get_casting_right_to_remove(us: Side, from: u8) -> u8 {
    match us {
        Side::White => match from {
            // rook moves
            Squares::A1 => CastlingAvailability::WHITE_QUEENSIDE,
            Squares::H1 => CastlingAvailability::WHITE_KINGSIDE,
            Squares::E1 => {
                CastlingAvailability::WHITE_QUEENSIDE | CastlingAvailability::WHITE_KINGSIDE
            }
            _ => 0,
        },
        Side::Black => match from {
            // rook moves
            Squares::A8 => CastlingAvailability::BLACK_QUEENSIDE,
            Squares::H8 => CastlingAvailability::BLACK_KINGSIDE,
            Squares::E8 => {
                CastlingAvailability::BLACK_QUEENSIDE | CastlingAvailability::BLACK_KINGSIDE
            }
            _ => 0,
        },
        _ => panic!("Invalid piece"),
    }
}
