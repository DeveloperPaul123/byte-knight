use crate::{board::Board, definitions::Side, moves::Move, pieces::Piece};

impl Board {
    pub fn make_move(&mut self, mv: &Move) -> Result<(), &'static str> {
        let from = mv.from();
        let to = mv.to();
        let maybe_piece = self.piece_on_square(from as usize);
        if maybe_piece.is_none() {
            return Err("No piece on square");
        }

        let piece_info = maybe_piece.unwrap();
        let piece = piece_info.0;
        let side = piece_info.1;

        let us = side;
        let them = Side::opposite(us);

        let captured_piece = self.piece_on_square(to as usize);
        let is_capture = captured_piece.is_some();
        let can_castle = self.castling_rights() > 0;

        if captured_piece.is_some() {
            // remove the captured piece

            self.remove_piece(them, captured_piece.unwrap().0, to);
            // update the piece list
        }

        if piece == Piece::Pawn {
            // check if this is a double pawn push
            // if so, set the en passant square
        } else {
            // just move the piece
            self.move_piece(us, piece, from, to)
        }

        // update the castling rights

        // TODO: keep some sort of history
        return Ok(());
    }

    pub fn unmake_move(mv: &Move) {
        todo!("Implement unmake_move")
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
