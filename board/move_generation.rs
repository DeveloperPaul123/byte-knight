/*
 * move_generation.rs
 * Part of the byte-knight project
 * Created Date: Wednesday, August 28th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Thu Aug 29 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use serde::de::value;

use crate::{
    bitboard::Bitboard,
    definitions::{File, NumberOf, Rank, Side},
    square,
};

type FileBitboards = [Bitboard; NumberOf::FILES];
type RankBitboards = [Bitboard; NumberOf::RANKS];

const fn initialize_file_bitboards() -> FileBitboards {
    let mut file_bitboards = [Bitboard::default(); NumberOf::FILES];
    let mut i = 0;
    let file_a_bb: u64 = 0x101010101010101;
    while i < NumberOf::FILES {
        file_bitboards[i] = Bitboard::new(file_a_bb << i as u64);
        i += 1;
    }
    return file_bitboards;
}

const fn initialize_rank_bitboards() -> RankBitboards {
    let mut rank_bitboards = [Bitboard::default(); NumberOf::RANKS];
    let rank_1_bb = 0xFF;
    let mut i = 0;
    while i < NumberOf::RANKS {
        rank_bitboards[i] = Bitboard::new(rank_1_bb << ((i * 8) as u64));
        i += 1;
    }
    return rank_bitboards;
}

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
    for sq in 0..NumberOf::SQUARES {
        let mut bb = Bitboard::default();
        bb.set_square(sq);

        let mut attacks_w_bb = Bitboard::default();
        let mut attacks_b_bb = Bitboard::default();

        let not_a_file = !FILE_BITBOARDS[File::A as usize];
        let not_h_file = !FILE_BITBOARDS[File::H as usize];

        // white is NORTH_WEST and NORTH_EAST
        attacks_w_bb |= (bb & not_a_file) << NORTH_WEST;
        attacks_w_bb |= (bb & not_h_file) << NORTH_EAST;

        attacks_b_bb |= (bb & not_a_file) >> SOUTH_WEST;
        attacks_b_bb |= (bb & not_h_file) >> SOUTH_EAST;

        attacks[Side::White as usize][sq] = attacks_w_bb;
        attacks[Side::Black as usize][sq] = attacks_b_bb;
    }
}

pub struct MoveGenerator {
    king_attacks: [Bitboard; NumberOf::SQUARES],
    knight_attacks: [Bitboard; NumberOf::SQUARES],
    pawn_attacks: [[Bitboard; NumberOf::SQUARES]; NumberOf::SIDES],
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

    fn relevant_rook_bits(square: usize) -> Bitboard {
        let mut bb = Bitboard::default();
        bb.set_square(square);

        // need to get the rank and file of the square
        let (file, rank) = square::from_square(square as u8);
        let file_bb = FILE_BITBOARDS[file as usize];
        let rank_bb = RANK_BITBOARDS[rank as usize];
        let rook_rays_bb = MoveGenerator::orthogonal_ray_attacks(square as u8, 0);
        // get the edges of the board
        let edges = MoveGenerator::edges(file, rank);

        return rook_rays_bb & !edges & !bb;
    }

    fn relevant_bishop_bits(square: usize) -> Bitboard {
        let mut bb = Bitboard::default();
        bb.set_square(square);

        let (file, rank) = square::from_square(square as u8);
        let edges = MoveGenerator::edges(file, rank);

        // need to calculate ray attacks for the bishop from its square
        let bishop_ray_attacks = MoveGenerator::diagonal_ray_attacks(square as u8, 0);

        return bishop_ray_attacks & !edges & !bb;
    }
}

#[cfg(test)]
mod tests {
    use crate::{definitions::Squares, move_generation, square};

    use super::*;

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

        for square in 0..NumberOf::SQUARES {
            let rook_bits = move_generation::MoveGenerator::relevant_rook_bits(square);
            assert_eq!(rook_bits.as_number(), rook_relevant_bit_expected[square]);
        }
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
        for square in 0..NumberOf::SQUARES {
            let bishop_bits = move_generation::MoveGenerator::relevant_bishop_bits(square);
            assert_eq!(
                bishop_bits.as_number(),
                bishop_relevant_bit_expected[square]
            );
        }
    }
}
