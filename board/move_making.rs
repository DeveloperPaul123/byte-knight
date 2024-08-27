use crate::{board::Board, definitions::Side, moves::Move, pieces::Piece};

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

        if captured_piece.is_some() {
            let cap = captured_piece.unwrap();
            // remove the captured piece
            self.remove_piece(them, cap, to);
            // reset half move clock
            self.set_half_move_clock(0);
            //check for need to update castling rights
            if cap == Piece::Rook {
                // TODO: determine what castling rights need to be updated
            }
        }

        if piece == Piece::Pawn {
            self.remove_piece(us, piece, from);
            // take into account the promotion piece if any
            let piece_to_add = if mv.is_promotion() {
                mv.promotion_piece().unwrap()
            } else {
                piece
            };
            self.add_piece(us, piece_to_add, to);

            if mv.is_en_passant_capture() {
                // remove the captured pawn
                // TODO: Is ^8 right?
                self.remove_piece(them, Piece::Pawn, to ^ 8);
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
            self.move_piece(us, piece, from, to)
        }

        // todo:update the castling rights
        if can_castle && (piece == Piece::King || piece == Piece::Rook) {
            // TODO: update castling rights
        }

        if mv.is_castle() {
            // TODO: Handle castling
        }

        self.switch_side();

        if us == Side::Black {
            self.set_full_move_number(self.half_move_clock() + 1);
        }

        // pseudo legal check
        // check if we are in check
        // TODO: Implement this

        return Ok(());
    }

    pub fn unmake_move(&mut self) -> Result<(), &'static str> {
        let maybe_state = self.history.pop();
        if maybe_state.is_none() {
            return Err("No move to unmake");
        }

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
        if captured_piece.is_some() {
            // TODO: Restore the captured piece to the board
            todo!("Implement this");
        }

        // we're going to assume that the move is valid, and that it was the last move made
        todo!("Implement this. Need a move history to pop from");
    }

    /// Add a piece to the board for a given side and square. Will also update the zobrist hash.
    fn add_piece(&mut self, side: Side, piece: Piece, square: u8) {
        let bb = self.mut_piece_bitboard(piece, side);
        bb.set_square(square as usize);
        self.update_zobrist_hash_for_piece(square, piece, side)
    }

    /// Remove a piece from the board for a given side and square. Will also update the zobrist hash.
    fn remove_piece(&mut self, side: Side, piece: Piece, square: u8) {
        let bb = self.mut_piece_bitboard(piece, side);
        bb.clear_square(square as usize);
        self.update_zobrist_hash_for_piece(square, piece, side)
    }

    /// Move a piece from one square to another. Will update the zobrist hash for the
    ///  removal and addition of the piece for the to and from square.
    fn move_piece(&mut self, side: Side, piece: Piece, from: u8, to: u8) {
        self.remove_piece(side, piece, from);
        self.add_piece(side, piece, to);
    }

    fn switch_side(&mut self) {
        self.set_side_to_move(Side::opposite(self.side_to_move()));
    }
}
