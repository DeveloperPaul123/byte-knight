/*
 * psqt.rs
 * Part of the byte-knight project
 * Created Date: Thursday, November 21st 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Mon Dec 02 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use chess::{bitboard_helpers, board::Board, pieces::PIECE_NAMES, side::Side};

use crate::score::{Score, ScoreType};

/// Mid-game piece values
/// Ordered to match the indexing of [`Piece`]
/// King, Queen, Rook, Bishop, Knight, Pawn
const MG_VALUE: [ScoreType; 6] = [0, 1025, 477, 365, 337, 82];

/// End-game piece values
/// Ordered to match the indexing of [`Piece`]
/// King, Queen, Rook, Bishop, Knight, Pawn
const EG_VALUE: [ScoreType; 6] = [0, 936, 512, 297, 281, 94];

#[rustfmt::skip]
const MG_PAWN_TABLE: [ScoreType; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    98, 134, 61, 95, 68, 126, 34, -11,
    -6, 7, 26, 31, 65, 56, 25, -20,
    -14, 13, 6, 21, 23, 12, 17, -23,
    -27, -2, -5, 12, 17, 6, 10, -25,
    -26, -4, -4, -10, 3, 3, 33, -12,
    -35, -1, -20, -23, -15, 24, 38, -22,
    0, 0, 0, 0, 0, 0, 0, 0,
];

#[rustfmt::skip]
const EG_PAWN_TABLE: [ScoreType; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    178, 173, 158, 134, 147, 132, 165, 187,
    94, 100, 85, 67, 56, 53, 82, 84,
    32, 24, 13, 5, -2, 4, 17, 17,
    13, 9, -3, -7, -7, -8, 3, -1,
    4, 7, -6, 1, 0, -5, -1, -8,
    13, 8, 8, 10, 13, 0, 2, -7,
    0, 0, 0, 0, 0, 0, 0, 0,
];

#[rustfmt::skip]
const MG_KNIGHT_TABLE: [ScoreType; 64] = [
    -167, -89, -34, -49, 61, -97, -15, -107,
    -73, -41, 72, 36, 23, 62, 7, -17,
    -47, 60, 37, 65, 84, 129, 73, 44,
    -9, 17, 19, 53, 37, 69, 18, 22,
    -13, 4, 16, 13, 28, 19, 21, -8,
    -23, -9, 12, 10, 19, 17, 25, -16,
    -29, -53, -12, -3, -1, 18, -14, -19,
    -105, -21, -58, -33, -17, -28, -19, -23,
];

#[rustfmt::skip]
const EG_KNIGHT_TABLE: [ScoreType; 64] = [
    -58, -38, -13, -28, -31, -27, -63, -99,
    -25, -8, -25, -2, -9, -25, -24, -52,
    -24, -20, 10, 9, -1, -9, -19, -41,
    -17, 3, 22, 22, 22, 11, 8, -18,
    -18, -6, 16, 25, 16, 17, 4, -18,
    -23, -3, -1, 15, 10, -3, -20, -22,
    -42, -20, -10, -5, -2, -20, -23, -44,
    -29, -51, -23, -15, -22, -18, -50, -64,
];

#[rustfmt::skip]
const MG_BISHOP_TABLE: [ScoreType; 64] = [
    -29, 4, -82, -37, -25, -42, 7, -8,
    -26, 16, -18, -13, 30, 59, 18, -47,
    -16, 37, 43, 40, 35, 50, 37, -2,
    -4, 5, 19, 50, 37, 37, 7, -2,
    -6, 13, 13, 26, 34, 12, 10, 4,
    0, 15, 15, 15, 14, 27, 18, 10,
    4, 15, 16, 0, 7, 21, 33, 1,
    -33, -3, -14, -21, -13, -12, -39, -21,
];

#[rustfmt::skip]
const EG_BISHOP_TABLE: [ScoreType; 64] = [
    -14, -21, -11, -8, -7, -9, -17, -24,
    -8, -4, 7, -12, -3, -13, -4, -14,
    2, -8, 0, -1, -2, 6, 0, 4,
    -3, 9, 12, 9, 14, 10, 3, 2,
    -6, 3, 13, 19, 7, 10, -3, -9,
    -12, -3, 8, 10, 13, 3, -7, -15,
    -14, -18, -7, -1, 4, -9, -15, -27,
    -23, -9, -23, -5, -9, -16, -5, -17,
];

#[rustfmt::skip]
const MG_ROOK_TABLE: [ScoreType; 64] = [
    32, 42, 32, 51, 63, 9, 31, 43,
    27, 32, 58, 62, 80, 67, 26, 44,
    -5, 19, 26, 36, 17, 45, 61, 16,
    -24, -11, 7, 26, 24, 35, -8, -20,
    -36, -26, -12, -1, 9, -7, 6, -23,
    -45, -25, -16, -17, 3, 0, -5, -33,
    -44, -16, -20, -9, -1, 11, -6, -71,
    -19, -13, 1, 17, 16, 7, -37, -26,
];

#[rustfmt::skip]
const EG_ROOK_TABLE: [ScoreType; 64] = [
    13, 10, 18, 15, 12, 12, 8, 5,
    11, 13, 13, 11, -3, 3, 8, 3,
    7, 7, 7, 5, 4, -3, -5, -3,
    4, 3, 13, 1, 2, 1, -1, 2,
    3, 5, 8, 4, -5, -6, -8, -11,
    -4, 0, -5, -1, -7, -12, -8, -16,
    -6, -6, 0, 2, -9, -9, -11, -3,
    -9, 2, 3, -1, -5, -13, 4, -20,
];

#[rustfmt::skip]
const MG_QUEEN_TABLE: [ScoreType; 64] = [
    -28, 0, 29, 12, 59, 44, 43, 45,
    -24, -39, -5, 1, -16, 57, 28, 54,
    -13, -17, 7, 8, 29, 56, 47, 57,
    -27, -27, -16, -16, -1, 17, -2, 1,
    -9, -26, -9, -10, -2, -4, 3, -3,
    -14, 2, -11, -2, -5, 2, 14, 5,
    -35, -8, 11, 2, 8, 15, -3, 1,
    -1, -18, -9, 10, -15, -25, -31, -50,
];

#[rustfmt::skip]
const EG_QUEEN_TABLE: [ScoreType; 64] = [
    -9, 22, 22, 27, 27, 19, 10, 20,
    -17, 20, 32, 41, 58, 25, 30, 0,
    -20, 6, 9, 49, 47, 35, 19, 9,
    3, 22, 24, 45, 57, 40, 57, 36,
    -18, 28, 19, 47, 31, 34, 39, 23,
    -16, -27, 15, 6, 9, 17, 10, 5,
    -22, -23, -30, -16, -16, -23, -36, -32,
    -33, -28, -22, -43, -5, -32, -20, -41,
];

#[rustfmt::skip]
const MG_KING_TABLE: [ScoreType; 64] = [
    -65, 23, 16, -15, -56, -34, 2, 13,
    29, -1, -20, -7, -8, -4, -38, -29,
    -9, 24, 2, -16, -20, 6, 22, -22,
    -17, -20, -12, -27, -30, -25, -14, -36,
    -49, -1, -27, -39, -46, -44, -33, -51,
    -14, -14, -22, -46, -44, -30, -15, -27,
    1, 7, -8, -64, -43, -16, 9, 8,
    -15, 36, 12, -54, 8, -28, 24, 14,
];

#[rustfmt::skip]
const EG_KING_TABLE: [ScoreType; 64] = [
    -74, -35, -18, -18, -11, 15, 4, -17,
    -12, 17, 14, 17, 17, 38, 23, 11,
    10, 17, 23, 15, 20, 45, 44, 13,
    -8, 22, 24, 27, 26, 33, 26, 3,
    -18, -4, 21, 24, 27, 23, 9, -11,
    -19, -3, 11, 21, 23, 16, 7, -9,
    -27, -11, 4, 13, 14, 4, -5, -17,
    -53, -34, -21, -11, -28, -14, -24, -43,
];

/// Opening/mid-game piece-square tables
/// Ordered to match the indexing of [`Piece`]
const MG_PESTO_TABLE: [&[ScoreType; 64]; 6] = [
    &MG_KING_TABLE,
    &MG_QUEEN_TABLE,
    &MG_ROOK_TABLE,
    &MG_BISHOP_TABLE,
    &MG_KNIGHT_TABLE,
    &MG_PAWN_TABLE,
];

/// Endgame piece-square tables
/// Ordered to match the indexing of [`Piece`]
const EG_PESTO_TABLE: [&[ScoreType; 64]; 6] = [
    &EG_KING_TABLE,
    &EG_QUEEN_TABLE,
    &EG_ROOK_TABLE,
    &EG_BISHOP_TABLE,
    &EG_KNIGHT_TABLE,
    &EG_PAWN_TABLE,
];

/// Game phase increment for each piece
/// Ordered to match the indexing of [`Piece`]
/// King, Queen, Rook, Bishop, Knight, Pawn
const GAMEPHASE_INC: [ScoreType; 6] = [0, 4, 2, 1, 1, 0];

/// Piece-Square Tables (PST) for evaluation
pub(crate) struct Psqt {
    mg_table: [[ScoreType; 64]; 12],
    eg_table: [[ScoreType; 64]; 12],
}

const FLIP: fn(usize) -> usize = |sq| sq ^ 56;

impl Psqt {
    /// Creates a new [`Psqt`] instance and initializes the piece-square tables.
    pub(crate) fn new() -> Self {
        let mut psqt = Psqt {
            mg_table: [[0; 64]; 12],
            eg_table: [[0; 64]; 12],
        };
        psqt.initialize_tables();
        psqt
    }

    /// Evaluates the given [`Board`] position. This uses the piece square tables
    /// as well as a tapering function to take into account game phase.
    ///
    /// See <https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function> for more information.
    ///
    /// # Arguments
    ///
    /// - `board`: The [`Board`] to evaluate.
    ///
    /// # Returns
    ///
    /// The score of the position.
    pub(crate) fn evaluate(&self, board: &Board) -> Score {
        let side_to_move = board.side_to_move();
        let mut mg: [i32; 2] = [0; 2];
        let mut eg: [i32; 2] = [0; 2];
        let mut game_phase = 0_i32;

        let mut occupancy = board.all_pieces();
        // loop through occupied squares
        while occupancy.as_number() > 0 {
            let sq = bitboard_helpers::next_bit(&mut occupancy);
            let maybe_piece = board.piece_on_square(sq as u8);
            if let Some((piece, side)) = maybe_piece {
                let pc_idx = piece as usize * 2 + side as usize;

                mg[side as usize] += self.mg_table[pc_idx][sq] as i32;
                eg[side as usize] += self.eg_table[pc_idx][sq] as i32;

                game_phase += GAMEPHASE_INC[piece as usize] as i32;
            }
        }

        let mg_score = mg[side_to_move as usize] - mg[Side::opposite(side_to_move) as usize];
        let eg_score = eg[side_to_move as usize] - eg[Side::opposite(side_to_move) as usize];
        let mg_phase = game_phase.min(24);
        let eg_phase = 24 - mg_phase;
        let score = (mg_score * mg_phase + eg_score * eg_phase) / 24;
        Score::new(score as i16)
    }

    /// Helper to initialize the tables
    /// See <https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function>
    ///
    /// Here white is 0 and black is 1, see [`Side`]. The PSQT tables are from white's perspective, so we need to flip
    /// the board for white and not for black.
    fn initialize_tables(&mut self) {
        for (p, pc) in (0..6).zip((0..12).step_by(2)) {
            for sq in 0..64 {
                self.mg_table[pc][sq] = MG_VALUE[p] + MG_PESTO_TABLE[p][FLIP(sq)];
                self.eg_table[pc][sq] = EG_VALUE[p] + EG_PESTO_TABLE[p][FLIP(sq)];
                self.mg_table[pc + 1][sq] = MG_VALUE[p] + MG_PESTO_TABLE[p][sq];
                self.eg_table[pc + 1][sq] = EG_VALUE[p] + EG_PESTO_TABLE[p][sq];
            }
        }
    }

    /// Helper to print the mid-game and end-game tables
    /// Output is formatted as a 8x8 board with (mg, eg) values for each square, per piece
    #[allow(dead_code)]
    fn print_tables(&self) {
        for (p, pc) in (0..6).zip((0..12).step_by(2)) {
            println!("Piece: {}", PIECE_NAMES[p]);
            for row in 0..8 {
                for col in 0..8 {
                    let sq = row * 8 + col;
                    if col == 0 {
                        print!("| ");
                    }

                    print!(
                        "({:4}, {:4}), ",
                        self.mg_table[pc][sq], self.eg_table[pc][sq]
                    );
                    if col == 7 {
                        println!(" |");
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chess::board::Board;

    use crate::{psqt::Psqt, score::Score};

    #[test]
    fn default_position_is_equal() {
        let board = Board::default_board();
        let psqt = Psqt::new();
        let score = psqt.evaluate(&board);
        assert_eq!(score, super::Score::new(0));
    }

    #[test]
    fn white_ahead() {
        let fens = [
            "4k3/8/8/8/8/8/PPPPPPPP/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/NNNNNNNN/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/BBBBBBBB/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/RRRR1RRR/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/QQQQ1QQQ/4K3 w - - 0 1",
            "4k3/1QQR1RQQ/1QQQKQQ1/1BBNN3/8/8/8/8 w - - 0 1",
        ];

        for fen in fens {
            let pos = Board::from_fen(fen).unwrap();
            let eval = Psqt::new().evaluate(&pos);
            assert!(eval > Score::new(0));
        }
    }
}
