use crate::move_generation::NORTH;
use crate::move_generation::RANK_BITBOARDS;
use crate::move_generation::SOUTH;
use crate::move_list::MoveList;
use crate::square;
use crate::{
    bitboard::Bitboard,
    bitboard_helpers,
    board::Board,
    definitions::Squares,
    move_generation::MoveGenerator,
    pieces::{Piece, SLIDER_PIECES},
    rank::Rank,
    side::Side,
    square::Square,
};

impl MoveGenerator {
    /// Calculate 'checkers' and 'pinned' bitboard masks for the current position.
    ///
    /// # Arguments
    /// - board - The current board state
    ///
    /// # Returns
    ///
    /// A tuple containing the 'checkers' and 'pinned' bitboards in that order.
    /// Checkers are the squares that are attacking the king, and pinned are squares/pieces that are pinned.
    fn calculate_checkers_and_pinned_masks(
        &self,
        board: &Board,
        occupancy: &Bitboard,
    ) -> (Bitboard, Bitboard, Bitboard) {
        let us = board.side_to_move();
        let them = Side::opposite(us);
        let our_pieces = board.pieces(us);
        let king_bb = board.piece_bitboard(Piece::King, us);
        let king_square = bitboard_helpers::next_bit(&mut king_bb.clone()) as u8;

        // 1. Calculate opponent sliding piece moves to our king square
        // 2. Calculate sliding piece moves from our king position
        // 3. Calculate "pinned rays" by overlapping 1 and 2
        // 4. Calculate pinned pieces by overlapping 3 and our pieces
        let mut opponent_slider_attacks = Bitboard::default();
        let their_pieces = board.pieces(them);
        for piece in SLIDER_PIECES.iter() {
            let mut piece_bb = board.piece_bitboard(*piece, them).clone();

            while piece_bb.as_number() > 0 {
                let slider_sq = bitboard_helpers::next_bit(&mut piece_bb) as u8;
                let slider_attacks = self.get_slider_attacks(*piece, slider_sq, &their_pieces);
                opponent_slider_attacks |= slider_attacks;
            }
        }

        let mut from_our_king_attacks = Bitboard::default();
        let mut pinners = Bitboard::default();
        for piece in SLIDER_PIECES.iter() {
            let piece_bb = board.piece_bitboard(*piece, them);
            let attacks = self.get_slider_attacks(*piece, king_square, &their_pieces);

            // does our piece interserct with the attacks?
            if attacks.intersects(*piece_bb) {
                // we have an intersection, but we may have multiple pieces on the enemy side that intersect
                // we need to now identify the actual pinners
                let mut p_bb = attacks & *piece_bb;

                while p_bb.as_number() > 0 {
                    let sq = bitboard_helpers::next_bit(&mut p_bb) as u8;
                    let ray = self.ray_between(
                        Square::from_square_index(king_square),
                        Square::from_square_index(sq),
                    );

                    let ray_piece_intersection = ray & our_pieces;
                    if ray_piece_intersection.as_number().count_ones() == 1 {
                        // only 1 piece between the king and the attacker, so it's pinned
                        // if there are multiple pieces, then it's not pinned
                        from_our_king_attacks |= ray;
                        pinners |= Bitboard::from_square(sq);
                    }
                }
            }
        }

        let pin_rays = (opponent_slider_attacks & from_our_king_attacks) | pinners;
        let pinned_pieces = pin_rays & our_pieces;

        // ensure we definitely don't have the king in the occupancy
        let kingless_occupancy = *occupancy & !(*king_bb);
        // an enemy king cannot check our king, so we ignore it
        let knight_attacks = self.get_non_slider_attacks(Piece::Knight, king_square);
        let rook_attacks = self.get_slider_attacks(Piece::Rook, king_square, &kingless_occupancy);
        let bishop_attacks =
            self.get_slider_attacks(Piece::Bishop, king_square, &kingless_occupancy);
        let queen_attacks = rook_attacks | bishop_attacks;
        // note we use the opposite side for the pawn attacks
        let pawn_attacks = self.pawn_attacks[Side::opposite(them) as usize][king_square as usize];

        let enemy_pawns = board.piece_bitboard(Piece::Pawn, them);
        let enemy_knights = board.piece_bitboard(Piece::Knight, them);
        let enemy_bishops = board.piece_bitboard(Piece::Bishop, them);
        let enemy_rooks = board.piece_bitboard(Piece::Rook, them);
        let enemy_queens = board.piece_bitboard(Piece::Queen, them);

        // calculate our checkers bb
        let checkers = knight_attacks & *enemy_knights
            | rook_attacks & *enemy_rooks
            | bishop_attacks & *enemy_bishops
            | queen_attacks & *enemy_queens
            | pawn_attacks & *enemy_pawns;

        (checkers, pinned_pieces, pin_rays)
    }

