use crate::{
    bitboard::Bitboard,
    definitions::NumberOf,
    magics::{BISHOP_MAGIC_VALUES, MagicNumber, ROOK_MAGIC_VALUES},
    move_generation::MoveGenerator,
    pext::{Pext, has_bmi2},
    pieces::{Piece, SQUARE_NAME},
    slider_pieces::SliderPiece,
};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64;

pub(crate) struct SlidingPieceAttacks {
    pub(crate) rook_magics: [MagicNumber; NumberOf::SQUARES],
    pub(crate) bishop_magics: [MagicNumber; NumberOf::SQUARES],
    pub(crate) rook_attacks: Vec<Bitboard>,
    pub(crate) bishop_attacks: Vec<Bitboard>,
    pub(crate) rook_pext: [Pext; NumberOf::SQUARES],
    pub(crate) bishop_pext: [Pext; NumberOf::SQUARES],
    pub(crate) rook_pext_attacks: Vec<Bitboard>,
    pub(crate) bishop_pext_attacks: Vec<Bitboard>,
}

// Public API
impl SlidingPieceAttacks {
    /// Create a new instance of SlidingPieceAttacks with initialized magic numbers and attack tables.
    pub(crate) fn new() -> Self {
        let mut instance = SlidingPieceAttacks {
            rook_magics: [MagicNumber::default(); NumberOf::SQUARES],
            bishop_magics: [MagicNumber::default(); NumberOf::SQUARES],
            rook_attacks: vec![Bitboard::default(); 102400], // 2^12 * 64
            bishop_attacks: vec![Bitboard::default(); 5248], // 2^10 * 64
            rook_pext: [Pext::default(); NumberOf::SQUARES],
            bishop_pext: [Pext::default(); NumberOf::SQUARES],
            rook_pext_attacks: vec![Bitboard::default(); 102400], // 2^12 * 64
            bishop_pext_attacks: vec![Bitboard::default(); 5248], // 2^10 * 64
        };

        // Initialize the magic numbers and attack tables.
        instance.initialize_magic_numbers(Piece::Rook);
        instance.initialize_magic_numbers(Piece::Bishop);

        // Initialize the PEXT tables if BMI2 is available.
        if has_bmi2() {
            instance.initialize_pext_tables(SliderPiece::Rook);
            instance.initialize_pext_tables(SliderPiece::Bishop);
        }

        instance
    }

    /// Get the attack bitboard for a sliding piece (rook, bishop, or queen) from a given square,
    /// considering the current occupancy of the board.
    ///
    /// # Arguments
    ///
    /// - `piece` - The type of sliding piece (rook, bishop, or queen).
    /// - `from_square` - The square from which the piece is attacking (0-63).
    /// - `occupancy` - The current occupancy bitboard of the board.
    ///
    /// # Returns
    ///
    /// - A Bitboard representing the attack squares for the given piece from the specified square.
    pub(crate) fn get_slider_attack(
        &self,
        piece: SliderPiece,
        from_square: u8,
        occupancy: &Bitboard,
    ) -> Bitboard {
        if has_bmi2() {
            self.get_attack_pext(piece, from_square, occupancy)
        } else {
            self.get_attack(piece, from_square, occupancy)
        }
    }
}

// Private functions
impl SlidingPieceAttacks {
    #[cfg(target_arch = "x86_64")]
    fn initialize_pext_tables(&mut self, piece: SliderPiece) {
        use std::i64;
        let mut offset = 0usize;

        assert!(piece == SliderPiece::Bishop || piece == SliderPiece::Rook);
        let relevant_bits_fn = if piece == SliderPiece::Rook {
            MoveGenerator::relevant_rook_bits
        } else {
            MoveGenerator::relevant_bishop_bits
        };
        let get_attacks_fn = if piece == SliderPiece::Rook {
            MoveGenerator::rook_attacks
        } else {
            MoveGenerator::bishop_attacks
        };

        let mut min_pext = i64::MAX;
        let mut max_pext = i64::MIN;
        for square in 0..NumberOf::SQUARES as u8 {
            let relevant_bits = relevant_bits_fn(square);

            let blocker_bitboards = MoveGenerator::create_blocker_permutations(relevant_bits);
            let attacks = get_attacks_fn(square, &blocker_bitboards);
            let attack_table = if piece == SliderPiece::Rook {
                &mut self.rook_pext_attacks
            } else {
                &mut self.bishop_pext_attacks
            };

            let pext_table = if piece == SliderPiece::Rook {
                &mut self.rook_pext
            } else {
                &mut self.bishop_pext
            };

            let total_permutations = blocker_bitboards.len();

            pext_table[square as usize] = Pext::new(relevant_bits, offset);
            let current_pext = pext_table[square as usize];
            for i in 0..blocker_bitboards.len() {
                let blocker = blocker_bitboards[i];
                let index = current_pext.index(&blocker);
                if (index as i64) < min_pext {
                    min_pext = index as i64;
                }
                if (index as i64) > max_pext {
                    max_pext = index as i64;
                }
                assert!(index < attack_table.len());

                attack_table[index] = attacks[i];
            } // blocker bitboards loop

            offset += total_permutations;
            println!("Min/max pext index {min_pext}/{max_pext}");
        }
    }

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

