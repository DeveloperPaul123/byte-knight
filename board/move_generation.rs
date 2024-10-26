/*
 * move_generation.rs
 * Part of the byte-knight project
 * Created Date: Wednesday, August 28th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Fri Oct 25 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use core::num;
use std::u64;

use crate::{
    bitboard::Bitboard,
    bitboard_helpers,
    board::Board,
    definitions::{
        NumberOf, Squares, BISHOP_BLOCKER_PERMUTATIONS, QUEEN_OFFSETS, ROOK_BLOCKER_PERMUTATIONS,
    },
    file::File,
    magics::{MagicNumber, BISHOP_MAGIC_VALUES, ROOK_MAGIC_VALUES},
    move_list::MoveList,
    moves::{Move, MoveDescriptor, MoveType, PromotionDescriptor},
    pieces::{Piece, SLIDER_PIECES, SQUARE_NAME},
    rank::Rank,
    side::{self, Side},
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

fn initialize_king_attacks(square: u8, attacks: &mut [Bitboard; NumberOf::SQUARES]) {
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

fn initialize_knight_attacks(square: u8, attacks: &mut [Bitboard; NumberOf::SQUARES]) {
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

    attacks_bb |= (bb & not_h_file) << NORTH_NORTH_EAST;
    attacks_bb |= (bb & not_gh_file) << EAST_NORTH_EAST;
    attacks_bb |= (bb & not_a_file) << NORTH_NORTH_WEST;
    attacks_bb |= (bb & not_ab_file) << WEST_NORTH_WEST;

    attacks_bb |= (bb & not_h_file) >> SOUTH_SOUTH_EAST;
    attacks_bb |= (bb & not_gh_file) >> EAST_SOUTH_EAST;
    attacks_bb |= (bb & not_a_file) >> SOUTH_SOUTH_WEST;
    attacks_bb |= (bb & not_ab_file) >> WEST_SOUTH_WEST;

    attacks[square as usize] = attacks_bb;
}

fn initialize_pawn_attacks(
    square: u8,
    attacks: &mut [[Bitboard; NumberOf::SQUARES]; NumberOf::SIDES],
) {
    let mut bb = Bitboard::default();
    bb.set_square(square as u8);

    let mut attacks_w_bb = Bitboard::default();
    let mut attacks_b_bb = Bitboard::default();

    let not_a_file = !FILE_BITBOARDS[File::A as usize];
    let not_h_file = !FILE_BITBOARDS[File::H as usize];

    // white is NORTH_WEST and NORTH_EAST
    attacks_w_bb |= (bb & not_a_file) << NORTH_WEST;
    attacks_w_bb |= (bb & not_h_file) << NORTH_EAST;

    attacks_b_bb |= (bb & not_a_file) >> SOUTH_WEST;
    attacks_b_bb |= (bb & not_h_file) >> SOUTH_EAST;

    attacks[Side::White as usize][square as usize] = attacks_w_bb;
    attacks[Side::Black as usize][square as usize] = attacks_b_bb;
}

fn initialize_rays_between(rays_between: &mut [[Bitboard; NumberOf::SQUARES]; NumberOf::SQUARES]) {
    for sq in 0..NumberOf::SQUARES as u8 {
        let from_square = Square::from_square_index(sq);
        for (delta_file, delta_rank) in QUEEN_OFFSETS {
            let mut ray = Bitboard::default();
            let mut to = from_square;
            while let Some(shifted) = to.offset(delta_file, delta_rank) {
                ray.set_square(shifted.to_square_index());
                to = shifted;
                let from_index = from_square.to_square_index() as usize;
                let to_index = to.to_square_index() as usize;
                rays_between[from_index][to_index] =
                    ray ^ Bitboard::from_square(to.to_square_index());
            }
        }
    }
}

pub struct MoveGenerator {
    king_attacks: [Bitboard; NumberOf::SQUARES],
    knight_attacks: [Bitboard; NumberOf::SQUARES],
    pawn_attacks: [[Bitboard; NumberOf::SQUARES]; NumberOf::SIDES],
    rook_magics: [MagicNumber; NumberOf::SQUARES],
    bishop_magics: [MagicNumber; NumberOf::SQUARES],
    rook_attacks: Vec<Bitboard>,
    bishop_attacks: Vec<Bitboard>,
    rays_between: [[Bitboard; NumberOf::SQUARES]; NumberOf::SQUARES],
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
            rays_between: [[Bitboard::default(); NumberOf::SQUARES]; NumberOf::SQUARES],
        };

        move_gen.initialize_attack_boards();
        initialize_rays_between(&mut move_gen.rays_between);
        return move_gen;
    }

    fn initialize_attack_boards(&mut self) {
        for square in 0..NumberOf::SQUARES as u8 {
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

        for square in 0..NumberOf::SQUARES as u8 {
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

    #[allow(dead_code)]
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

    pub fn relevant_rook_bits(square: u8) -> Bitboard {
        let mut bb = Bitboard::default();
        bb.set_square(square);

        // need to get the rank and file of the square
        let (file, rank) = square::from_square(square as u8);
        let rook_rays_bb = MoveGenerator::orthogonal_ray_attacks(square as u8, 0);
        // get the edges of the board
        let edges = MoveGenerator::edges(file, rank);

        return rook_rays_bb & !edges & !bb;
    }

    pub fn relevant_bishop_bits(square: u8) -> Bitboard {
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

    /// Generate rook attacks for all possible blocker permutations at a given square.
    ///
    /// # Arguments
    /// - square - The square to generate the attacks for
    /// - blockers - The list of blocker permutations
    ///
    /// # Returns
    ///
    /// A vector of bitboards representing the rook attacks for each blocker permutation.
    pub fn rook_attacks(square: u8, blockers: &Vec<Bitboard>) -> Vec<Bitboard> {
        let mut attacks = Vec::with_capacity(blockers.len());
        for blocker in blockers {
            attacks.push(MoveGenerator::calculate_rook_attack(square, blocker));
        }
        return attacks;
    }

    pub fn calculate_rook_attack(square: u8, blocker: &Bitboard) -> Bitboard {
        // calculate ray attacks for the rook from its square
        let rook_rays_bb = MoveGenerator::orthogonal_ray_attacks(square as u8, blocker.as_number());
        return rook_rays_bb;
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
        return bishop_rays_bb;
    }

    pub fn ray_between(&self, from: Square, to: Square) -> Bitboard {
        return self.rays_between[from.to_square_index() as usize][to.to_square_index() as usize];
    }

    /// Calculate all squares currently being attacked by a given side.
    ///
    /// # Arguments
    /// - board - The current board state
    /// - side - The side to calculate the attacked squares for
    ///
    /// # Returns
    ///
    /// A bitboard representing all squares currently being attacked by the given side.
    fn get_attacked_squares(&self, board: &Board, side: Side, occupancy: &Bitboard) -> Bitboard {
        let mut attacks = Bitboard::default();
        let our_pieces = board.pieces(side);

        // get the squares attacked by each piece
        for piece in [
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
            Piece::Knight,
            Piece::Pawn,
        ]
        .iter()
        {
            let mut piece_bb = board.piece_bitboard(*piece, side).clone();
            if piece_bb.as_number() == 0 {
                continue;
            }

            while piece_bb.as_number() > 0 {
                let from = bitboard_helpers::next_bit(&mut piece_bb) as u8;
                let attacks_bb =
                    if piece == &Piece::Bishop || piece == &Piece::Queen || piece == &Piece::Rook {
                        self.get_slider_attacks(*piece, from, &occupancy)
                    } else if piece == &Piece::Pawn {
                        self.pawn_attacks[side as usize][from as usize]
                    } else {
                        self.get_non_slider_attacks(*piece, from)
                    };

                attacks |= attacks_bb;
            }
        }

        return attacks & !our_pieces;
    }

    fn get_piece_attacks(
        &self,
        piece: Piece,
        square: u8,
        attacking_side: Side,
        occupancy: &Bitboard,
    ) -> Bitboard {
        if piece.is_none() {
            panic!("Cannot get attacks for an non-piece");
        }

        if piece.is_slider() {
            self.get_slider_attacks(piece, square, occupancy)
        } else if piece == Piece::Pawn {
            self.pawn_attacks[Side::opposite(attacking_side) as usize][square as usize]
        } else {
            self.get_non_slider_attacks(piece, square)
        }
    }

    fn calculate_attacks_from_square(
        &self,
        square: u8,
        attackers: &[Piece],
        side: Side,
        occupancy: &Bitboard,
    ) -> Bitboard {
        let mut attacks = Bitboard::default();
        for attacker in attackers {
            let attack_bb = self.get_piece_attacks(*attacker, square, side, occupancy);
            attacks |= attack_bb;
        }
        return attacks;
    }

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
    ) -> (Bitboard, Bitboard) {
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

        println!("Opponent Slider Attacks:\n{}", opponent_slider_attacks);

        let mut from_our_king_attacks = Bitboard::default();
        for piece in SLIDER_PIECES.iter() {
            let piece_bb = board.piece_bitboard(*piece, them);
            let attacks = self.get_slider_attacks(*piece, king_square, &their_pieces);
            println!("attacks:\n{}", attacks);
            if attacks.intersects(*piece_bb) {
                println!("-- intersects");
                let intersection = attacks & *piece_bb;
                let piece_sq = bitboard_helpers::next_bit(&mut intersection.clone()) as u8;
                from_our_king_attacks |= self.ray_between(
                    Square::from_square_index(king_square),
                    Square::from_square_index(piece_sq),
                );
            }
        }

        println!("From Our King Attacks:\n{}", from_our_king_attacks);

        let pin_rays = opponent_slider_attacks & from_our_king_attacks;
        println!("Pin Rays:\n{}", pin_rays);
        let pinned_pieces = pin_rays & our_pieces;
        println!("Pinned Pieces:\n{}", pinned_pieces);

        println!("Occupancy:\n{}", occupancy);
        println!("King BB:\n{}", king_bb);
        // ensure we definitely don't have the king in the occupancy
        let kingless_occupancy = *occupancy & !(*king_bb);
        println!("Kingless Occupancy:\n{}", kingless_occupancy);
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

        println!("Checkers:\n{}", checkers);
        // loop through the sliding attacks to check if they are checkers or a pinner

        // while slider_attacks.as_number() > 0 {
        //     let attacker_sq = bitboard_helpers::next_bit(&mut slider_attacks) as u8;
        //     let ray = self.ray_between(
        //         Square::from_square_index(king_square),
        //         Square::from_square_index(attacker_sq),
        //     );

        //     println!("Ray:\n{}", ray);
        //     println!("Occupancy:\n{}", kingless_occupancy);
        //     println!("Both:\n{}", kingless_occupancy & ray);
        //     // check if the ray is blocked
        //     // if it is blocked, then we have a pinner
        //     // if it is not blocked, then we have a checker

        //     match (kingless_occupancy & ray).as_number().count_ones() {
        //         0 => {
        //             checkers |= Bitboard::from_square(attacker_sq);
        //         }
        //         1 => {
        //             pinned |= ray & our_pieces;
        //         }
        //         _ => {}
        //     }
        // }

        (checkers, pinned_pieces)
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
                println!("Right:\n{}", right);
                println!("Checkers:\n{}", checkers);
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
                let (mut checkers, _) = self.calculate_checkers_and_pinned_masks(board, &occupancy);
                println!("raw ep checkers:\n{}", checkers);
                // filter checkers to the same rank as the king
                let king_sq = bitboard_helpers::next_bit(
                    &mut board
                        .piece_bitboard(Piece::King, board.side_to_move())
                        .clone(),
                ) as u8;
                let king_rank = square::from_square(king_sq).1;
                checkers &= RANK_BITBOARDS[king_rank as usize];
                println!("ep checkers:\n{}", checkers);
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

        let mut pushes = match to_square {
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

        let mut pawn_pin_mask = if is_pinned {
            Bitboard::default()
        } else {
            Bitboard::from(u64::MAX)
        };

        let king_square =
            bitboard_helpers::next_bit(&mut board.piece_bitboard(Piece::King, us).clone()) as u8;

        // TODO: This is wrong, we need the pin ray to be the ray between the king and the pinning piece
        pawn_pin_mask |= self.ray_between(*square, Square::from_square_index(king_square))
            | Bitboard::from(*square)
            | Bitboard::from(king_square);

        println!("Pawn Pin Mask:\n{}", pawn_pin_mask);
        let en_passant_bb = self.calculate_en_passant_bitboard(from_square, board);

        // filter pushes by the occupany
        let legal_pushes = pushes & !occupancy;
        let attacks = self.pawn_attacks[us as usize][square.to_square_index() as usize]
            & (their_pieces | en_passant_bb);
        println!("Attacks:\n{}", attacks);
        (legal_pushes | attacks) & (*capture_mask | *push_mask) & pawn_pin_mask
    }

    fn generate_normal_piece_legal_mobility(
        &self,
        piece: Piece,
        square: &Square,
        board: &Board,
        capture_mask: &Bitboard,
        push_mask: &Bitboard,
    ) -> Bitboard {
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

        let our_pieces = board.pieces(us);
        let empty = !(their_pieces | our_pieces);
        // empty pushes or captures masked by capture mask and push mask in the case that we are
        // single checked
        mobility = (attacks & *capture_mask & their_pieces) | (attacks & empty & *push_mask);

        return mobility;
    }

    fn generate_legal_mobility(
        &self,
        piece: Piece,
        square: &Square,
        board: &Board,
        pinned_mask: &Bitboard,
        capture_mask: &Bitboard,
        push_mask: &Bitboard,
    ) -> Bitboard {
        match piece {
            Piece::Pawn => self.generate_legal_pawn_mobility(
                board,
                square,
                pinned_mask,
                capture_mask,
                push_mask,
            ),
            Piece::King => todo!("King mobility"),
            _ => self.generate_normal_piece_legal_mobility(
                piece,
                square,
                board,
                capture_mask,
                push_mask,
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

        let (checkers, pinned) = self.calculate_checkers_and_pinned_masks(board, &occupancy);

        let (capture_mask, push_mask) = self.calculate_capture_and_push_masks(board, &checkers);

        println!("Capture Mask:\n{}", capture_mask);
        println!("Push Mask:\n{}", push_mask);

        // generate king moves
        // calculate attacked squares
        let attacked_squares = self.get_attacked_squares(board, them, &(occupancy & !*king_bb));
        let king_moves_bb = self.get_non_slider_attacks(Piece::King, king_square);
        println!("{}", king_moves_bb);
        let king_pushes = king_moves_bb & !attacked_squares & !our_pieces & !their_pieces;
        let king_attacks = king_moves_bb & capture_mask & their_pieces;
        let mut king_moves = king_pushes | king_attacks;
        println!("attacked squares:\n{}", attacked_squares);
        println!("King Moves:\n{}", king_moves);

        while king_moves.as_number() > 0 {
            let to = bitboard_helpers::next_bit(&mut king_moves) as u8;
            let captured_piece = match board.piece_on_square(to) {
                Some((piece, _)) => Some(piece),
                None => None,
            };

            let mv = Move::new_king_move(
                &Square::from_square_index(king_square),
                &Square::from_square_index(to),
                captured_piece,
            );
            move_list.push(mv);
        }

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

            let mut moves = self.generate_legal_mobility(
                piece,
                &from_square,
                board,
                &pinned,
                &capture_mask,
                &push_mask,
            );

            println!("moves:\n{}", moves);
            self.enumerate_moves(&moves, &from_square, piece, board, move_list);
            // use capture mask if there is a checker
            // use push mask for non-captures
            // TODO
        }
    }

    /// Generates pseudo-legal moves for the current board state.
    /// This function does not check for legality of the moves.
    ///
    /// # Arguments
    /// - board - The current board state
    /// - move_list - The list of moves to append to.
    /// - move_type - The type of moves to generate
    pub fn generate_moves(&self, board: &Board, move_list: &mut MoveList, move_type: MoveType) {
        // get moves for each piece except pawns
        for piece in [
            Piece::King,
            Piece::Knight,
            Piece::Rook,
            Piece::Bishop,
            Piece::Queen,
        ] {
            self.get_piece_moves(piece, board, move_list, &move_type);
        }
        // handle pawn moves separately
        self.get_pawn_moves(board, move_list, &move_type);

        if move_type == MoveType::All || move_type == MoveType::Quiet {
            // handle castling moves
            self.get_castling_moves(board, move_list);
        }
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
        if board.can_castle_kingside(Side::White) && board.side_to_move() == Side::White {
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

        if board.can_castle_queenside(Side::White) && board.side_to_move() == Side::White {
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

        if board.can_castle_kingside(Side::Black) && board.side_to_move() == Side::Black {
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

        if board.can_castle_queenside(Side::Black) && board.side_to_move() == Side::Black {
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
                Piece::King | Piece::Knight => self.get_non_slider_attacks(piece, from_square),
                Piece::Rook | Piece::Bishop | Piece::Queen => {
                    self.get_slider_attacks(piece, from_square, &occupancy)
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

    pub(crate) fn get_non_slider_attacks(&self, piece: Piece, from_square: u8) -> Bitboard {
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

    fn get_slider_attacks(&self, piece: Piece, from_square: u8, occupancy: &Bitboard) -> Bitboard {
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

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
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
            let to_square = match us {
                Side::White => from_square as u64 + direction,
                Side::Black => from_square as u64 - direction,
                Side::Both => panic!("Both side not allowed"),
            };

            // pawn non-capture moves
            if *move_type == MoveType::All || *move_type == MoveType::Quiet {
                let bb_push = Bitboard::new(1u64 << to_square);
                let bb_single_push = bb_push & empty;
                let can_double_push = match us {
                    Side::White => Board::is_square_on_rank(from_square, Rank::R2 as u8),
                    Side::Black => Board::is_square_on_rank(from_square, Rank::R7 as u8),
                    Side::Both => panic!("Both side not allowed"),
                };

                let double_push_square = if can_double_push {
                    match us {
                        Side::White => {
                            let (value, did_overflow) = to_square.overflowing_add(direction);
                            if did_overflow {
                                None
                            } else {
                                Some(value)
                            }
                        }
                        Side::Black => {
                            let (value, did_overflow) = to_square.overflowing_sub(direction);
                            if did_overflow {
                                None
                            } else {
                                Some(value)
                            }
                        }
                        Side::Both => panic!("Both side not allowed"),
                    }
                } else {
                    None
                };

                // note that the single push square has to be empty in addition to the double push square being empty
                let is_double_push_unobstructed = if double_push_square.is_some() {
                    !occupancy.is_square_occupied(to_square as u8)
                        && !occupancy.is_square_occupied(double_push_square.unwrap() as u8)
                } else {
                    false
                };

                let bb_double_push = if can_double_push && is_double_push_unobstructed {
                    Bitboard::new(1u64 << double_push_square.unwrap()) & empty
                } else {
                    Bitboard::default()
                };
                bb_moves |= bb_single_push | bb_double_push;
            }

            // pawn captures
            if move_type == &MoveType::All || move_type == &MoveType::Capture {
                let bb_capture = attack_bb & their_pieces;
                // en passant
                let bb_en_passant = match board.en_passant_square() {
                    Some(en_passant_square) => {
                        // we only want to add the en passant square if it is within range of the pawn
                        // this means that the en passant square is within 1 rank of the pawn and the en passant square
                        // is in the pawn's attack table
                        let en_passant_bb = Bitboard::from_square(en_passant_square);
                        let result = en_passant_bb & !(attack_bb);
                        let is_in_range = result == 0;
                        if is_in_range {
                            en_passant_bb
                        } else {
                            Bitboard::default()
                        }
                    }
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

            let en_passant = match board.en_passant_square() {
                Some(en_passant_square) => en_passant_square == to_square && piece == Piece::Pawn,
                None => false,
            };

            let is_capture: bool = enemy_pieces.is_square_occupied(to_square) || en_passant;
            // 2 rows = 16 squares
            let is_double_move = piece == Piece::Pawn
                && (to_square as i8 - from.to_square_index() as i8).abs() == 16;
            let is_promotion =
                piece == Piece::Pawn && Board::is_square_on_rank(to_square, promotion_rank as u8);

            if is_double_move && en_passant {
                panic!("Double move and en passant should not happen");
            }

            let mut move_desc = MoveDescriptor::None;
            if is_double_move {
                move_desc = MoveDescriptor::PawnTwoUp;
            } else if en_passant {
                move_desc = MoveDescriptor::EnPassantCapture;
            }

            let capture_piece = if is_capture && !en_passant {
                Some(board.piece_on_square(to_square).unwrap().0)
            } else if en_passant {
                Some(Piece::Pawn)
            } else {
                None
            };

            let to_square = square::to_square_object(file, rank);
            if is_promotion {
                // we have to add 4 moves for each promotion type
                for promotion_type in [
                    PromotionDescriptor::Queen,
                    PromotionDescriptor::Rook,
                    PromotionDescriptor::Bishop,
                    PromotionDescriptor::Knight,
                ] {
                    let mv = Move::new(
                        &from,
                        &to_square,
                        move_desc,
                        piece,
                        capture_piece,
                        Some(promotion_type.to_piece()),
                    );
                    move_list.push(mv);
                }
            } else {
                let mv = Move::new(&from, &to_square, move_desc, piece, capture_piece, None);
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
    pub fn is_square_attacked_with_occupancy(
        &self,
        board: &Board,
        square: &Square,
        attacking_side: Side,
        occupancy: &Bitboard,
    ) -> bool {
        let king_bb = board.piece_bitboard(Piece::King, attacking_side);
        let knight_bb = board.piece_bitboard(Piece::Knight, attacking_side);
        let bishop_bb = board.piece_bitboard(Piece::Bishop, attacking_side);
        let rook_bb = board.piece_bitboard(Piece::Rook, attacking_side);
        let queen_bb = board.piece_bitboard(Piece::Queen, attacking_side);
        let pawn_bb: &Bitboard = board.piece_bitboard(Piece::Pawn, attacking_side);

        let king_attacks = self.get_non_slider_attacks(Piece::King, square.to_square_index());
        let knight_attacks = self.get_non_slider_attacks(Piece::Knight, square.to_square_index());
        let rook_attacks =
            self.get_slider_attacks(Piece::Rook, square.to_square_index(), &occupancy);
        let bishop_attacks =
            self.get_slider_attacks(Piece::Bishop, square.to_square_index(), &occupancy);
        let queen_attacks = rook_attacks | bishop_attacks;
        // note we use the opposite side for the pawn attacks
        let pawn_attacks = self.pawn_attacks[Side::opposite(attacking_side) as usize]
            [square.to_square_index() as usize];

        let is_king_attacker = (king_attacks & *king_bb) > 0;
        let is_knight_attacker = (knight_attacks & *knight_bb) > 0;
        let is_rook_attacker = (rook_attacks & *rook_bb) > 0;
        let is_bishop_attacker = (bishop_attacks & *bishop_bb) > 0;
        let is_queen_attacker = (queen_attacks & *queen_bb) > 0;
        let is_pawn_attacker = (pawn_attacks & *pawn_bb) > 0;

        return is_king_attacker
            || is_knight_attacker
            || is_rook_attacker
            || is_bishop_attacker
            || is_queen_attacker
            || is_pawn_attacker;
    }

    pub fn is_square_attacked(&self, board: &Board, square: &Square, attacking_side: Side) -> bool {
        return self.is_square_attacked_with_occupancy(
            board,
            square,
            attacking_side,
            &board.all_pieces(),
        );
    }
}

#[cfg(test)]
mod tests {

    use crate::move_generation;

    use super::*;
    #[test]
    fn calculate_pinned_pieces() {
        let move_gen = MoveGenerator::new();
        let board =
            Board::from_fen("2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2")
                .unwrap();
        let occupancy = board.all_pieces();
        let (checkers, pinned) = move_gen.calculate_checkers_and_pinned_masks(&board, &occupancy);
        assert_eq!(checkers, 0);
        assert_eq!(pinned, Bitboard::from_square(Squares::D7));
    }

    #[test]
    fn calculate_pinned_pieces_2() {
        let move_gen = MoveGenerator::new();
        let board = Board::from_fen("8/8/8/8/k2Pp2Q/8/8/3K4 b - d3 0 1").unwrap();
        let occupancy = board.all_pieces();
        let (checkers, pinned) = move_gen.calculate_checkers_and_pinned_masks(&board, &occupancy);
        assert_eq!(checkers, 0);
        assert_eq!(pinned, Bitboard::default());
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
        move_gen.generate_moves(&board, &mut move_list, MoveType::All);
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

        {
            let board = Board::from_fen("r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2").unwrap();
            let mut king_bb = *board.piece_bitboard(Piece::King, board.side_to_move());
            let square = bitboard_helpers::next_bit(&mut king_bb) as u8;
            assert_eq!(board.side_to_move(), Side::Black);
            assert!(move_gen.is_square_attacked(
                &board,
                &Square::from_square_index(square),
                Side::opposite(board.side_to_move())
            ));
        }

        {
            let mut board =
                Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")
                    .unwrap();
            move_gen.generate_moves(&board, &mut move_list, MoveType::All);
            let mv = move_list
                .iter()
                .find(|mv| mv.to_long_algebraic() == "b1c3")
                .unwrap();
            assert!(board.make_move(mv, &move_gen).is_ok());

            // did we leave the king in check?
            let mut king_bb = *board.piece_bitboard(Piece::King, Side::White);
            let square = bitboard_helpers::next_bit(&mut king_bb) as u8;
            assert_eq!(board.side_to_move(), Side::Black);
            // there should be no attacks on the king
            assert!(!move_gen.is_square_attacked(
                &board,
                &Square::from_square_index(square),
                Side::Black
            ));
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
            33816580,
            84410376,
            168886289,
            337772578,
            675545156,
            1351090312,
            2685403152,
            1075839008,
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
            288234782788157440,
            576469569871282176,
            1224997833292120064,
            2449995666584240128,
            4899991333168480256,
            9799982666336960512,
            1152939783987658752,
            2305878468463689728,
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
            let rook_bits = move_generation::MoveGenerator::relevant_rook_bits(square as u8);
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
            let bishop_bits = move_generation::MoveGenerator::relevant_bishop_bits(square as u8);
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
            let rook_bb = MoveGenerator::relevant_rook_bits(sq as u8);
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
            let rook_bb = MoveGenerator::relevant_rook_bits(square as u8);
            let blockers = MoveGenerator::create_blocker_permutations(rook_bb);
            let edges = MoveGenerator::edges_from_square(square as u8);
            let rook_bb_with_edges = rook_bb | edges;

            let attacks = MoveGenerator::rook_attacks(square as u8, &blockers);
            assert!(attacks.len() <= blockers.len());

            for attack in attacks {
                // attack should be a subset of the rook bitboard with edges
                // blockers does not include the edges
                // but attacks do include them
                assert_eq!(attack & !rook_bb_with_edges, 0);
            }
        }
    }

    #[test]
    fn check_bishop_attacks() {
        for square in 0..1 {
            let bishop_bb = MoveGenerator::relevant_bishop_bits(square as u8);
            let blockers = MoveGenerator::create_blocker_permutations(bishop_bb);
            let edges = MoveGenerator::edges_from_square(square as u8);
            let bishop_bb_with_edges = bishop_bb | edges;

            let attacks = MoveGenerator::bishop_attacks(square as u8, &blockers);
            assert!(attacks.len() <= blockers.len());

            for attack in attacks {
                println!("attack: \n{}", attack);
                // attack should be a subset of the bishop bitboard
                assert_eq!(attack & !bishop_bb_with_edges, 0);
            }
        }
    }

    #[test]
    fn check_basic_move_gen() {
        let board = Board::default_board();
        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&board, &mut move_list, MoveType::All);

        for mv in move_list.iter() {
            println!("{}", mv);
            assert!(!mv.is_castle());
            assert!(!mv.is_en_passant_capture());
            assert!(!mv.is_promotion());
        }

        assert_eq!(move_list.len(), 20);

        move_list.clear();
        move_gen.generate_legal_moves(&board, &mut move_list);

        for mv in move_list.iter() {
            println!("{}", mv);
        }
        assert_eq!(move_list.len(), 20);
    }

    #[test]
    fn check_en_passant_capture_move_gen() {
        let board = Board::from_fen("8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0 3").unwrap();
        assert!(board.en_passant_square().is_some());

        assert_eq!(board.side_to_move(), Side::Black);
        let mut move_list = MoveList::new();
        let move_gen = MoveGenerator::new();
        move_gen.generate_moves(&board, &mut move_list, MoveType::All);
        let en_passant_move = move_list.iter().find(|mv| mv.is_en_passant_capture());
        assert!(en_passant_move.is_some());
        assert!(move_list.len() >= 8);
    }
}
