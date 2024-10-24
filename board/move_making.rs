/*
 * move_making.rs
 * Part of the byte-knight project
 * Created Date: Friday, August 23rd 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Fri Oct 18 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use crate::{
    bitboard_helpers,
    board::Board,
    definitions::{CastlingAvailability, Squares},
    move_generation::MoveGenerator,
    moves::{self, Move},
    pieces::{Piece, SQUARE_NAME},
    rank::Rank,
    side::Side,
    square::Square,
};
use anyhow::{bail, Result};

impl Board {
    /// Make a move using UCI notation.
    ///
    /// This function will make a move on the board using UCI notation. It will first parse the move and then try to determine
    /// the move type and other information about it. It will then make the move on the board and update the board state.
    ///
    /// # Arguments
    ///
    /// * `mv` - The move to make in UCI notation.
    ///
    ///
    pub fn make_uci_move(&mut self, mv: &str, move_gen: &MoveGenerator) -> Result<()> {
        println!("Making UCI move: {}", mv);
        if mv.len() < 4 {
            bail!("Invalid move length");
        }

        // parse the move to and from squares
        // also check if this is a promotion if there is a promotion piece at the end of the move
        let from_str = &mv[0..2];
        let to_str = &mv[2..4];
        println!("From: {}, To: {}", from_str, to_str);
        let from =
            Square::try_from(&mv[0..2]).map_err(|_| anyhow::anyhow!("Invalid from square"))?;
        let to = Square::try_from(&mv[2..4]).map_err(|_| anyhow::anyhow!("Invalid to square"))?;

        let has_promotion_piece = mv.len() >= 5 && mv.chars().nth(4).unwrap() != ' ';
        let promotion_piece = if has_promotion_piece {
            let promotion_piece_char = mv.chars().nth(4).unwrap();
            Some(
                Piece::try_from(promotion_piece_char)
                    .map_err(|_| anyhow::anyhow!("Invalid promotion piece"))?,
            )
        } else {
            None
        };

        let (piece, side) = self
            .piece_on_square(from.to_square_index())
            .ok_or_else(|| anyhow::anyhow!("No piece on square"))?;
        let captured_piece = match self.piece_on_square(to.to_square_index()) {
            Some((piece, _)) => Some(piece),
            None => None,
        };

        // now just figure out the move descriptor
        // need to check if the move is a castle, en passant, promotion or a pawn two up move
        let can_double_push = piece == Piece::Pawn
            && Board::is_square_on_rank(
                from.to_square_index(),
                Rank::pawn_start_rank(side).as_number(),
            );

        let is_double_push = can_double_push
            && (from.rank.as_number() as i8).abs_diff(to.rank.as_number() as i8) == 2;
        let is_castle = piece == Piece::King && (from.file as i8).abs_diff(to.file as i8) == 2;
        let is_en_passant = piece == Piece::Pawn
            && self.en_passant_square().is_some()
            && self.en_passant_square().unwrap() == to.to_square_index();

        let states = [is_double_push, is_castle, is_en_passant];
        if states.iter().filter(|&&x| x).count() > 1 {
            bail!("Invalid move, only 1 move state can be true");
        }
        let move_desc = if is_double_push {
            moves::MoveDescriptor::PawnTwoUp
        } else if is_castle {
            moves::MoveDescriptor::Castle
        } else if is_en_passant {
            moves::MoveDescriptor::EnPassantCapture
        } else {
            moves::MoveDescriptor::None
        };

        let mv = Move::new(
            &from,
            &to,
            move_desc,
            piece,
            captured_piece,
            promotion_piece,
        );
        self.make_move(&mv, move_gen)
    }

    fn check_move_preconditions(&mut self, mv: &Move) -> Result<()> {
        let from = mv.from();
        let to: u8 = mv.to();
        let piece = mv.piece();

        let us = self.side_to_move();
        let them = Side::opposite(us);

        let piece_and_side = self.piece_on_square(from);
        if piece_and_side.is_none() {
            bail!("No piece on square");
        }

        let (piece_on_square, side) = piece_and_side.unwrap();
        if piece_on_square != piece || side != us {
            bail!("Invalid piece on square");
        }

        if mv.captured_piece().is_some() {
            let captured_piece = mv.captured_piece().unwrap();
            let piece_and_side = self.piece_on_square(to);
            if piece_and_side.is_none() {
                bail!("No piece on square");
            }

            let (piece_on_square, side) = piece_and_side.unwrap();
            // check that the capture piece matches and is not our own
            if piece_on_square != captured_piece || side != them {
                bail!("Invalid captured piece on square");
            }

            if captured_piece == Piece::King {
                bail!("Invalid move, cannot capture king");
            }
        }

        let move_desc = mv.move_descriptor();
        match move_desc {
            moves::MoveDescriptor::EnPassantCapture => {
                if piece != Piece::Pawn {
                    bail!("Invalid en passant, not a pawn");
                }
            }
            moves::MoveDescriptor::Castle => {
                if !self.can_castle_kingside(us) && !self.can_castle_queenside(us) {
                    bail!("Tried to castle without castling rights");
                }
            }
            moves::MoveDescriptor::PawnTwoUp => {
                if piece != Piece::Pawn {
                    bail!("Invalid double pawn push, not a pawn");
                }
            }
            // We don't handle None, quiet moves are ok
            _ => {}
        }

        Ok(())
    }

    /// Make a move on the board and update the board state
    ///
    /// # Errors
    ///
    /// This function will return an error if the move is illegal. The passed in moves are assumed to be pseudo-legal,
    /// hence why the check has to be done after making the move. This function will make the move, check for legality
    /// and then undo the move if it is illegal.
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn make_move(&mut self, mv: &Move, move_gen: &MoveGenerator) -> Result<()> {
        // validate pre-conditions first before even bothering to go further
        self.check_move_preconditions(mv)?;
        
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

        // en passant capture is handled separately
        if captured_piece.is_some() && !mv.is_en_passant_capture() {
            let cap = captured_piece.unwrap();
            // remove the captured piece from the board
            self.remove_piece(them, cap, to, update_zobrist_hash);
            // reset half move clock
            self.set_half_move_clock(0);
            //check for need to update castling rights
            if cap == Piece::Rook {
                // check if the rook was on a corner square
                // if so, remove the castling rights for that side
                let corners = [
                    Squares::A8 as u8,
                    Squares::H8 as u8,
                    Squares::A1 as u8,
                    Squares::H1 as u8,
                ];
                if corners.iter().any(|sq| *sq == to) {
                    self.set_castling_rights(
                        self.castling_rights() & !(get_casting_right_to_remove(them, to)),
                    );
                }
            }
        }

        if piece == Piece::Pawn {
            // reset half move clock
            self.set_half_move_clock(0);

            self.remove_piece(us, piece, from, update_zobrist_hash);
            // take into account the promotion piece if any
            let piece_to_add = if mv.is_promotion() {
                mv.promotion_piece().unwrap()
            } else {
                piece
            };
            self.add_piece(us, piece_to_add, to, update_zobrist_hash);

            if mv.is_en_passant_capture() {
                // remove the "captured" pawn
                // this pawn should be either one rank above or below the destination square
                // depending on the side to move
                // if white, the pawn is one rank below the destination square
                // if black, the pawn is one rank above the destination square
                let en_passant_pawn_location = if us == Side::White {
                    to - 8u8
                } else {
                    to + 8u8
                };
                let pawns = self.piece_bitboard(Piece::Pawn, them);
                debug_assert!(pawns.is_square_occupied(en_passant_pawn_location));
                self.remove_piece(
                    them,
                    Piece::Pawn,
                    en_passant_pawn_location as u8,
                    update_zobrist_hash,
                );
            }
            // check if this is a double pawn push
            // if so, set the en passant square
            if mv.is_pawn_two_up() {
                // get the en passant square from the new move
                // if white, the en passant square is one rank below the destination square
                // if black, the en passant square is one rank above the destination square
                let en_passant_square = if us == Side::White {
                    to - 8u8
                } else {
                    to + 8u8
                };
                self.set_en_passant_square(Some(en_passant_square));
            } else {
                self.set_en_passant_square(None);
            }
        } else {
            // reset the en passant square if it exists
            if self.en_passant_square().is_some() {
                self.set_en_passant_square(None);
            }
            // just move the piece
            self.move_piece(us, piece, from, to, update_zobrist_hash);
            self.set_half_move_clock(self.half_move_clock() + 1);
        }

        // update the castling rights
        if can_castle && (piece == Piece::King || piece == Piece::Rook) {
            // we moved our king or rook, so we need to update the castling rights
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

        // switch side to move
        self.switch_side();

        // update full move number
        if us == Side::Black {
            self.set_full_move_number(self.half_move_clock() + 1);
        }

        // pseudo legal check
        // check if we are in check
        // get the kings location and check if that square is attacked by the opponent
        let mut king_bb = self.piece_bitboard(Piece::King, us).clone();
        let king_square = bitboard_helpers::next_bit(&mut king_bb) as u8;
        let is_king_in_check =
            move_gen.is_square_attacked(self, &Square::from_square_index(king_square), them);

        if is_king_in_check {
            self.unmake_move()?;
            bail!("Illegal move");
        }

        return Ok(());
    }

    /// Undo the last move made on this [`Board`].
    /// In general, each undo() should be preceded by a [Board::make_move()]. But this isn't a hard requirement.
    ///
    /// # Errors
    ///
    /// This function will return an error if it is unable to undo the last move. This can happen if
    /// no moves have been made on the board.
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn unmake_move(&mut self) -> Result<()> {
        let maybe_state = self.history.pop();
        if maybe_state.is_none() {
            bail!("No move to unmake");
        }

        // note that we don't update the zobrist hash here as we are
        // undoing moves because it gets restored from the game state
        let update_zobrist_hash = false;
        // restore the board state
        let state = maybe_state.unwrap();
        self.set_board_state(state);

        let us = self.side_to_move();
        let them = Side::opposite(us);
        // this is move that we're unmaking
        let chess_move = state.next_move;

        // handle null moves
        if chess_move.is_null_move() {
            //nothing else to undo...
            return Ok(());
        }

        let from = chess_move.from();
        let to = chess_move.to();
        let piece = chess_move.piece();
        let captured_piece = chess_move.captured_piece();
        let promoted_piece = chess_move.promotion_piece();
        if promoted_piece.is_some() {
            // remove the promoted piece
            // note that we don't update the zobrist hash here
            self.remove_piece(us, promoted_piece.unwrap(), to, update_zobrist_hash);
            // put the pawn back
            self.add_piece(us, Piece::Pawn, from, update_zobrist_hash);
        } else {
            self.undo_move(us, piece, from, to, update_zobrist_hash);
        }

        if chess_move.is_castle() {
            // also need to move the rook back
            let (rook_from, rook_to) = match to {
                Squares::G1 => (Squares::H1, Squares::F1),
                Squares::C1 => (Squares::A1, Squares::D1),
                Squares::G8 => (Squares::H8, Squares::F8),
                Squares::C8 => (Squares::A8, Squares::D8),
                _ => panic!("Invalid castling move"),
            };

            self.undo_move(us, Piece::Rook, rook_from, rook_to, update_zobrist_hash);
            // we don't need to update the castling rights here as it is restored from the game state
        }

        if captured_piece.is_some() && !chess_move.is_en_passant_capture() {
            // Restore the captured piece to the board
            self.add_piece(them, captured_piece.unwrap(), to, update_zobrist_hash);
        } else if chess_move.is_en_passant_capture() {
            let en_passant_square: u8 = if us == Side::White {
                to - 8u8
            } else {
                to + 8u8
            };
            self.add_piece(them, Piece::Pawn, en_passant_square, update_zobrist_hash);
            // we don't need to set the en passant square here as it is restored from the game state
        }

        return Ok(());
    }

    pub fn null_move(&mut self) {
        let mut current_state = *self.board_state();
        current_state.next_move = Move::default();
        // update history before modifying the current state
        self.history.push(current_state);

        self.switch_side();
    }

    fn undo_move(&mut self, side: Side, piece: Piece, from: u8, to: u8, update_zobrist_hash: bool) {
        self.remove_piece(side, piece, to, update_zobrist_hash);
        self.add_piece(side, piece, from, update_zobrist_hash);
    }

    /// Add a piece to the board for a given side and square.
    ///
    /// # Arguments
    ///
    /// * `side` - The side to add the piece for.
    /// * `piece` - The piece to add.
    /// * `square` - The square to add the piece to.
    /// * `update_zobrist_hash` - Whether to update the zobrist hash for the addition of the piece.
    fn add_piece(&mut self, side: Side, piece: Piece, square: u8, update_zobrist_hash: bool) {
        let bb = self.mut_piece_bitboard(piece, side);
        bb.set_square(square);
        if update_zobrist_hash {
            self.update_zobrist_hash_for_piece(square, piece, side)
        }
    }

    /// Remove a piece from the board for a given side and square.
    ///
    /// # Arguments
    ///
    /// * `side` - The side to remove the piece for.
    /// * `piece` - The piece to remove.
    /// * `square` - The square to remove the piece from.
    /// * `update_zobrist_hash` - Whether to update the zobrist hash for the removal of the piece.
    fn remove_piece(&mut self, side: Side, piece: Piece, square: u8, update_zobrist_hash: bool) {
        let bb = self.mut_piece_bitboard(piece, side);
        if !bb.is_square_occupied(square) {
            println!(
                "square {} not occupied by {}\n{}",
                SQUARE_NAME[square as usize], piece, bb
            )
        }
        debug_assert!(bb.is_square_occupied(square));
        bb.clear_square(square);
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