    /// Calculate the capture and push masks for the king in the current position
    ///
    /// # Arguments
    ///
    /// - board - The current board state
    /// - checkers - The squares that are attacking the king. See [calculate_checkers_and_pinned_mask][MoveGenerator::calculate_checkers_and_pinned_masks] for more.
    ///
    /// # Returns
    ///
    /// A tuple containing the capture and push masks in that order.
    fn calculate_capture_and_push_masks(
        &self,
        board: &Board,
        checkers: &Bitboard,
    ) -> (Bitboard, Bitboard) {
        let them = Side::opposite(board.side_to_move());
        let king_bb = board.piece_bitboard(Piece::King, board.side_to_move());
        let king_square = bitboard_helpers::next_bit(&mut king_bb.clone()) as u8;
        // initialize these to all squares on the board by default
        let mut capture_mask = Bitboard::new(u64::MAX);
        let mut push_mask = Bitboard::new(u64::MAX);

        // now we generate moves for the king
        let num_checkers = checkers.as_number().count_ones();
        if num_checkers == 1 {
            // if there is only one checker, we can capture it or move the king
            capture_mask = *checkers;

            let checker_sq = bitboard_helpers::next_bit(&mut checkers.clone()) as u8;
            if let Some((piece, side)) = board.piece_on_square(checker_sq) {
                debug_assert!(side == them);
                // check if the piece is a slider
                if piece.is_slider() {
                    // if the piece is a slider, we can evade check by blocking it, so moving (push)
                    // a piece into the ray between the checker and the king is valid
                    push_mask = self.ray_between(
                        Square::from_square_index(king_square),
                        Square::from_square_index(checker_sq),
                    );
                } else {
                    // we can only capture, so push mask needs to be empty (0)
                    push_mask = Bitboard::default();
                }
            }
        }

        // Add the enpassant square to our capture mask if it's viable
        let en_passant_bb = board
            .en_passant_square()
            .map(|sq| Bitboard::from(sq))
            .unwrap_or_default();
        // we need to first check if the en passant square would capture a checker
        // if "us" is white, then we whould shift the en passant square left
        // if "us" is black, then we should shift the en passant square right
        match board.side_to_move() {
            Side::White => {
                let left = en_passant_bb >> SOUTH;
                if left & *checkers != 0 {
                    capture_mask |= en_passant_bb;
                }
            }
            Side::Black => {
                let right = en_passant_bb << NORTH;
                if right & *checkers != 0 {
                    capture_mask |= en_passant_bb;
                }
            }
            Side::Both => panic!("Both side not allowed"),
        }

        (capture_mask, push_mask)
    }

    fn calculate_en_passant_bitboard(&self, from: u8, board: &Board) -> Bitboard {
        let en_passant_sq = board.en_passant_square();

        match en_passant_sq {
            Some(sq) => {
                let en_passant_bb = en_passant_sq
                    .map(|sq| Bitboard::from(sq))
                    .unwrap_or_default();

                // check for discovered checks
                // remove the captured and capturing pawns from the bitboard
                let mut occupancy = board.all_pieces();
                // remove the attacking pawn
                occupancy &= !(Bitboard::from_square(from));
                // remove the captured pawn
                let captured_sq = match board.side_to_move() {
                    Side::White => sq - SOUTH as u8,
                    Side::Black => sq + NORTH as u8,
                    Side::Both => panic!("Both side not allowed"),
                };
                occupancy &= !(Bitboard::from_square(captured_sq));
                // get the squares attacked by the sliding pieces
                let (mut checkers, _, _) =
                    self.calculate_checkers_and_pinned_masks(board, &occupancy);
                // filter checkers to the same rank as the king
                let king_sq = bitboard_helpers::next_bit(
                    &mut board
                        .piece_bitboard(Piece::King, board.side_to_move())
                        .clone(),
                ) as u8;
                let king_rank = square::from_square(king_sq).1;
                checkers &= RANK_BITBOARDS[king_rank as usize];
                if checkers.number_of_occupied_squares() == 0 {
                    // we are now in check, so we cannot use this en passant capture
                    en_passant_bb
                } else {
                    Bitboard::default()
                }
            }
            None => Bitboard::default(),
        }
    }

