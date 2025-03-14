use chess::{definitions::NumberOf, side::Side};

use crate::{math, parameters::Parameters, tuner_score::TuningScore};

pub(crate) struct TuningPosition {
    pub(crate) parameter_indexes: [Vec<usize>; NumberOf::SIDES],
    pub(crate) phase: usize,
    pub(crate) game_result: f64,
}

impl TuningPosition {
    pub(crate) fn new(
        white_indexes: Vec<usize>,
        black_indexes: Vec<usize>,
        phase: usize,
        game_result: f64,
    ) -> Self {
        // Side::White == 0, Side::Black == 1
        let parameter_indexes = [white_indexes, black_indexes];
        Self {
            parameter_indexes,
            phase,
            game_result,
        }
    }

    pub(crate) fn evaluate(&self, parameters: &Parameters) -> f64 {
        let mut score: TuningScore = Default::default();

        for &idx in &self.parameter_indexes[Side::White as usize] {
            score += parameters[idx];
        }

        for &idx in &self.parameter_indexes[Side::Black as usize] {
            score -= parameters[idx];
        }

        self.phase as f64 * score.mg() as f64 + (1 - self.phase) as f64 * score.eg() as f64
    }

    pub(crate) fn error(&self, k: f64, params: &Parameters) -> f64 {
        (self.game_result - math::sigmoid(k * self.evaluate(params))).powi(2)
    }
}
