/*
 * lib.rs
 * Part of the byte-knight project
 * Created Date: Friday, August 16th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified:
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

pub mod bitboard;
mod bitboard_helpers;
pub mod board;
pub mod board_state;
pub mod definitions;
pub mod fen;
pub mod magics;
pub mod move_generation;
pub mod move_history;
pub mod move_list;
pub mod move_making;
pub mod moves;
pub mod pieces;
pub mod square;
pub mod zobrist;