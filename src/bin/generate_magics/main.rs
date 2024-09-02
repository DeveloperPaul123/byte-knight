/*
 * main.rs
 * Part of the byte-knight project
 * Created Date: Friday, August 30th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Sun Sep 01 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use byte_board::{
    bitboard::Bitboard,
    definitions::{NumberOf, BISHOP_BLOCKER_PERMUTATIONS, ROOK_BLOCKER_PERMUTATIONS},
    magics::MagicNumber,
    move_generation::MoveGenerator,
    pieces::Piece,
    square,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

fn random_u64<rng: Rng>(rng: &mut rng) -> u64 {
    let u1 = rng.gen::<u64>() & 0xFFFF;
    let u2 = rng.gen::<u64>() & 0xFFFF;
    let u3 = rng.gen::<u64>() & 0xFFFF;
    let u4 = rng.gen::<u64>() & 0xFFFF;

    return u1 | (u2 << 16) | (u3 << 32) | (u4 << 48);
}

fn random_u64_few_bits<rng: Rng>(rng: &mut rng) -> u64 {
    return random_u64(rng) & random_u64(rng) & random_u64(rng);
}

fn find_magic_number(square: u8, piece: Piece) -> MagicNumber {
    // TODO
    return MagicNumber::default();
}

fn find_magic_numbers(piece: Piece) -> Vec<MagicNumber> {
    let mut rng = ChaChaRng::from_entropy();
    let mut magic_numbers = Vec::with_capacity(NumberOf::SQUARES);
    assert!(piece == Piece::Rook || piece == Piece::Bishop);
    let mut rook_hash_table = vec![Bitboard::default(); ROOK_BLOCKER_PERMUTATIONS];
    let mut bishop_hash_table = vec![Bitboard::default(); BISHOP_BLOCKER_PERMUTATIONS];
    const BASE: u64 = 2 as u64;
    
    let mut offset = 0;

    for sq in 0..NumberOf::SQUARES {
        let rook_mask = MoveGenerator::relevant_rook_bits(sq);
        let bishop_mask = MoveGenerator::relevant_bishop_bits(sq);

        let use_mask = if piece == Piece::Rook {
            rook_mask
        } else {
            bishop_mask
        };

        let bit_count = use_mask.as_number().count_ones();
        let total_permutations = BASE.pow(bit_count);
        let end = offset + total_permutations - 1u64;
        let blocker_bitboards = MoveGenerator::create_blocker_permutations(use_mask);
        assert_eq!(blocker_bitboards.len(), total_permutations as usize);

        let rook_attacks = MoveGenerator::rook_attacks(sq as u8, &blocker_bitboards);
        let bishop_attacks = MoveGenerator::bishop_attacks(sq as u8, &blocker_bitboards);
        let attacks = if piece == Piece::Rook {
            rook_attacks
        } else {
            bishop_attacks
        };
        assert_eq!(attacks.len(), blocker_bitboards.len());

        // flag that stops the loop
        let mut found = false;
        // Seems like we have to use a while loop with a flag here.
        // Using `loop` and `break` didn't work here and caused the loop to break early or never end.

        let mut magic = MagicNumber::new(use_mask, (64 - bit_count) as u8, offset, 0);

        while !found {
            // generate a random magic value
            // we & the values together to try and reduce the number of bits
            let mgc = random_u64_few_bits(&mut rng);
            let test = (use_mask.as_number() * mgc) & 0xFF00000000000000;
            if test.count_ones() < 6 {
                continue;
            }

            magic.magic_value = mgc;
            // let's be optimistic
            found = true;
            for i in 0..blocker_bitboards.len() {
                let index = magic.index(blocker_bitboards[i]);
                // figure out what table to insert the value into
                let table: &mut [Bitboard] = if piece == Piece::Rook {
                    &mut rook_hash_table
                } else {
                    &mut bishop_hash_table
                };

                if table[index] == Bitboard::default() {
                    assert!(
                        index as u64 >= offset && index as u64 <= end,
                        "index out of bounds"
                    );

                    table[index] = attacks[i];
                } else {
                    // Non-empty value found in the table so we have a key collision.
                    // No bueno because the magic value doesn't work. Time to try again.
                    // First we clean up
                    for tbl_index in offset..end {
                        table[tbl_index as usize] = Bitboard::default();
                    }
                    found = false;
                    break;
                }
            }
        }
        println!("{}", magic);
        magic_numbers.push(magic);
        offset += total_permutations;
    }

    // the offset should match the table size
    let total = if piece == Piece::Rook {
        ROOK_BLOCKER_PERMUTATIONS
    } else {
        BISHOP_BLOCKER_PERMUTATIONS
    };
    assert_eq!(offset, total as u64, "Permutations were skipped.");

    return magic_numbers;
}
fn main() {
    let magic_rook_numbers = find_magic_numbers(Piece::Rook);
    println!("");
    let magic_bishop_numbers = find_magic_numbers(Piece::Bishop);

    // output the magic numbers to a csv
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    for magic in magic_bishop_numbers {
        writer.serialize(magic).expect("Failed to write to csv");
    }

    for magic in magic_rook_numbers {
        writer.serialize(magic).expect("Failed to write to csv");
    }
}
