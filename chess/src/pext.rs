/*
 * pext.rs
 * Part of the byte-knight project
 * Created Date: Tuesday, July 8, 2025
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Tues July 8 2025
 * -----
 * Copyright (c) 2024-2025 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use crate::bitboard::Bitboard;

/// Checks to see if the BMI2 instructions set is available on the current machine.
///
/// Returns false if unavailable and true otherwise.
pub(crate) fn has_bmi2() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        is_x86_feature_detected!("bmi2")
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct Pext {
    pub relevant_bits_mask: u64,
    pub offset: usize,
}

impl Pext {
    pub(crate) fn new(relavent_bits: Bitboard, offset: usize) -> Self {
        Pext {
            relevant_bits_mask: relavent_bits.as_number(),
            offset,
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub(crate) fn index(&self, occupancy: &Bitboard) -> usize {
        unsafe {
            use std::arch::x86_64::_pext_u64;
            let index = _pext_u64(occupancy.as_number(), self.relevant_bits_mask) as usize;
            index + self.offset
        }
    }
}
