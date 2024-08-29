use chess::ALL_FILES;

use crate::{bitboard::Bitboard, definitions::NumberOf};

/**
 * Not H file
 */
const NOT_A_FILE: u64 = 0x7F7F7F7F7F7F7F7F;
const NOT_H_FILE: u64 = 0xFEFEFEFEFEFEFEFE;
const NOT_RANK_1: u64 = 0xFFFFFFFFFFFFFF00;
const NOT_RANK_8: u64 = 0xFFFFFFFFFFFFFF;

const NORTH: u64 = 8;
const SOUTH: u64 = 8;
const NORTH_NORTH: u64 = 16;
const WEST: u64 = 1;
const EAST: u64 = 1;
const NORTH_EAST: u64 = 9;
const NORTH_WEST: u64 = 7;
const SOUTH_EAST: u64 = 7;
const SOUTH_WEST: u64 = 9;

fn initialize_king_attacks(square: usize, attacks: &mut [Bitboard; NumberOf::SQUARES]) {
    let mut bb = Bitboard::default();
    let mut attacks_bb = Bitboard::default();

    // king can move 1 square in any direction
    bb.set_square(square);

    // with our bit board setup, "east" means right, and "west" means left
    // so this means east means we move more towards the MSB, so shift.
    // So all the east and north moves are shifted left, all south and west moves are shifted right
    if ((bb << EAST) & NOT_H_FILE) > (0 as u64) {
        attacks_bb |= bb << EAST;
    }
    if (bb << NORTH_EAST) & NOT_H_FILE > (0 as u64) {
        attacks_bb |= bb << NORTH_EAST;
    }
    if ((bb << NORTH_WEST) & NOT_A_FILE) > (0 as u64) {
        attacks_bb |= bb << NORTH_WEST;
    }
    if bb << NORTH > (0 as u64) {
        attacks_bb |= bb << NORTH;
    }

    if ((bb >> WEST) & NOT_A_FILE) > (0 as u64) {
        attacks_bb |= bb >> WEST;
    }
    if (bb >> SOUTH_EAST) & NOT_H_FILE > (0 as u64) {
        attacks_bb |= bb >> SOUTH_EAST;
    }

    if (bb >> SOUTH_WEST) & NOT_A_FILE > (0 as u64) {
        attacks_bb |= bb >> SOUTH_WEST;
    }
    if bb >> SOUTH > (0 as u64) {
        attacks_bb |= bb >> SOUTH;
    }

    attacks[square as usize] = attacks_bb;
}

pub struct MoveGenerator {
    king_attacks: [Bitboard; NumberOf::SQUARES],
    knight_attacks: [Bitboard; NumberOf::SQUARES],
    pawn_attacks: [Bitboard; NumberOf::SQUARES],
}

impl MoveGenerator {
    pub fn new() -> Self {
        let king_attacks = [Bitboard::default(); NumberOf::SQUARES];
        let knight_attacks = [Bitboard::default(); NumberOf::SQUARES];
        let pawn_attacks = [Bitboard::default(); NumberOf::SQUARES];
        let mut move_gen = Self {
            king_attacks,
            knight_attacks,
            pawn_attacks,
        };

        move_gen.initialize_attack_boards();
        return move_gen;
    }

    fn initialize_attack_boards(&mut self) {
        for square in 0..NumberOf::SQUARES {
            initialize_king_attacks(square, &mut self.king_attacks);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::definitions::Squares;

    use super::*;

    #[test]
    fn initialize_attack_boards() {
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
}
