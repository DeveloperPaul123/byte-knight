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

use crate::definitions::{File, Rank};

pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }
    pub fn to_square_index(&self) -> u8 {
        to_square(self.file as u8, self.rank as u8)
    }
    pub fn from_square_index(square: u8) -> Self {
        let (file, rank) = from_square(square);
        Self {
            file: File::try_from(file).unwrap(),
            rank: Rank::try_from(rank).unwrap(),
        }
    }
}

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

pub fn to_square_object(file: u8, rank: u8) -> Square {
    Square::new(File::try_from(file).unwrap(), Rank::try_from(rank).unwrap())
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
