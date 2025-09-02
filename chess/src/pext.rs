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

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64;

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
