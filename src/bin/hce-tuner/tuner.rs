use std::thread::current;

use chess::board::Board;
use engine::{
    psqt::{
        EG_BISHOP_TABLE, EG_KING_TABLE, EG_KNIGHT_TABLE, EG_PAWN_TABLE, EG_QUEEN_TABLE,
        EG_ROOK_TABLE, EG_VALUE, GAMEPHASE_INC, MG_BISHOP_TABLE, MG_KING_TABLE, MG_KNIGHT_TABLE,
        MG_PAWN_TABLE, MG_QUEEN_TABLE, MG_ROOK_TABLE, MG_VALUE,
    },
    score::ScoreType,
};

/// Start
type Offset = usize;
pub(crate) struct Offsets {
    table: Vec<Offset>,
}

const MG_VALUE_INDEX: usize = 0;
const EG_TABLE_INDEX: usize = 1;
const GAMEPHASE_INC_INDEX: usize = 2;
const MG_PAWN_TABLE_INDEX: usize = 3;
const EG_PAWN_TABLE_INDEX: usize = 4;
const MG_KNIGHT_TABLE_INDEX: usize = 5;
const EG_KNIGHT_TABLE_INDEX: usize = 6;
const MG_BISHOP_TABLE_INDEX: usize = 7;
const EG_BISHOP_TABLE_INDEX: usize = 8;
const MG_ROOK_TABLE_INDEX: usize = 9;
const EG_ROOK_TABLE_INDEX: usize = 10;
const MG_QUEEN_TABLE_INDEX: usize = 11;
const EG_QUEEN_TABLE_INDEX: usize = 12;
const MG_KING_TABLE_INDEX: usize = 13;
const EG_KING_TABLE_INDEX: usize = 14;

const K_PRECISION: usize = 10;

impl Offsets {
    pub(crate) fn new() -> Self {
        Self {
            table: vec![
                0,   // MG_VALUE start
                6,   // EG_TABLE start
                12,  // GAMEPHASE_INC start
                18,  // MG_PAWN_TABLE start
                82,  // EG_PAWN_TABLE start
                146, // MG_KNIGHT_TABLE start
                210, // EG_KNIGHT_TABLE start
                274, // MG_BISHOP_TABLE start
                338, // EG_BISHOP_TABLE start
                402, // MG_ROOK_TABLE start
                466, // EG_ROOK_TABLE start
                530, // MG_QUEEN_TABLE start
                594, // EG_QUEEN_TABLE start
                658, // MG_KING_TABLE start
                722, // EG_KING_TABLE start
            ],
        }
    }

    pub(crate) fn total_size(&self) -> usize {
        self.table.last().unwrap() + EG_KING_TABLE.len()
    }
}

impl Default for Offsets {
    fn default() -> Self {
        Self::new()
    }
}

struct Tuner<'a> {
    offsets: Offsets,
    params: Vec<ScoreType>,
    positions: &'a Vec<Board>,
}

impl<'a> Tuner<'a> {
    pub(crate) fn new(positions: &'a Vec<Board>) -> Self {
        let offsets = Offsets::new();
        let mut params: Vec<ScoreType> = Vec::with_capacity(offsets.total_size());
        // TODO: Populate params from psqts
        params.extend(&MG_VALUE);
        params.extend(&EG_VALUE);
        params.extend(&GAMEPHASE_INC);
        params.extend(&MG_PAWN_TABLE);
        params.extend(&EG_PAWN_TABLE);
        params.extend(&MG_KNIGHT_TABLE);
        params.extend(&EG_KNIGHT_TABLE);
        params.extend(&MG_BISHOP_TABLE);
        params.extend(&EG_BISHOP_TABLE);
        params.extend(&MG_ROOK_TABLE);
        params.extend(&EG_ROOK_TABLE);
        params.extend(&MG_QUEEN_TABLE);
        params.extend(&EG_QUEEN_TABLE);
        params.extend(&MG_KING_TABLE);
        params.extend(&EG_KING_TABLE);

        assert_eq!(params.len(), offsets.total_size());
        Self {
            offsets,
            params,
            positions,
        }
    }

    pub(crate) fn tune(&self) -> Vec<ScoreType> {
        let K: f64 = self.compute_k();
        let adjustement = 1;

        let best_error = self.mean_square_error(K);
        let mut improved = true;
        let mut best_params = self.params.clone();

        while improved {
            improved = false;
            for i in 0..self.params.len() {
                let mut new_params = self.params.clone();
                new_params[i] += adjustement;
                let new_error = self.mean_square_error(K);
                if new_error < best_error {
                    best_params = new_params;
                    improved = true;
                } else {
                    new_params[i] -= 2 * adjustement;
                    let new_error = self.mean_square_error(K);
                    if new_error < best_error {
                        best_params = new_params;
                        improved = true;
                    }
                }
            }
        }

        best_params
    }

    fn sigmoid(K: f64, score: ScoreType) -> f64 {
        1.0 / (1.0 + 10_f64.powf(-K * score as f64 / 400.0))
    }
    fn mean_square_error(&self, K: f64) -> f64 {
        let error = 0.0;
        // TODO:
        //  - Loop over all positions
        //  - Evalute the board using our current parameters
        //  - Calculate the sigmoid of the score
        //  - Calculate the error as the square of the difference between the actual result and the sigmoid
        //  - Increase the error accumulator
        error / self.number_of_positions()
    }

    fn number_of_positions(&self) -> f64 {
        self.positions.len() as f64
    }

    /// Computes the optimal K value to minimize the error of the initial parameters.
    /// Based on the implementation in Andy Grant's original paper. 
    fn compute_k(&self) -> f64 {
        let mut start = 0.0;
        let mut end = 10.0;
        let mut step = 1.0;
        let mut best_e = self.mean_square_error(start);

        for i in 0..K_PRECISION {
            let mut current_k = start - step;
            // Optimize K to minimize the error
            while current_k < end {
                current_k = current_k + step;
                let e = self.mean_square_error(current_k);
                if e <= best_e {
                    // found a better value, update the best error and the start value
                    best_e = e;
                    start = current_k;
                }
            }

            // We will repeat, but adjust the search space
            end = start + step;
            start = start - step;
            step = step / 10.0;
        }

        start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offsets() {
        let offsets = Offsets::new();
        assert_eq!(offsets.total_size(), 786);
    }

    #[test]
    fn construct_tuner() {
        let positions = vec![]; // Add appropriate Board instances here
        let tuner = Tuner::new(&positions);
        assert_eq!(tuner.params.len(), 786);
    }
}
