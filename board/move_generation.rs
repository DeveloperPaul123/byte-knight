/*
 * move_generation.rs
 * Part of the byte-knight project
 * Created Date: Wednesday, August 28th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Wed Oct 02 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use rand_chacha::rand_core::block;

use crate::{
    bitboard::Bitboard,
    bitboard_helpers,
    board::Board,
    definitions::{
        File, NumberOf, Rank, Side, Squares, BISHOP_BLOCKER_PERMUTATIONS, ROOK_BLOCKER_PERMUTATIONS,
    },
    magics::{MagicNumber, BISHOP_MAGIC_VALUES, ROOK_MAGIC_VALUES},
    move_list::MoveList,
    moves::{Flags, Move, MoveType},
    pieces::{Piece, SQUARE_NAME},
    square::{self, Square},
};

type FileBitboards = [Bitboard; NumberOf::FILES];
type RankBitboards = [Bitboard; NumberOf::RANKS];

const FILE_BITBOARDS: FileBitboards = [
    Bitboard::new(72340172838076673),
    Bitboard::new(144680345676153346),
    Bitboard::new(289360691352306692),
    Bitboard::new(578721382704613384),
    Bitboard::new(1157442765409226768),
    Bitboard::new(2314885530818453536),
    Bitboard::new(4629771061636907072),
    Bitboard::new(9259542123273814144),
];
const RANK_BITBOARDS: RankBitboards = [
    Bitboard::new(255),
    Bitboard::new(65280),
    Bitboard::new(16711680),
    Bitboard::new(4278190080),
    Bitboard::new(1095216660480),
    Bitboard::new(280375465082880),
    Bitboard::new(71776119061217280),
    Bitboard::new(18374686479671623680),
];

const NORTH: u64 = 8;
const SOUTH: u64 = 8;

const WEST: u64 = 1;
const EAST: u64 = 1;
const NORTH_EAST: u64 = 9;
const NORTH_WEST: u64 = 7;
const SOUTH_EAST: u64 = 7;
const SOUTH_WEST: u64 = 9;
const NORTH_NORTH_EAST: u64 = 17;
const WEST_NORTH_WEST: u64 = 6;
const NORTH_NORTH_WEST: u64 = 15;
const EAST_NORTH_EAST: u64 = 10;
const SOUTH_SOUTH_WEST: u64 = 17;
const WEST_SOUTH_WEST: u64 = 10;
const SOUTH_SOUTH_EAST: u64 = 15;
const EAST_SOUTH_EAST: u64 = 6;

fn initialize_king_attacks(square: usize, attacks: &mut [Bitboard; NumberOf::SQUARES]) {
    let mut bb = Bitboard::default();
    let mut attacks_bb = Bitboard::default();

    // king can move 1 square in any direction
    bb.set_square(square);

    // with our bit board setup, "east" means right, and "west" means left
    // so this means east means we move more towards the MSB, so shift.
    // So all the east and north moves are shifted left, all south and west moves are shifted right

    let not_h_file = !FILE_BITBOARDS[File::H as usize];
    let not_a_file = !FILE_BITBOARDS[File::A as usize];
    let not_r8_rank = !RANK_BITBOARDS[Rank::R8 as usize];
    let not_r1_rank = !RANK_BITBOARDS[Rank::R1 as usize];

    attacks_bb |= (bb & not_r8_rank) << NORTH;
    attacks_bb |= (bb & not_a_file & not_r8_rank) << NORTH_WEST;
    attacks_bb |= (bb & not_h_file & not_r8_rank) << NORTH_EAST;
    attacks_bb |= (bb & not_h_file) << EAST;

    attacks_bb |= (bb & not_r1_rank) >> SOUTH;
    attacks_bb |= (bb & not_a_file & not_r1_rank) >> SOUTH_WEST;
    attacks_bb |= (bb & not_h_file & not_r1_rank) >> SOUTH_EAST;
    attacks_bb |= (bb & not_a_file) >> WEST;

    attacks[square as usize] = attacks_bb;
}

fn initialize_knight_attacks(square: usize, attacks: &mut [Bitboard; NumberOf::SQUARES]) {
    let mut bb = Bitboard::default();
    let mut attacks_bb = Bitboard::default();

    // knight can move 1 square in any direction
    bb.set_square(square);

    // with our bit board setup, "east" means right, and "west" means left
    // so this means east means we move more towards the MSB, so shift.
    // So all the east and north moves are shifted left, all south and west moves are shifted right
    let not_h_file = !FILE_BITBOARDS[File::H as usize];
    let not_gh_file = !FILE_BITBOARDS[File::G as usize] & !FILE_BITBOARDS[File::H as usize];
    let not_ab_file = !FILE_BITBOARDS[File::A as usize] & !FILE_BITBOARDS[File::B as usize];
    let not_a_file = !FILE_BITBOARDS[File::A as usize];
    let not_r8r7_rank = !RANK_BITBOARDS[Rank::R8 as usize] & !RANK_BITBOARDS[Rank::R7 as usize];
    let not_r1r2_rank = !RANK_BITBOARDS[Rank::R1 as usize] & !RANK_BITBOARDS[Rank::R2 as usize];

    attacks_bb |= (bb & not_h_file & not_r8r7_rank) << NORTH_NORTH_EAST;
    attacks_bb |= (bb & not_gh_file & not_r8r7_rank) << EAST_NORTH_EAST;
    attacks_bb |= (bb & not_a_file & not_r8r7_rank) << NORTH_NORTH_WEST;
    attacks_bb |= (bb & not_ab_file & not_r8r7_rank) << WEST_NORTH_WEST;

    attacks_bb |= (bb & not_h_file & not_r1r2_rank) >> SOUTH_SOUTH_EAST;
    attacks_bb |= (bb & not_gh_file & not_r1r2_rank) >> EAST_SOUTH_EAST;
    attacks_bb |= (bb & not_a_file & not_r1r2_rank) >> SOUTH_SOUTH_WEST;
    attacks_bb |= (bb & not_ab_file & not_r1r2_rank) >> WEST_SOUTH_WEST;

    attacks[square as usize] = attacks_bb;
}

fn initialize_pawn_attacks(
    square: usize,
    attacks: &mut [[Bitboard; NumberOf::SQUARES]; NumberOf::SIDES],
) {
    let mut bb = Bitboard::default();
    bb.set_square(square);

    let mut attacks_w_bb = Bitboard::default();
    let mut attacks_b_bb = Bitboard::default();

    let not_a_file = !FILE_BITBOARDS[File::A as usize];
    let not_h_file = !FILE_BITBOARDS[File::H as usize];

    // white is NORTH_WEST and NORTH_EAST
    attacks_w_bb |= (bb & not_a_file) << NORTH_WEST;
    attacks_w_bb |= (bb & not_h_file) << NORTH_EAST;

    attacks_b_bb |= (bb & not_a_file) >> SOUTH_WEST;
    attacks_b_bb |= (bb & not_h_file) >> SOUTH_EAST;

    attacks[Side::White as usize][square] = attacks_w_bb;
    attacks[Side::Black as usize][square] = attacks_b_bb;
}

pub struct MoveGenerator {
    king_attacks: [Bitboard; NumberOf::SQUARES],
    knight_attacks: [Bitboard; NumberOf::SQUARES],
    pawn_attacks: [[Bitboard; NumberOf::SQUARES]; NumberOf::SIDES],
    rook_magics: [MagicNumber; NumberOf::SQUARES],
    bishop_magics: [MagicNumber; NumberOf::SQUARES],
    rook_attacks: Vec<Bitboard>,
    bishop_attacks: Vec<Bitboard>,
}

impl MoveGenerator {
    pub fn new() -> Self {
        let king_attacks = [Bitboard::default(); NumberOf::SQUARES];
        let knight_attacks = [Bitboard::default(); NumberOf::SQUARES];
        let pawn_attacks = [[Bitboard::default(); NumberOf::SQUARES]; NumberOf::SIDES];
        let mut move_gen = Self {
            king_attacks,
            knight_attacks,
            pawn_attacks,
            rook_magics: [MagicNumber::default(); NumberOf::SQUARES],
            bishop_magics: [MagicNumber::default(); NumberOf::SQUARES],
            rook_attacks: vec![Bitboard::default(); ROOK_BLOCKER_PERMUTATIONS],
            bishop_attacks: vec![Bitboard::default(); BISHOP_BLOCKER_PERMUTATIONS],
        };

        move_gen.initialize_attack_boards();
        return move_gen;
    }

    fn initialize_attack_boards(&mut self) {
        for square in 0..NumberOf::SQUARES {
            initialize_king_attacks(square, &mut self.king_attacks);
            initialize_knight_attacks(square, &mut self.knight_attacks);
            initialize_pawn_attacks(square, &mut self.pawn_attacks);
        }

        self.initialize_magic_numbers(Piece::Rook);
        self.initialize_magic_numbers(Piece::Bishop);
    }

    fn initialize_magic_numbers(&mut self, piece: Piece) {
        assert!(piece == Piece::Rook || piece == Piece::Bishop);
        let mut offset = 0;

        for square in 0..NumberOf::SQUARES {
            let rook_relevant_bits = MoveGenerator::relevant_rook_bits(square);
            let bishop_relevant_bits = MoveGenerator::relevant_bishop_bits(square);
            let use_mask = if piece == Piece::Rook {
                rook_relevant_bits
            } else {
                bishop_relevant_bits
            };

            let bit_count = use_mask.as_number().count_ones();
            let total_permutations = 2u64.pow(bit_count);
            let end = offset + total_permutations - 1u64;
            let blocker_bitboards = MoveGenerator::create_blocker_permutations(use_mask);
            assert_eq!(blocker_bitboards.len(), total_permutations as usize);

            let rook_attacks = MoveGenerator::rook_attacks(square as u8, &blocker_bitboards);
            let bishop_attacks = MoveGenerator::bishop_attacks(square as u8, &blocker_bitboards);
            let attacks = if piece == Piece::Rook {
                rook_attacks
            } else {
                bishop_attacks
            };

            let magics = if piece == Piece::Rook {
                &mut self.rook_magics
            } else {
                &mut self.bishop_magics
            };

            let magic_constant = if piece == Piece::Rook {
                ROOK_MAGIC_VALUES
            } else {
                BISHOP_MAGIC_VALUES
            };

            let attack_table = if piece == Piece::Rook {
                &mut self.rook_attacks
            } else {
                &mut self.bishop_attacks
            };

            magics[square as usize] = MagicNumber::new(
                use_mask,
                (64 - bit_count) as u8,
                offset,
                magic_constant[square as usize],
            );

            for i in 0..blocker_bitboards.len() {
                let blocker = blocker_bitboards[i];
                let index = magics[square as usize].index(blocker);

                if attack_table[index] == Bitboard::default() || attack_table[index] == attacks[i] {
                    // did we fail high or low index wise? (out of bounds)
                    assert!(
                        index as u64 >= offset && index as u64 <= end,
                        "index out of bounds"
                    );
                    attack_table[index] = attacks[i];
                } else {
                    panic!("Collision detected while initializing attack tables for piece {:?} and square {} - \n\t{}", piece, SQUARE_NAME[square as usize], magics[square as usize]);
                }
            }

            // update the offset for the next square
            offset += total_permutations;
        }
    }

    fn edges(file: u8, rank: u8) -> Bitboard {
        // need to get the rank and file of the square
        let file_bb = FILE_BITBOARDS[file as usize];
        let rank_bb = RANK_BITBOARDS[rank as usize];
        // get the edges of the board
        let edges = (FILE_BITBOARDS[File::A as usize] & !file_bb)
            | (FILE_BITBOARDS[File::H as usize] & !file_bb)
            | (RANK_BITBOARDS[Rank::R1 as usize] & !rank_bb)
            | (RANK_BITBOARDS[Rank::R8 as usize] & !rank_bb);

        return edges;
    }

    fn edges_from_square(square: u8) -> Bitboard {
        let (file, rank) = square::from_square(square as u8);
        return MoveGenerator::edges(file, rank);
    }

    fn orthogonal_ray_attacks(square: u8, occupied: u64) -> Bitboard {
        let mut attacks = Bitboard::default();
        let bb = Bitboard::new(1u64 << square);
        let not_a_file = !FILE_BITBOARDS[File::A as usize];
        let not_h_file = !FILE_BITBOARDS[File::H as usize];

        // North
        let mut ray = bb;
        while ray != 0 {
            ray <<= 8;
            attacks |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        // South
        let mut ray = bb;
        while ray != 0 {
            ray >>= 8;
            attacks |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        // East
        let mut ray = bb;
        while ray != 0 && ray & not_h_file != 0 {
            ray <<= 1;
            attacks |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        // West
        let mut ray = bb;
        while ray != 0 && ray & not_a_file != 0 {
            ray >>= 1;
            attacks |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        attacks
    }

    fn diagonal_ray_attacks(square: u8, occupied: u64) -> Bitboard {
        let mut attacks = Bitboard::default();
        let bb = Bitboard::new(1u64 << square);
        let not_a_file = !FILE_BITBOARDS[File::A as usize];
        let not_h_file = !FILE_BITBOARDS[File::H as usize];

        // Northeast
        let mut ray = bb;
        while ray != 0 && ray & not_h_file != 0 {
            ray <<= 9;
            attacks |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        // Northwest
        let mut ray = bb;
        while ray != 0 && ray & not_a_file != 0 {
            ray <<= 7;
            attacks |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        // Southeast
        let mut ray = bb;
        while ray != 0 && ray & not_h_file != 0 {
            ray >>= 7;
            attacks |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        // Southwest
        let mut ray = bb;
        while ray != 0 && ray & not_a_file != 0 {
            ray >>= 9;
            attacks |= ray;
            if ray & occupied != 0 {
                break;
            }
        }

        attacks
    }

    pub fn relevant_rook_bits(square: usize) -> Bitboard {
        let mut bb = Bitboard::default();
        bb.set_square(square);

        // need to get the rank and file of the square
        let (file, rank) = square::from_square(square as u8);
        let rook_rays_bb = MoveGenerator::orthogonal_ray_attacks(square as u8, 0);
        // get the edges of the board
        let edges = MoveGenerator::edges(file, rank);

        return rook_rays_bb & !edges & !bb;
    }

    pub fn relevant_bishop_bits(square: usize) -> Bitboard {
        let mut bb = Bitboard::default();
        bb.set_square(square);

        let (file, rank) = square::from_square(square as u8);
        let edges = MoveGenerator::edges(file, rank);

        // need to calculate ray attacks for the bishop from its square
        let bishop_ray_attacks = MoveGenerator::diagonal_ray_attacks(square as u8, 0);

        return bishop_ray_attacks & !edges & !bb;
    }

    pub fn create_blocker_permutations(bb: Bitboard) -> Vec<Bitboard> {
        // use the carry-rippler method to cycle through all possible permutations of the given bitboard
        let mask = bb;
        let mut subset = Bitboard::default();

        const BASE: u64 = 2 as u64;
        let total_permutations = BASE.pow(bb.as_number().count_ones());

        let mut blocker_bitboards = Vec::with_capacity(total_permutations as usize);
        loop {
            // there could be no blockers, so start with that...
            blocker_bitboards.push(subset);
            // carry-rippler method to generate all possible permutations of the given bitboard
            subset = Bitboard::new(subset.as_number().wrapping_sub(mask.as_number())) & mask;
            if subset == 0 {
                break;
            }
        }
        blocker_bitboards
    }

    pub fn rook_attacks(square: u8, blockers: &Vec<Bitboard>) -> Vec<Bitboard> {
        let mut attacks = Vec::with_capacity(blockers.len());
        for blocker in blockers {
            attacks.push(MoveGenerator::calculate_rook_attack(square, blocker));
        }
        return attacks;
    }

    fn calculate_rook_attack(square: u8, blocker: &Bitboard) -> Bitboard {
        // calculate ray attacks for the rook from its square
        let rook_rays_bb = MoveGenerator::orthogonal_ray_attacks(square as u8, blocker.as_number());
        // get the edges of the board
        let edges = MoveGenerator::edges_from_square(square);
        // remove the edges from the ray attacks
        return rook_rays_bb & !edges;
    }

    pub fn bishop_attacks(square: u8, blockers: &Vec<Bitboard>) -> Vec<Bitboard> {
        let mut attacks = Vec::with_capacity(blockers.len());
        for blocker in blockers {
            attacks.push(MoveGenerator::calculate_bishop_attack(square, blocker));
        }
        return attacks;
    }

    pub fn calculate_bishop_attack(square: u8, blocker: &Bitboard) -> Bitboard {
        let bishop_rays_bb = MoveGenerator::diagonal_ray_attacks(square as u8, blocker.as_number());
        let edges = MoveGenerator::edges_from_square(square);
        return bishop_rays_bb & !edges;
    }

    /// Generates pseudo-legal moves for the current board state.
    /// This function does not check for legality of the moves.
    /// 
    /// # Arguments
    /// - board - The current board state
    /// - move_list - The list of moves to append to.
    /// - move_type - The type of moves to generate
    pub fn generate_moves(&self, board: &Board, move_list: &mut MoveList, move_type: &MoveType) {
        // get moves for each piece except pawns
        for piece in [
            Piece::King,
            Piece::Knight,
            Piece::Rook,
            Piece::Bishop,
            Piece::Queen,
        ] {
            self.get_piece_moves(piece, board, move_list, move_type);
        }
        // handle pawn moves separately
        self.get_pawn_moves(board, move_list, move_type);
        // handle castling moves
        self.get_castling_moves(board, move_list);
    }

    fn get_castling_moves(&self, board: &Board, move_list: &mut MoveList) {
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

        let occupancy = board.all_pieces();

        // white king side castling
        if board.can_castle_kingside(Side::White) {
            let king_from = Square::from_square_index(Squares::E1); // e1
            let king_to = Square::from_square_index(Squares::G1); // g1
            let blockers = Bitboard::from_square(Squares::F1) | Bitboard::from_square(Squares::G1);
            let king_ray = [Squares::E1, Squares::F1, Squares::G1];

            let is_blocked = (blockers & occupancy) > 0;
            let are_any_attacked = king_ray.iter().any(|&square| {
                self.is_square_attacked(board, &Square::from_square_index(square), Side::Black)
            });

            if !is_blocked
                && !are_any_attacked
                && !self.is_square_attacked(board, &king_from, Side::Black)
                && !self.is_square_attacked(board, &king_to, Side::Black)
            {
                move_list.push(Move::new_castle(&king_from, &king_to));
            }
        }

        if board.can_castle_queenside(Side::White) {
            let king_from = Square::from_square_index(Squares::E1);
            let king_to = Square::from_square_index(Squares::C1);
            let blockers = Bitboard::from_square(Squares::D1)
                | Bitboard::from_square(Squares::C1)
                | Bitboard::from_square(Squares::B1);
            let king_ray = [Squares::E1, Squares::D1, Squares::C1];

            let is_blocked = (blockers & occupancy) > 0;
            let are_any_attacked = king_ray.iter().any(|&square| {
                self.is_square_attacked(board, &Square::from_square_index(square), Side::Black)
            });

            if !is_blocked
                && !are_any_attacked
                && !self.is_square_attacked(board, &king_from, Side::Black)
                && !self.is_square_attacked(board, &king_to, Side::Black)
            {
                move_list.push(Move::new_castle(&king_from, &king_to));
            }
        }

        if board.can_castle_kingside(Side::Black) {
            let king_from = Square::from_square_index(Squares::E8);
            let king_to = Square::from_square_index(Squares::G8);
            let blockers = Bitboard::from_square(Squares::F8) | Bitboard::from_square(Squares::G8);
            let king_ray = [Squares::E8, Squares::F8, Squares::G8];
            let is_blocked = (blockers & occupancy) > 0;
            let are_any_attacked = king_ray.iter().any(|&square| {
                self.is_square_attacked(board, &Square::from_square_index(square), Side::White)
            });

            if !is_blocked
                && !are_any_attacked
                && !self.is_square_attacked(board, &king_from, Side::White)
                && !self.is_square_attacked(board, &king_to, Side::White)
            {
                move_list.push(Move::new_castle(&king_from, &king_to));
            }
        }

        if board.can_castle_queenside(Side::Black) {
            let king_from = Square::from_square_index(Squares::E8);
            let king_to = Square::from_square_index(Squares::C8);
            let blockers = Bitboard::from_square(Squares::D8)
                | Bitboard::from_square(Squares::C8)
                | Bitboard::from_square(Squares::B8);
            let king_ray = [Squares::E8, Squares::D8, Squares::C8];
            let is_blocked = (blockers & occupancy) > 0;
            let are_any_attacked = king_ray.iter().any(|&square| {
                self.is_square_attacked(board, &Square::from_square_index(square), Side::White)
            });

            if !is_blocked
                && !are_any_attacked
                && !self.is_square_attacked(board, &king_from, Side::White)
                && !self.is_square_attacked(board, &king_to, Side::White)
            {
                move_list.push(Move::new_castle(&king_from, &king_to));
            }
        }
    }

    fn get_piece_moves(
        &self,
        piece: Piece,
        board: &Board,
        move_list: &mut MoveList,
        move_type: &MoveType,
    ) {
        let us = board.side_to_move();
        let them = Side::opposite(us);
        let our_pieces = board.pieces(us);
        let their_pieces = board.pieces(them);
        let occupancy = board.all_pieces();
        let empty = !occupancy;

        let mut piece_bb = *board.piece_bitboard(piece, us);
        // loop through all the pieces of the given type
        while piece_bb.as_number() > 0 {
            let from_square = bitboard_helpers::next_bit(&mut piece_bb) as u8;
            let attack_bb = match piece {
                Piece::King | Piece::Knight => self.get_non_slider_moves(piece, from_square),
                Piece::Rook | Piece::Bishop | Piece::Queen => {
                    self.get_slider_moves(piece, from_square, &occupancy)
                }
                _ => panic!("Piece must be non-slider and not pawn"),
            };

            let bb_moves = match move_type {
                MoveType::Capture => attack_bb & their_pieces,
                MoveType::Quiet => attack_bb & empty,
                MoveType::All => attack_bb & !our_pieces,
            };

            self.enumerate_moves(
                &bb_moves,
                &Square::from_square_index(from_square),
                piece,
                board,
                move_list,
            );
        }
    }

    fn get_non_slider_moves(&self, piece: Piece, from_square: u8) -> Bitboard {
        assert!(
            piece == Piece::King || piece == Piece::Knight,
            "Piece must be non-slider and not pawn"
        );

        let attack_table = match piece {
            Piece::King => self.king_attacks,
            Piece::Knight => self.knight_attacks,
            _ => panic!("Piece must be non-slider and not pawn"),
        };

        attack_table[from_square as usize]
    }

    fn get_slider_moves(&self, piece: Piece, from_square: u8, occupancy: &Bitboard) -> Bitboard {
        assert!(
            piece == Piece::Rook || piece == Piece::Bishop || piece == Piece::Queen,
            "Piece must be a slider"
        );

        match piece {
            Piece::Rook => {
                let index = self.rook_magics[from_square as usize].index(*occupancy);
                self.rook_attacks[index]
            }
            Piece::Bishop => {
                let index = self.bishop_magics[from_square as usize].index(*occupancy);
                self.bishop_attacks[index]
            }
            Piece::Queen => {
                let rook_index = self.rook_magics[from_square as usize].index(*occupancy);
                let bishop_index = self.bishop_magics[from_square as usize].index(*occupancy);
                self.rook_attacks[rook_index] ^ self.bishop_attacks[bishop_index]
            }
            _ => panic!("Piece must be a slider"),
        }
    }

    fn get_pawn_moves(&self, board: &Board, move_list: &mut MoveList, move_type: &MoveType) {
        let us = board.side_to_move();
        let them = Side::opposite(us);
        let their_pieces = board.pieces(them);
        let occupancy = board.all_pieces();
        let empty = !occupancy;
        let direction = if us == Side::White { NORTH } else { SOUTH };
        let pawns_bb = board.piece_bitboard(Piece::Pawn, us);

        let mut bb = *pawns_bb;

        // loop through all the pawns for us
        while bb > 0 {
            let from_square = bitboard_helpers::next_bit(&mut bb) as u8;
            let attack_bb = self.pawn_attacks[us as usize][from_square as usize];

            let mut bb_moves = Bitboard::default();
            let to_square = from_square as u64 + direction;
            let double_push_square = match us {
                Side::White => to_square + direction,
                Side::Black => to_square - direction,
                Side::Both => panic!("Both side not allowed"),
            };

            // pawn non-capture moves
            if *move_type == MoveType::All || *move_type == MoveType::Quiet {
                let bb_push = Bitboard::new(1u64 << to_square);
                println!("push: \n{}", bb_push);
                let bb_single_push = bb_push & empty;
                let bb_double_push = Bitboard::new(1u64 << double_push_square) & empty;
                bb_moves |= bb_single_push | bb_double_push;
            }

            // pawn captures
            if move_type == &MoveType::All || move_type == &MoveType::Capture {
                let bb_capture = attack_bb & their_pieces;
                // en passant
                let bb_en_passant = match board.en_passant_square() {
                    Some(en_passant_square) => Bitboard::new(en_passant_square as u64),
                    None => Bitboard::default(),
                };

                bb_moves |= bb_capture | bb_en_passant;
            }

            self.enumerate_moves(
                &bb_moves,
                &Square::from_square_index(from_square),
                Piece::Pawn,
                board,
                move_list,
            );
        }
    }

    fn enumerate_moves(
        &self,
        bitboard: &Bitboard,
        from: &Square,
        piece: Piece,
        board: &Board,
        move_list: &mut MoveList,
    ) {
        let mut bb = *bitboard;
        let us = board.side_to_move();
        let them = Side::opposite(us);
        let enemy_pieces = board.pieces(them);
        let promotion_rank = Rank::promotion_rank(us);
        while bb > 0 {
            let to_square = bitboard_helpers::next_bit(&mut bb) as u8;
            let (file, rank) = square::from_square(to_square as u8);

            let en_passant = match board.board_state().en_passant_square {
                Some(en_passant_square) => en_passant_square == to_square && piece == Piece::Pawn,
                None => false,
            };

            let is_capture = enemy_pieces.is_square_occupied(to_square as usize) || en_passant;
            // 2 rows = 16 squares
            let is_double_move = piece == Piece::Pawn
                && (to_square as i8 - from.to_square_index() as i8).abs() == 16;
            let is_promotion =
                piece == Piece::Pawn && Board::is_square_on_rank(to_square, promotion_rank as u8);

            let mut flags = Flags::NONE;

            if is_double_move {
                flags |= Flags::PAWN_TWO_UP;
            }

            if en_passant {
                flags |= Flags::EN_PASSANT_CAPTURE;
            }

            let capture_piece = if is_capture {
                Some(board.piece_on_square(to_square as usize).unwrap().0)
            } else {
                None
            };

            let to_square = square::to_square_object(file, rank);
            if is_promotion {
                // we have to add 4 moves for each promotion type
                for promotion_flag in [
                    Flags::PROMOTE_TO_BISHOP,
                    Flags::PROMOTE_TO_KNIGHT,
                    Flags::PROMOTE_TO_QUEEN,
                    Flags::PROMOTE_TO_ROOK,
                ] {
                    let mv = Move::new(
                        &from,
                        &to_square,
                        flags | promotion_flag,
                        piece,
                        capture_piece,
                    );
                    move_list.push(mv);
                }
            } else {
                let mv = Move::new(&from, &to_square, flags, piece, capture_piece);
                move_list.push(mv);
            }
        }
    }

    /// Returns true if the given square is attacked by any piece that is on the attacking_side.
    /// This method uses the so called "super-piece" method.
    /// See: https://talkchess.com/viewtopic.php?t=27152
    ///
    /// The gist is that we treat the attacking square as the from square and we project the attacks to the same sides pieces.
    /// If there are any collisions, then we know that a piece is attacking that square.
    ///
    /// # Arguments
    /// - board: the current board state
    /// - square: the square to check if it is attacked
    /// - attacking_side: the side that is potentially attacking the square
    ///
    /// # Returns
    /// - true if the square is attacked, false otherwise
    pub fn is_square_attacked(&self, board: &Board, square: &Square, attacking_side: Side) -> bool {
        let king_bb = board.piece_bitboard(Piece::King, attacking_side);
        let knight_bb = board.piece_bitboard(Piece::Knight, attacking_side);
        let bishop_bb = board.piece_bitboard(Piece::Bishop, attacking_side);
        let rook_bb = board.piece_bitboard(Piece::Rook, attacking_side);
        let queen_bb = board.piece_bitboard(Piece::Queen, attacking_side);
        let pawn_bb = board.piece_bitboard(Piece::Pawn, attacking_side);

        let occupancy = board.all_pieces();
        let king_attacks = self.get_non_slider_moves(Piece::King, square.to_square_index());
        let knight_attacks = self.get_non_slider_moves(Piece::Knight, square.to_square_index());
        let rook_attacks = self.get_slider_moves(Piece::Rook, square.to_square_index(), &occupancy);
        let bishop_attacks =
            self.get_slider_moves(Piece::Bishop, square.to_square_index(), &occupancy);
        let queen_attacks = rook_attacks | bishop_attacks;
        // note we use the opposite side for the pawn attacks
        let pawn_attacks = self.pawn_attacks[Side::opposite(attacking_side) as usize]
            [square.to_square_index() as usize];

        return (king_attacks & *king_bb) > 0
            || (knight_attacks & *knight_bb) > 0
            || (rook_attacks & *rook_bb) > 0
            || (bishop_attacks & *bishop_bb) > 0
            || (queen_attacks & *queen_bb) > 0
            || (pawn_attacks & *pawn_bb) > 0;
    }
}

#[cfg(test)]
mod tests {

    use crate::move_generation;

    use super::*;

    #[test]
    fn check_is_square_attacked() {
        let board = Board::default_board();
        let move_gen = MoveGenerator::new();
        // loop through all the occupied squares and check if they are attacked
        let mut occupancy = board.all_pieces();
        let mut sq = bitboard_helpers::next_bit(&mut occupancy);
        while sq > 0 {
            let square = Square::from_square_index(sq as u8);
            let is_attacked = move_gen.is_square_attacked(&board, &square, Side::White);
            assert!(!is_attacked);
            sq = bitboard_helpers::next_bit(&mut occupancy);
        }

        // now generate moves and check if the squares that pieces can move to are attacked
        let mut move_list = MoveList::new();
        move_gen.generate_moves(&board, &mut move_list, &MoveType::All);
        let side_to_move = board.side_to_move();
        // we ignore pawn two up moves because they are not "attacks"
        for mv in move_list.iter().filter(|mv| !mv.is_pawn_two_up()) {
            let to = mv.to();
            let is_attacked =
                move_gen.is_square_attacked(&board, &Square::from_square_index(to), side_to_move);
            assert!(
                is_attacked,
                "Square {} is not attacked by move\n\t{}",
                to, mv
            );
        }
    }

    #[test]
    fn check_basic_construction() {
        let move_gen = MoveGenerator::new();
        // verify the order of the magic numbers
        for square in 0..NumberOf::SQUARES {
            let rook_magic = move_gen.rook_magics[square];
            let bishop_magic = move_gen.bishop_magics[square];
            assert_eq!(rook_magic.magic_value, ROOK_MAGIC_VALUES[square as usize]);
            assert_eq!(
                bishop_magic.magic_value,
                BISHOP_MAGIC_VALUES[square as usize]
            );
        }
    }

    #[test]
    fn check_king_attacks() {
        let move_gen = MoveGenerator::new();
        let king_attacks = move_gen.king_attacks;

        // these were generated empirically by running this test and printing out the attack bitboards as numbers
        let expected_king_attacks: [u64; NumberOf::SQUARES] = [
            770,
            1797,
            3594,
            7188,
            14376,
            28752,
            57504,
            49216,
            197123,
            460039,
            920078,
            1840156,
            3680312,
            7360624,
            14721248,
            12599488,
            50463488,
            117769984,
            235539968,
            471079936,
            942159872,
            1884319744,
            3768639488,
            3225468928,
            12918652928,
            30149115904,
            60298231808,
            120596463616,
            241192927232,
            482385854464,
            964771708928,
            825720045568,
            3307175149568,
            7718173671424,
            15436347342848,
            30872694685696,
            61745389371392,
            123490778742784,
            246981557485568,
            211384331665408,
            846636838289408,
            1975852459884544,
            3951704919769088,
            7903409839538176,
            15806819679076352,
            31613639358152704,
            63227278716305408,
            54114388906344448,
            216739030602088448,
            505818229730443264,
            1011636459460886528,
            2023272918921773056,
            4046545837843546112,
            8093091675687092224,
            16186183351374184448,
            13853283560024178688,
            144959613005987840,
            362258295026614272,
            724516590053228544,
            1449033180106457088,
            2898066360212914176,
            5796132720425828352,
            11592265440851656704,
            4665729213955833856,
        ];

        for square in 0..NumberOf::SQUARES {
            let attacks_bb = king_attacks[square];
            assert_eq!(attacks_bb.as_number(), expected_king_attacks[square]);
        }
    }

    #[test]
    fn check_knight_attacks() {
        let move_gen = MoveGenerator::new();
        let knight_attacks = move_gen.knight_attacks;
        let expected_knight_attacks: [u64; NumberOf::SQUARES] = [
            132096,
            329728,
            659712,
            1319424,
            2638848,
            5277696,
            10489856,
            4202496,
            33816576,
            84410368,
            168886272,
            337772544,
            675545088,
            1351090176,
            2685403136,
            1075838976,
            8657044482,
            21609056261,
            43234889994,
            86469779988,
            172939559976,
            345879119952,
            687463207072,
            275414786112,
            2216203387392,
            5531918402816,
            11068131838464,
            22136263676928,
            44272527353856,
            88545054707712,
            175990581010432,
            70506185244672,
            567348067172352,
            1416171111120896,
            2833441750646784,
            5666883501293568,
            11333767002587136,
            22667534005174272,
            45053588738670592,
            18049583422636032,
            145241105196122112,
            362539804446949376,
            725361088165576704,
            1450722176331153408,
            2901444352662306816,
            5802888705324613632,
            11533718717099671552,
            4620693356194824192,
            4406636445696,
            8817567858688,
            18734647345152,
            37469294690304,
            74938589380608,
            149877178761216,
            18279380811776,
            35459249995776,
            1128098930098176,
            2257297371824128,
            4796069720358912,
            9592139440717824,
            19184278881435648,
            38368557762871296,
            4679521487814656,
            9077567998918656,
        ];
        for square in 0..NumberOf::SQUARES {
            let attacks_bb = knight_attacks[square];
            assert_eq!(attacks_bb.as_number(), expected_knight_attacks[square]);
        }
    }

    #[test]
    fn check_pawn_attacks() {
        let move_gen = MoveGenerator::new();
        let pawn_attacks = move_gen.pawn_attacks;
        let expected_black_pawn_attacks: [u64; NumberOf::SQUARES] = [
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            2,
            5,
            10,
            20,
            40,
            80,
            160,
            64,
            512,
            1280,
            2560,
            5120,
            10240,
            20480,
            40960,
            16384,
            131072,
            327680,
            655360,
            1310720,
            2621440,
            5242880,
            10485760,
            4194304,
            33554432,
            83886080,
            167772160,
            335544320,
            671088640,
            1342177280,
            2684354560,
            1073741824,
            8589934592,
            21474836480,
            42949672960,
            85899345920,
            171798691840,
            343597383680,
            687194767360,
            274877906944,
            2199023255552,
            5497558138880,
            10995116277760,
            21990232555520,
            43980465111040,
            87960930222080,
            175921860444160,
            70368744177664,
            562949953421312,
            1407374883553280,
            2814749767106560,
            5629499534213120,
            11258999068426240,
            22517998136852480,
            45035996273704960,
            18014398509481984,
        ];

        let expected_white_pawn_attacks: [u64; NumberOf::SQUARES] = [
            512,
            1280,
            2560,
            5120,
            10240,
            20480,
            40960,
            16384,
            131072,
            327680,
            655360,
            1310720,
            2621440,
            5242880,
            10485760,
            4194304,
            33554432,
            83886080,
            167772160,
            335544320,
            671088640,
            1342177280,
            2684354560,
            1073741824,
            8589934592,
            21474836480,
            42949672960,
            85899345920,
            171798691840,
            343597383680,
            687194767360,
            274877906944,
            2199023255552,
            5497558138880,
            10995116277760,
            21990232555520,
            43980465111040,
            87960930222080,
            175921860444160,
            70368744177664,
            562949953421312,
            1407374883553280,
            2814749767106560,
            5629499534213120,
            11258999068426240,
            22517998136852480,
            45035996273704960,
            18014398509481984,
            144115188075855872,
            360287970189639680,
            720575940379279360,
            1441151880758558720,
            2882303761517117440,
            5764607523034234880,
            11529215046068469760,
            4611686018427387904,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        for square in 0..NumberOf::SQUARES {
            let attacks_b_bb = pawn_attacks[Side::Black as usize][square];
            let attacks_w_bb = pawn_attacks[Side::White as usize][square];
            assert_eq!(
                attacks_b_bb.as_number(),
                expected_black_pawn_attacks[square]
            );
            assert_eq!(
                attacks_w_bb.as_number(),
                expected_white_pawn_attacks[square]
            );
        }
    }

    #[test]
    fn check_rook_relevant_bits() {
        let rook_relevant_bit_expected: [u64; NumberOf::SQUARES] = [
            282578800148862,
            565157600297596,
            1130315200595066,
            2260630401190006,
            4521260802379886,
            9042521604759646,
            18085043209519166,
            36170086419038334,
            282578800180736,
            565157600328704,
            1130315200625152,
            2260630401218048,
            4521260802403840,
            9042521604775424,
            18085043209518592,
            36170086419037696,
            282578808340736,
            565157608292864,
            1130315208328192,
            2260630408398848,
            4521260808540160,
            9042521608822784,
            18085043209388032,
            36170086418907136,
            282580897300736,
            565159647117824,
            1130317180306432,
            2260632246683648,
            4521262379438080,
            9042522644946944,
            18085043175964672,
            36170086385483776,
            283115671060736,
            565681586307584,
            1130822006735872,
            2261102847592448,
            4521664529305600,
            9042787892731904,
            18085034619584512,
            36170077829103616,
            420017753620736,
            699298018886144,
            1260057572672512,
            2381576680245248,
            4624614895390720,
            9110691325681664,
            18082844186263552,
            36167887395782656,
            35466950888980736,
            34905104758997504,
            34344362452452352,
            33222877839362048,
            30979908613181440,
            26493970160820224,
            17522093256097792,
            35607136465616896,
            9079539427579068672,
            8935706818303361536,
            8792156787827803136,
            8505056726876686336,
            7930856604974452736,
            6782456361169985536,
            4485655873561051136,
            9115426935197958144,
        ];

        let mut offset_sum: u64 = 0;
        const BASE: u64 = 2 as u64;
        for square in 0..NumberOf::SQUARES {
            let rook_bits = move_generation::MoveGenerator::relevant_rook_bits(square);
            assert_eq!(rook_bits.as_number(), rook_relevant_bit_expected[square]);

            offset_sum += BASE.pow(rook_bits.as_number().count_ones());
        }
        println!("rook offset sum: {}", offset_sum);
    }

    #[test]
    fn check_relevant_bishop_bits() {
        let bishop_relevant_bit_expected: [u64; NumberOf::SQUARES] = [
            18049651735527936,
            70506452091904,
            275415828992,
            1075975168,
            38021120,
            8657588224,
            2216338399232,
            567382630219776,
            9024825867763712,
            18049651735527424,
            70506452221952,
            275449643008,
            9733406720,
            2216342585344,
            567382630203392,
            1134765260406784,
            4512412933816832,
            9024825867633664,
            18049651768822272,
            70515108615168,
            2491752130560,
            567383701868544,
            1134765256220672,
            2269530512441344,
            2256206450263040,
            4512412900526080,
            9024834391117824,
            18051867805491712,
            637888545440768,
            1135039602493440,
            2269529440784384,
            4539058881568768,
            1128098963916800,
            2256197927833600,
            4514594912477184,
            9592139778506752,
            19184279556981248,
            2339762086609920,
            4538784537380864,
            9077569074761728,
            562958610993152,
            1125917221986304,
            2814792987328512,
            5629586008178688,
            11259172008099840,
            22518341868716544,
            9007336962655232,
            18014673925310464,
            2216338399232,
            4432676798464,
            11064376819712,
            22137335185408,
            44272556441600,
            87995357200384,
            35253226045952,
            70506452091904,
            567382630219776,
            1134765260406784,
            2832480465846272,
            5667157807464448,
            11333774449049600,
            22526811443298304,
            9024825867763712,
            18049651735527936,
        ];

        let mut offset_sum: u64 = 0;
        const BASE: u64 = 2 as u64;

        for square in 0..NumberOf::SQUARES {
            let bishop_bits = move_generation::MoveGenerator::relevant_bishop_bits(square);
            assert_eq!(
                bishop_bits.as_number(),
                bishop_relevant_bit_expected[square]
            );

            offset_sum += BASE.pow(bishop_bits.as_number().count_ones());
        }

        println!("bishop offset sum: {}", offset_sum);
    }

    #[test]
    fn check_blocker_permutations() {
        const BASE: u64 = 2 as u64;

        for sq in 0..NumberOf::SQUARES {
            let rook_bb = MoveGenerator::relevant_rook_bits(sq);
            let permutations = MoveGenerator::create_blocker_permutations(rook_bb);
            let total_permutations = BASE.pow(rook_bb.as_number().count_ones());
            assert_eq!(permutations.len(), total_permutations as usize);
            for bb in permutations {
                // check that the permutation is a subset of the rook bitboard
                if (bb) != 0 {
                    assert_eq!(bb & !rook_bb, 0);
                }
            }
        }
    }

    #[test]
    fn check_rook_attacks() {
        for square in 0..NumberOf::SQUARES {
            let rook_bb = MoveGenerator::relevant_rook_bits(square as usize);
            let blockers = MoveGenerator::create_blocker_permutations(rook_bb);

            let attacks = MoveGenerator::rook_attacks(square as u8, &blockers);
            assert_eq!(attacks.len(), blockers.len());

            for attack in attacks {
                // attack should be a subset of the rook bitboard
                assert_eq!(attack & !rook_bb, 0);
            }
        }
    }

    #[test]
    fn check_bishop_attacks() {
        for square in 0..NumberOf::SQUARES {
            let bishop_bb = MoveGenerator::relevant_bishop_bits(square as usize);
            let blockers = MoveGenerator::create_blocker_permutations(bishop_bb);

            let attacks = MoveGenerator::bishop_attacks(square as u8, &blockers);
            assert_eq!(attacks.len(), blockers.len());

            for attack in attacks {
                // attack should be a subset of the bishop bitboard
                assert_eq!(attack & !bishop_bb, 0);
            }
        }
    }

    #[test]
    fn check_basic_move_gen() {
        let board = Board::default_board();
        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&board, &mut move_list, &MoveType::All);

        for mv in move_list.iter() {
            println!("{}", mv);
        }

        assert_eq!(move_list.len(), 20);
    }
}
