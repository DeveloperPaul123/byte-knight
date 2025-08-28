use crate::{
    bitboard::Bitboard, definitions::NumberOf, magics::MagicNumber, move_generation::MoveGenerator,
    pieces::Piece,
};

struct SlidingPieceAttacks {
    pub(crate) rook_magics: [MagicNumber; NumberOf::SQUARES],
    pub(crate) bishop_magics: [MagicNumber; NumberOf::SQUARES],
    pub(crate) rook_attacks: Vec<Bitboard>,
    pub(crate) bishop_attacks: Vec<Bitboard>,
    pub(crate) rook_pext_attacks: Vec<Bitboard>,
    pub(crate) bishop_pext_attacks: Vec<Bitboard>,
}

// Public API
impl SlidingPieceAttacks {
    pub fn new() {
        let mut instance = SlidingPieceAttacks {
            rook_magics: [MagicNumber::default(); NumberOf::SQUARES],
            bishop_magics: [MagicNumber::default(); NumberOf::SQUARES],
            rook_attacks: vec![Bitboard::default(); 102400], // 2^12 * 64
            bishop_attacks: vec![Bitboard::default(); 5248], // 2^10 * 64
            rook_pext_attacks: vec![Bitboard::default(); 4096], // 2^12
            bishop_pext_attacks: vec![Bitboard::default(); 1024], // 2^10
        };

        instance.initialize_magic_numbers(Piece::Rook);
        instance.initialize_magic_numbers(Piece::Bishop);

        // TODO: Initialize PEXT attack tables
        for square in 0..NumberOf::SQUARES as u8 {
            let rook_relevant_bits = MoveGenerator::relevant_rook_bits(square);
            let bishop_relevant_bits = MoveGenerator::relevant_bishop_bits(square);

            let rook_bit_count = rook_relevant_bits.as_number().count_ones();
            let bishop_bit_count = bishop_relevant_bits.as_number().count_ones();

            let rook_total_permutations = 2u64.pow(rook_bit_count);
            let bishop_total_permutations = 2u64.pow(bishop_bit_count);

            let rook_blocker_bitboards =
                MoveGenerator::create_blocker_permutations(rook_relevant_bits);
            let bishop_blocker_bitboards =
                MoveGenerator::create_blocker_permutations(bishop_relevant_bits);

            let rook_attacks = MoveGenerator::rook_attacks(square, &rook_blocker_bitboards);
            let bishop_attacks = MoveGenerator::bishop_attacks(square, &bishop_blocker_bitboards);

            for i in 0..rook_blocker_bitboards.len() {
                let blocker = rook_blocker_bitboards[i];
                let index = blocker.pext(rook_relevant_bits) as usize;
                assert!(index < instance.rook_pext_attacks.len());
                instance.rook_pext_attacks[index] = rook_attacks[i];
            }

            for i in 0..bishop_blocker_bitboards.len() {
                let blocker = bishop_blocker_bitboards[i];
                let index = blocker.pext(bishop_relevant_bits) as usize;
                assert!(index < instance.bishop_pext_attacks.len());
                instance.bishop_pext_attacks[index] = bishop_attacks[i];
            }
        }
    }
}

// Private functions
impl SlidingPieceAttacks {
    /// Initialize the magic numbers and the associated attack boards.
    #[allow(clippy::panic)]
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

            let rook_attacks = MoveGenerator::rook_attacks(square, &blocker_bitboards);
            let bishop_attacks = MoveGenerator::bishop_attacks(square, &blocker_bitboards);
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
                    panic!(
                        "Collision detected while initializing attack tables for piece {:?} and square {} - \n\t{}",
                        piece, SQUARE_NAME[square as usize], magics[square as usize]
                    );
                }
            }

            // update the offset for the next square
            offset += total_permutations;
        }
    }
}