    fn generate_legal_pawn_mobility(
        &self,
        board: &Board,
        square: &Square,
        pinned_mask: &Bitboard,
        capture_mask: &Bitboard,
        push_mask: &Bitboard,
        pin_rays: &Bitboard,
    ) -> Bitboard {
        // pawns can get complex because of en passant and promotion
        // also, we need to take into account the pin direction
        // are we pinned?
        let is_pinned = pinned_mask.intersects(*square);
        let us = board.side_to_move();
        let their_pieces = board.pieces(Side::opposite(us));
        let direction = match us {
            Side::White => NORTH as u8,
            Side::Black => SOUTH as u8,
            Side::Both => panic!("Both side not allowed"),
        };
        let from_square = square.to_square_index();

        let to_square = match us {
            Side::White => {
                let (result, did_overflow) = from_square.overflowing_add(direction);
                match did_overflow {
                    true => None,
                    false => Some(result),
                }
            }
            Side::Black => {
                let (result, did_overflow) = from_square.overflowing_sub(direction);
                match did_overflow {
                    true => None,
                    false => Some(result),
                }
            }
            Side::Both => panic!("Both side not allowed"),
        };

        let mut pushes: Bitboard = match to_square {
            Some(to) => Bitboard::from_square(to),
            None => Bitboard::default(),
        };

        let occupancy = board.all_pieces();
        let is_unobstructed = pushes & !occupancy == Bitboard::default();

        let can_double_push = match us {
            Side::White => Board::is_square_on_rank(from_square, Rank::R2 as u8),
            Side::Black => Board::is_square_on_rank(from_square, Rank::R7 as u8),
            Side::Both => panic!("Both side not allowed"),
        };

        // if single push is obstructed, we can't double push
        if can_double_push && !is_unobstructed {
            let double_push_sq = match us {
                Side::White => {
                    let (result, did_overflow) = from_square.overflowing_add(2 * NORTH as u8);
                    match did_overflow {
                        true => None,
                        false => Some(result),
                    }
                }
                Side::Black => {
                    let (result, did_overflow) = from_square.overflowing_sub(2 * SOUTH as u8);
                    match did_overflow {
                        true => None,
                        false => Some(result),
                    }
                }
                Side::Both => panic!("Both side not allowed"),
            };

            match double_push_sq {
                Some(to) => {
                    let bb = Bitboard::from_square(to);
                    // append to our pushes
                    pushes |= bb;
                }
                None => {}
            }
        }

        let pawn_pin_mask = if is_pinned {
            *pin_rays
        } else {
            Bitboard::from(u64::MAX)
        };

        let en_passant_bb = self.calculate_en_passant_bitboard(from_square, board);

        // filter pushes by the occupany
        let legal_pushes = pushes & !occupancy;
        let attacks = self.pawn_attacks[us as usize][square.to_square_index() as usize]
            & (their_pieces | en_passant_bb);
        (legal_pushes | attacks) & (*capture_mask | *push_mask) & pawn_pin_mask
    }

