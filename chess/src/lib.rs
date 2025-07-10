/*
 * lib.rs
 * Part of the byte-knight project
 * Created Date: Friday, August 16th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Thu Apr 24 2025
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

#![deny(clippy::unused_result_ok)]
#![deny(clippy::panic)]
#![deny(clippy::expect_used)]

pub mod bitboard;
pub mod bitboard_helpers;
pub mod board;
pub mod board_state;
pub mod color;
pub mod definitions;
pub mod fen;
pub mod file;
pub mod legal_move_generation;
pub mod magics;
pub mod move_generation;
pub mod move_history;
pub mod move_list;
pub mod move_making;
pub mod moves;
pub mod non_slider_piece;
pub mod perft;
pub mod pext;
pub mod piece_category;
pub mod pieces;
pub mod rank;
pub mod side;
pub mod slider_pieces;
pub mod square;
pub mod zobrist;
