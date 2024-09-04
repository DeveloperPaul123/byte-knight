/*
 * magics.rs
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

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::bitboard::Bitboard;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MagicNumber {
    pub relevant_bits_mask: u64,
    pub shift: u8,
    pub offset: u64,
    pub magic_value: u64,
}

impl MagicNumber {
    pub fn new(relevant_bits_mask: Bitboard, shift: u8, offset: u64, magic_value: u64) -> Self {
        MagicNumber {
            relevant_bits_mask: relevant_bits_mask.as_number(),
            shift: shift,
            offset: offset,
            magic_value: magic_value,
        }
    }
    pub fn default() -> Self {
        MagicNumber {
            relevant_bits_mask: 0,
            shift: 0,
            offset: 0,
            magic_value: 0,
        }
    }

    /// Returns the index of the magic number in the table.
    /// This is basically the same formula used to calculate magic numbers, but it's just missing the magic value.
    /// We take into account the shift and offset to calculate the index without the magic value.
    pub fn index(&self, occupancy: Bitboard) -> usize {
        let blockers = occupancy & self.relevant_bits_mask;
        // need to shift
        let blocker_num = blockers.as_number();
        return ((blocker_num.wrapping_mul(self.magic_value) >> self.shift) + self.offset) as usize;
    }
}

impl Display for MagicNumber {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "bb {:24} shift {:4} offset {:6} magic {:24}",
            self.relevant_bits_mask, self.shift, self.offset, self.magic_value
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{definitions::Squares, move_generation::MoveGenerator};

    use super::*;

    #[test]
    fn test_magic_number_index() {
        // test a1 for the rook
        let relevant_bits = MoveGenerator::relevant_rook_bits(Squares::A1 as usize);
        let blockers = MoveGenerator::create_blocker_permutations(relevant_bits);
        let magic_value = 684547693657194778;
        let magic = MagicNumber::new(
            relevant_bits,
            (64 - relevant_bits.as_number().count_ones()) as u8,
            0,
            magic_value,
        );
        let blocker = blockers[0];
        assert_eq!(magic.index(blocker), 0);

        let mut indexes = Vec::with_capacity(blockers.len());
        for blocker in blockers {
            let index = magic.index(blocker);
            indexes.push(index);
        }
    }
}