    fn generate_normal_piece_legal_mobility(
        &self,
        piece: Piece,
        square: &Square,
        board: &Board,
        capture_mask: &Bitboard,
        pinned_mask: &Bitboard,
        push_mask: &Bitboard,
        pin_rays: &Bitboard,
    ) -> Bitboard {
        let is_pinned = pinned_mask.intersects(*square);
        let us = board.side_to_move();
        let their_pieces = board.pieces(Side::opposite(us));
        let from_square = square.to_square_index();
        let mut mobility = Bitboard::default();

        let attacks = match piece {
            Piece::Knight => self.knight_attacks[from_square as usize],
            Piece::Bishop => self.get_slider_attacks(piece, from_square, &board.all_pieces()),
            Piece::Rook => self.get_slider_attacks(piece, from_square, &board.all_pieces()),
            Piece::Queen => self.get_slider_attacks(piece, from_square, &board.all_pieces()),
            _ => panic!("Invalid piece for normal piece mobility"),
        };

        // we need to further filter out moves and ensure they are along the correct pin ray

        let our_pieces = board.pieces(us);
        let empty = !(their_pieces | our_pieces);

        // empty pushes or captures masked by capture mask and push mask in the case that we are
        // single checked
        // note: everything gets masked by the pin ray mask

        // calculate the correct pin ray mask, only using pin rays that intersect with the piece
        // and the king
        let pin_ray_mask = if is_pinned {
            let king_sq = board.king_square(us);

            // at least one of our moves intersects with the pin ray, so we're attacking the pinner or moving along the pin ray
            // we need to ensure that we move along the correct pin ray
            let mut pinners = their_pieces & *pin_rays;
            let piece_bb = Bitboard::from_square(square.to_square_index() as u8);
            let mut true_ray_mask = Bitboard::default();

            while pinners.as_number() > 0 {
                let pinner_sq = bitboard_helpers::next_bit(&mut pinners) as u8;
                let ray = self.ray_between(
                    Square::from_square_index(pinner_sq),
                    Square::from_square_index(king_sq),
                );

                if ray.intersects(piece_bb) {
                    // the ray intersects with our piece,
                    true_ray_mask |= ray | Bitboard::from_square(pinner_sq);
                }
            }

            true_ray_mask
        } else {
            Bitboard::from(u64::MAX)
        };

        mobility = ((attacks & *capture_mask & their_pieces) | (attacks & empty & *push_mask))
            & pin_ray_mask;

        mobility
    }