    #[inline(always)]
    fn get_attack(&self, piece: SliderPiece, from_square: u8, occupancy: &Bitboard) -> Bitboard {
        match piece {
            SliderPiece::Rook => {
                let index = self.rook_magics[from_square as usize].index(*occupancy);
                self.rook_attacks[index]
            }
            SliderPiece::Bishop => {
                let index = self.bishop_magics[from_square as usize].index(*occupancy);
                self.bishop_attacks[index]
            }
            SliderPiece::Queen => {
                let rook_index = self.rook_magics[from_square as usize].index(*occupancy);
                let bishop_index = self.bishop_magics[from_square as usize].index(*occupancy);
                self.rook_attacks[rook_index] ^ self.bishop_attacks[bishop_index]
            }
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    fn get_attack_pext(
        &self,
        piece: SliderPiece,
        from_square: u8,
        occupancy: &Bitboard,
    ) -> Bitboard {
        return match piece {
            SliderPiece::Rook => {
                let pext = self.rook_pext[from_square as usize];
                let index = pext.index(occupancy);
                self.rook_pext_attacks[index]
            }
            SliderPiece::Bishop => {
                let pext = self.bishop_pext[from_square as usize];
                let index = pext.index(occupancy);
                self.bishop_pext_attacks[index]
            }
            SliderPiece::Queen => {
                let rook_index = self.rook_pext[from_square as usize].index(occupancy);
                let bishop_index = self.bishop_pext[from_square as usize].index(occupancy);
                self.rook_pext_attacks[rook_index] ^ self.bishop_pext_attacks[bishop_index]
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bitboard::Bitboard,
        definitions::{FILE_BITBOARDS, RANK_BITBOARDS, Squares},
        file::File,
        move_generation::MoveGenerator,
        rank::Rank,
        slider_pieces::SliderPiece,
    };

    #[test]
    fn check_queen_attacks() {
        let square = Squares::D8;
        let bishop_bb = MoveGenerator::relevant_bishop_bits(square);
        let rook_bb = MoveGenerator::relevant_rook_bits(square);
        let queen_bb = bishop_bb | rook_bb;

        let sliding_piece_attacks = super::SlidingPieceAttacks::new();
        let queen_attacks =
            sliding_piece_attacks.get_attack(SliderPiece::Queen, square, &Bitboard::default());
        println!("queen attacks: \n{queen_attacks}");
        println!("queen bb: \n{queen_bb}");

        let attacks_without_edges = queen_attacks
            & !FILE_BITBOARDS[File::A as usize]
            & !FILE_BITBOARDS[File::H as usize]
            & !RANK_BITBOARDS[Rank::R1 as usize];

        println!("attacks without edges: \n{attacks_without_edges}");
        assert_eq!(attacks_without_edges, queen_bb);
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn check_queen_attacks_pext() {
        let square = Squares::D8;
        let bishop_bb = MoveGenerator::relevant_bishop_bits(square);
        let rook_bb = MoveGenerator::relevant_rook_bits(square);
        let queen_bb = bishop_bb | rook_bb;

        let sliding_piece_attacks = super::SlidingPieceAttacks::new();
        let queen_attacks =
            sliding_piece_attacks.get_attack_pext(SliderPiece::Queen, square, &Bitboard::default());
        println!("queen attacks: \n{queen_attacks}");
        println!("queen bb: \n{queen_bb}");

        let attacks_without_edges = queen_attacks
            & !FILE_BITBOARDS[File::A as usize]
            & !FILE_BITBOARDS[File::H as usize]
            & !RANK_BITBOARDS[Rank::R1 as usize];

        println!("attacks without edges: \n{attacks_without_edges}");
        assert_eq!(attacks_without_edges, queen_bb);
    }
}
