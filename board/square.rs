/*
 * square.rs
 * Part of the byte-knight project
 * Created Date: Friday, August 16th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Sat Aug 31 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

/// Converts a file and rank tuple to a square
///
/// # Arguments
///
/// * `file` - The file to convert
/// * `rank` - The rank to convert
///
/// # Returns
///
/// The square corresponding to the given file and rank
pub const fn to_square(file: u8, rank: u8) -> u8 {
    rank * 8 + file
}

/// Converts a square to a file and rank tuple
///
/// # Arguments
///
/// * `square` - The square to convert
///
/// # Returns
///
/// A tuple containing the file and rank of the given square (file, rank)
pub const fn from_square(square: u8) -> (u8, u8) {
    let rank = square / 8;
    let file = square % 8;
    return (file, rank);
}