    fn generate_legal_castling_mobility(
        &self,
        square: &Square,
        board: &Board,
        attacked_squares: &Bitboard,
        checkers: &Bitboard,
    ) -> Bitboard {
        /*
         * For castling, the king and rook must not have moved.
         * The squares between the king and rook must be empty.
         * The squares the king moves through must not be under attack (including start and end).
         * The king must not be in check.
         * The king must not move through check.
         * The king must not end up in check.
         *
         * FIDE Laws of Chess:
         * 3.8.2.1 The right to castle has been lost:
         *     3.8.2.1.1 if the king has already moved, or
         *     3.8.2.1.2 with a rook that has already moved.
         *
         * 3.8.2.2 Castling is prevented temporarily:
         *     3.8.2.2.1 if the square on which the king stands, or the square which it must cross, or the square which it is to occupy, is attacked by one or more of the opponent's pieces, or
         *     3.8.2.2.2 if there is any piece between the king and the rook with which castling is to be effected.
         */

        // we cannot castle when in check
        let in_check = checkers.number_of_occupied_squares() > 0;
        if in_check {
            return Bitboard::default();
        }

        let us = board.side_to_move();
        let occupancy = board.all_pieces();
        let mut castling_moves = Bitboard::default();
        let king_side_castle = board.can_castle_kingside(us);
        let queen_side_castle = board.can_castle_queenside(us);

        let king_sq = match us {
            Side::White => Squares::E1,
            Side::Black => Squares::E8,
            Side::Both => panic!("Both side not allowed"),
        };

        // sanity check
        let king_in_place = king_sq == square.to_square_index();
        if !king_in_place {
            return Bitboard::default();
        }

        if king_side_castle {
            let king_side_rook = match us {
                Side::White => Squares::H1,
                Side::Black => Squares::H8,
                Side::Both => panic!("Both side not allowed"),
            };
            // sanity check for the rook placement
            let maybe_rook = board.piece_on_square(king_side_rook);
            let rook_in_place = match maybe_rook {
                Some((Piece::Rook, side)) => side == us,
                _ => false,
            };

            let king_side_empty = match us {
                Side::White => {
                    Bitboard::from_square(Squares::F1) | Bitboard::from_square(Squares::G1)
                }
                Side::Black => {
                    Bitboard::from_square(Squares::F8) | Bitboard::from_square(Squares::G8)
                }
                Side::Both => panic!("Both side not allowed"),
            };

            let king_side_target_sq = match us {
                Side::White => Squares::G1,
                Side::Black => Squares::G8,
                Side::Both => panic!("Both side not allowed"),
            };

            let is_king_ray_empty = king_side_empty & occupancy == Bitboard::default();
            let is_king_ray_attacked = king_side_empty & *attacked_squares != Bitboard::default();
            if is_king_ray_empty && !is_king_ray_attacked && rook_in_place && king_in_place {
                castling_moves |= Bitboard::from_square(king_side_target_sq);
            }
        }

        if queen_side_castle {
            let queen_side_rook = match us {
                Side::White => Squares::A1,
                Side::Black => Squares::A8,
                Side::Both => panic!("Both side not allowed"),
            };
            // sanity check for the rook placement
            let maybe_rook = board.piece_on_square(queen_side_rook);
            let rook_in_place = match maybe_rook {
                Some((Piece::Rook, side)) => side == us,
                _ => false,
            };

            let queen_side_no_attack = match us {
                Side::White => {
                    Bitboard::from_square(Squares::C1) | Bitboard::from_square(Squares::D1)
                }
                Side::Black => {
                    Bitboard::from_square(Squares::C8) | Bitboard::from_square(Squares::D8)
                }
                Side::Both => panic!("Both side not allowed"),
            };
            let queen_side_empty = match us {
                Side::White => queen_side_no_attack | Bitboard::from_square(Squares::B1),
                Side::Black => queen_side_no_attack | Bitboard::from_square(Squares::B8),
                Side::Both => panic!("Both side not allowed"),
            };

            let queen_side_target_sq = match us {
                Side::White => Squares::C1,
                Side::Black => Squares::C8,
                Side::Both => panic!("Both side not allowed"),
            };

            let is_king_ray_empty = queen_side_empty & occupancy == Bitboard::default();
            let is_king_ray_attacked =
                queen_side_no_attack & *attacked_squares != Bitboard::default();
            if is_king_ray_empty && !is_king_ray_attacked && rook_in_place && king_in_place {
                castling_moves |= Bitboard::from_square(queen_side_target_sq);
            }
        }
        castling_moves
    }

    fn generate_king_legal_mobility(
        &self,
        square: &Square,
        board: &Board,
        capture_mask: &Bitboard,
        push_mask: &Bitboard,
        checkers: &Bitboard,
    ) -> Bitboard {
        let us = board.side_to_move();
        let them = Side::opposite(us);
        let our_pieces = board.pieces(us);
        let their_pieces = board.pieces(them);
        let occupancy = our_pieces | their_pieces;

        let king_bb = board.piece_bitboard(Piece::King, us);

        // generate king moves
        // calculate attacked squares
        let king_moves_bb = self.get_non_slider_attacks(Piece::King, square.to_square_index());

        // remove the king from the attacked squares occupancy
        let attacked_squares_occupancy = occupancy & !*king_bb;
        let attacked_squares = self.get_attacked_squares(board, them, &attacked_squares_occupancy);
        let king_pushes = king_moves_bb & !attacked_squares & !our_pieces & !their_pieces;

        // also add castling if possible
        let castling_moves =
            self.generate_legal_castling_mobility(square, board, &attacked_squares, checkers);

        let king_non_checker_attacks =
            (king_moves_bb & their_pieces & !*checkers) & !*push_mask & !attacked_squares;
        let mut king_attacks = (king_moves_bb & *capture_mask & their_pieces & !attacked_squares)
            | king_non_checker_attacks;

        // do any of our attacks put us in check?
        let mut k_att = king_attacks.clone();
        while k_att.as_number() > 0 {
            let capture_sq = bitboard_helpers::next_bit(&mut k_att) as u8;
            // remove the piece we're capturing and the king from the occupancy
            let modified_occupancy = occupancy & !Bitboard::from_square(capture_sq) & !*king_bb;
            let is_invalid_capture = self.is_square_attacked_with_occupancy(
                board,
                &Square::from_square_index(capture_sq),
                them,
                &modified_occupancy,
            );
            if is_invalid_capture {
                // if the capture puts us in check, remove it
                king_attacks &= !Bitboard::from_square(capture_sq);
            }
        }

        king_pushes | king_attacks | castling_moves
    }

