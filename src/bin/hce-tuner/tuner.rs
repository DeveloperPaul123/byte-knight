use anyhow::Result;
use chess::{board::Board, pieces::Piece};
use engine::{evaluation::Evaluation, hce_values::PSQTS, score::ScoreType, traits::Eval};

use crate::{offsets::Offsets, tuner_values::TunerValues};

const K_PRECISION: usize = 10;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub(crate) enum TableType {
    Midgame,
    Endgame,
}

fn calculate_psqt_index(
    piece: Piece,
    square: usize,
    table_type: TableType,
    offsets: &Offsets,
) -> Result<usize> {
    let start_index = offsets.start_index_for_piece(piece, table_type)?;
    Ok(start_index + square)
}

pub(crate) struct Position {
    pub(crate) board: Board,
    pub(crate) game_result: f64,
}

pub(crate) struct Tuner<'a> {
    positions: &'a Vec<Position>,
    evaluation: Evaluation<TunerValues>,
}

impl<'a> Tuner<'a> {
    pub(crate) fn new(positions: &'a Vec<Position>) -> Self {
        let offsets = Offsets::new();
        let mut params: Vec<ScoreType> = vec![0; offsets.total_size()];
        for piece in Piece::iter() {
            for square in 0..64 {
                let ps = PSQTS[piece as usize][square];
                let mg_index = calculate_psqt_index(piece, square, TableType::Midgame, &offsets)
                    .expect(&format!("Failed to get MG index for {}", piece));
                let eg_index = calculate_psqt_index(piece, square, TableType::Endgame, &offsets)
                    .expect(&format!("Failed to get EG index for {}", piece));
                params[mg_index] = ps.mg();
                params[eg_index] = ps.eg();
            }
        }

        assert_eq!(params.len(), offsets.total_size());
        let evaluation = Evaluation::new(TunerValues::new(offsets.clone(), params));

        Self {
            positions,
            evaluation,
        }
    }

    fn tuner_values(&mut self) -> &mut TunerValues {
        self.evaluation.mutable_values()
    }

    pub(crate) fn tune(&mut self) -> &Vec<ScoreType> {
        println!("Computing optimal K value...");
        let computed_k: f64 = self.compute_k();
        println!("Optimal K value: {}", computed_k);
        let adjustement = 1;

        let mut best_error = self.mean_square_error(computed_k);
        println!("Initial error: {}", best_error);
        let mut improved = true;

        let param_len = self.evaluation.values().params().len();
        while improved {
            improved = false;
            for i in 0..param_len {
                self.tuner_values().increment_param(i, adjustement);
                let new_error = self.mean_square_error(computed_k);
                if new_error < best_error {
                    println!("New error: {} for param {}", new_error, i);
                    // commit the new param
                    self.tuner_values().commit();
                    // update the best error and mark the improvement
                    best_error = new_error;
                    improved = true;
                } else {
                    // if we're here, the increment didn't improve the error, let's try decrementing
                    self.tuner_values().decrement_param(i, adjustement);
                    let new_error = self.mean_square_error(computed_k);
                    if new_error < best_error {
                        println!("New error: {} for param {}", new_error, i);
                        // commit the new param
                        self.tuner_values().commit();
                        // update the best error and mark the improvement
                        best_error = new_error;
                        improved = true;
                    } else {
                        self.tuner_values().discard();
                    }
                }
            }
        }

        // return the best parameters
        self.evaluation.values().params()
    }

    fn sigmoid(K: f64, score: ScoreType) -> f64 {
        1.0 / (1.0 + 10_f64.powf(-K * score as f64 / 400.0))
    }

    fn mean_square_error(&self, K: f64) -> f64 {
        let mut error = 0.0;
        
        //  - Loop over all positions
        //  - Evalute the board using our current parameters
        //  - Calculate the sigmoid of the score
        //  - Calculate the error as the square of the difference between the actual result and the sigmoid
        //  - Increase the error accumulator

        for pos in self.positions {
            let score = self.evaluation.eval(&pos.board);
            let sigmoid = Self::sigmoid(K, score.0);
            error += (pos.game_result - sigmoid).powi(2);
        }
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

        for _i in 0..K_PRECISION {
            let mut current_k = start - step;
            // Optimize K to minimize the error
            while current_k < end {
                current_k = current_k + step;
                let e = self.mean_square_error(current_k);
                if e < best_e {
                    // found a better value, update the best error and the start value
                    best_e = e;
                    start = current_k;
                    println!("New best K: {} with error: {}", start, best_e);
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
        assert_eq!(offsets.total_size(), 768);
    }

    #[test]
    fn construct_tuner() {
        let positions = vec![]; // Add appropriate Board instances here
        let tuner = Tuner::new(&positions);
        assert_eq!(tuner.evaluation.values().params().len(), 768);
    }
}
