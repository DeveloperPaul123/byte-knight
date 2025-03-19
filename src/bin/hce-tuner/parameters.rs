use std::ops::{Add, Index, IndexMut};

use chess::{definitions::NumberOf, pieces::ALL_PIECES, side::Side};
use engine::hce_values::PSQTS;

use crate::{math, tuner_score::TuningScore, tuning_position::TuningPosition};

pub struct Parameters([TuningScore; NumberOf::PARAMETERS]);

impl Parameters {
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
    pub(crate) fn as_slice(&self) -> &[TuningScore] {
        &self.0
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self([TuningScore::default(); NumberOf::PARAMETERS])
    }
}

impl Index<usize> for Parameters {
    type Output = TuningScore;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Parameters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Add<Parameters> for Parameters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Parameters::default();
        for i in 0..NumberOf::PARAMETERS {
            result[i] = self[i] + rhs[i];
        }
        result
    }
}

impl Parameters {
    pub(crate) fn create_from_engine_values() -> Parameters {
        let mut params = Parameters::default();
        for &piece in ALL_PIECES.iter() {
            for sq in 0..NumberOf::SQUARES {
                // seed from our PSQTS table
                let s = PSQTS[piece as usize][sq].into();
                params[64 * piece as usize + sq] = s;
            }
        }
        params
    }

    pub(crate) fn gradient_batch(&self, k: f64, data: &[TuningPosition]) -> Self {
        let mut gradient = Parameters::default();
        for point in data {
            let sigmoid_result = math::sigmoid(k * point.evaluate(self));
            let term =
                (point.game_result - sigmoid_result) * (1. - sigmoid_result) * sigmoid_result;
            let phase_adjustment =
                term * TuningScore::new(point.phase as f64, 1f64 - point.phase as f64);

            for idx in &point.parameter_indexes[Side::White as usize] {
                gradient[*idx] += phase_adjustment;
            }

            for idx in &point.parameter_indexes[Side::Black as usize] {
                gradient[*idx] -= phase_adjustment;
            }
        }
        gradient
    }
}