    fn generate_legal_mobility(
        &self,
        piece: Piece,
        square: &Square,
        board: &Board,
        pinned_mask: &Bitboard,
        capture_mask: &Bitboard,
        push_mask: &Bitboard,
        pin_rays: &Bitboard,
        checkers: &Bitboard,
    ) -> Bitboard {
        match piece {
            Piece::Pawn => self.generate_legal_pawn_mobility(
                board,
                square,
                pinned_mask,
                capture_mask,
                push_mask,
                pin_rays,
            ),
            Piece::King => {
                self.generate_king_legal_mobility(square, board, capture_mask, push_mask, checkers)
            }
            _ => self.generate_normal_piece_legal_mobility(
                piece,
                square,
                board,
                capture_mask,
                pinned_mask,
                push_mask,
                pin_rays,
            ),
        }
    }
    pub fn generate_legal_moves(&self, board: &Board, move_list: &mut MoveList) {
        let us = board.side_to_move();
        let them = Side::opposite(us);
        let our_pieces = board.pieces(us);
        let their_pieces = board.pieces(them);
        let occupancy = our_pieces | their_pieces;

        let king_bb = board.piece_bitboard(Piece::King, us);
        let king_square = bitboard_helpers::next_bit(&mut king_bb.clone()) as u8;

        let (checkers, pinned, pin_rays) =
            self.calculate_checkers_and_pinned_masks(board, &occupancy);

        let (capture_mask, push_mask) = self.calculate_capture_and_push_masks(board, &checkers);

        let king_sq = Square::from_square_index(king_square);
        let king_moves = self.generate_king_legal_mobility(
            &king_sq,
            board,
            &capture_mask,
            &push_mask,
            &checkers,
        );

        self.enumerate_moves(&king_moves, &king_sq, Piece::King, board, move_list);

        let num_checkers = checkers.as_number().count_ones();
        if num_checkers > 1 {
            // if there are multiple checkers, the king must move
            return;
        }

        // now we generate moves for the rest of the pieces
        let mut moveable_pieces = our_pieces & !(*king_bb);
        while moveable_pieces.as_number() > 0 {
            // Generate moves for each piece
            let from = bitboard_helpers::next_bit(&mut moveable_pieces) as u8;
            let piece = match board.piece_on_square(from) {
                Some((piece, _)) => piece,
                None => continue,
            };

            let from_square = Square::from_square_index(from);
            // calculate our legal mobility
            let moves = self.generate_legal_mobility(
                piece,
                &from_square,
                board,
                &pinned,
                &capture_mask,
                &push_mask,
                &pin_rays,
                &checkers,
            );

            // enumerate the moves and add them to the move list
            self.enumerate_moves(&moves, &from_square, piece, board, move_list);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_pinned_pieces() {
        let move_gen = MoveGenerator::new();
        let board =
            Board::from_fen("2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2")
                .unwrap();
        let occupancy = board.all_pieces();
        let (checkers, pinned, _) =
            move_gen.calculate_checkers_and_pinned_masks(&board, &occupancy);
        assert_eq!(checkers, 0);
        assert_eq!(pinned, Bitboard::from_square(Squares::D7));
    }

    #[test]
    fn calculate_pinned_pieces_2() {
        let move_gen = MoveGenerator::new();
        let board = Board::from_fen("8/8/8/8/k2Pp2Q/8/8/3K4 b - d3 0 1").unwrap();
        let occupancy = board.all_pieces();
        let (checkers, pinned, _) =
            move_gen.calculate_checkers_and_pinned_masks(&board, &occupancy);
        assert_eq!(checkers, 0);
        assert_eq!(pinned, Bitboard::default());
    }

    #[test]
    fn calculate_pinned_pieces_3() {
        let move_gen = MoveGenerator::new();
        let board =
            Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQKR2 b Q - 2 8").unwrap();

        let occupancy = board.all_pieces();
        let (checkers, pinned, pin_rays) =
            move_gen.calculate_checkers_and_pinned_masks(&board, &occupancy);
        assert_eq!(checkers, 0);
        assert_eq!(pinned, 0);
        assert_eq!(pin_rays, 0);
    }

    #[test]
    fn check_pinned_and_capture_mask() {
        let move_generation = MoveGenerator::new();
        let board =
            Board::from_fen("rnQq1k1r/pp2bppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R b KQ - 0 8").unwrap();
        let occupancy = board.all_pieces();
        let (checkers, pinned, pin_rays) =
            move_generation.calculate_checkers_and_pinned_masks(&board, &occupancy);
        let (capture_mask, push_mask) =
            move_generation.calculate_capture_and_push_masks(&board, &checkers);
        assert_eq!(checkers, 0);
        assert_eq!(pinned, Bitboard::from_square(Squares::D8));
    }

    #[test]
    fn en_passant_capture_causes_discovered_check() {
        let move_gen = MoveGenerator::new();
        let board = Board::from_fen("8/8/8/8/k2Pp2Q/8/8/3K4 b - d3 0 1").unwrap();
        let mut move_list = MoveList::new();
        move_gen.generate_legal_moves(&board, &mut move_list);

        for mv in move_list.iter() {
            println!("{}", mv);
        }

        assert_eq!(move_list.len(), 6);
    }

    #[test]
    fn king_cannot_move_away_from_slider() {
        let move_gen = MoveGenerator::new();
        let board = Board::from_fen("4k3/8/8/8/4R3/8/8/4K3 b - - 0 1").unwrap();

        let mut move_list = MoveList::new();
        move_gen.generate_legal_moves(&board, &mut move_list);
        assert_eq!(move_list.len(), 4);
    }

    #[test]
    fn king_cannot_slide_away_from_bishop() {
        let move_gen = MoveGenerator::new();
        let board = Board::from_fen("r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2").unwrap();

        let mut move_list = MoveList::new();
        move_gen.generate_legal_moves(&board, &mut move_list);
        assert_eq!(move_list.len(), 8);
    }

    #[test]
    fn evade_check_with_en_passant_capture() {
        let move_gen = MoveGenerator::new();
        let board = Board::from_fen("8/8/8/2k5/3Pp3/8/8/4K3 b - d3 0 1").unwrap();
        let mut move_list = MoveList::new();
        move_gen.generate_legal_moves(&board, &mut move_list);

        for mv in move_list.iter() {
            println!("{}", mv);
        }

        assert_eq!(move_list.len(), 9);
    }

    #[test]
    fn rays_between_verification() {
        let move_gen = MoveGenerator::new();
        let from = Square::from_square_index(Squares::A1);
        let to = Square::from_square_index(Squares::H8);
        let rays = move_gen.ray_between(from, to);

        let expected = Bitboard::from_square(Squares::B2)
            | Bitboard::from_square(Squares::C3)
            | Bitboard::from_square(Squares::D4)
            | Bitboard::from_square(Squares::E5)
            | Bitboard::from_square(Squares::F6)
            | Bitboard::from_square(Squares::G7);
        println!("{}", rays);
        assert_eq!(rays, expected);

        let from = Square::from_square_index(Squares::H1);
        let to = Square::from_square_index(Squares::A8);
        let rays = move_gen.ray_between(from, to);

        let expected = Bitboard::from_square(Squares::G2)
            | Bitboard::from_square(Squares::F3)
            | Bitboard::from_square(Squares::E4)
            | Bitboard::from_square(Squares::D5)
            | Bitboard::from_square(Squares::C6)
            | Bitboard::from_square(Squares::B7);
        assert_eq!(rays, expected);

        let rays = move_gen.ray_between(
            Square::from_square_index(Squares::A1),
            Square::from_square_index(Squares::C2),
        );
        assert!(rays == Bitboard::default());
    }
}