use crate::{
    score::{Score, ScoreType},
    tuneable::{ASPIRATION_WINDOW, MIN_ASPIRATION_DEPTH, MIN_ASPIRATION_WINDOW},
};

pub(crate) struct AspirationWindow {
    alpha: Score,
    beta: Score,
    alpha_fails: u32,
    beta_fails: u32,
}

impl AspirationWindow {
    pub(crate) fn infinite() -> Self {
        Self {
            alpha: Score::ALPHA,
            beta: Score::BETA,
            alpha_fails: 0,
            beta_fails: 0,
        }
    }

    pub(crate) fn alpha(&self) -> Score {
        self.alpha
    }

    pub(crate) fn beta(&self) -> Score {
        self.beta
    }

    pub(crate) fn failed_low(&self, score: Score) -> bool {
        score != Score::ALPHA && score <= self.alpha
    }

    pub(crate) fn failed_high(&self, score: Score) -> bool {
        score != Score::BETA && score >= self.beta
    }

    /// Create a new [`AspirationWindow`] centered around the given score.
    pub(crate) fn around(score: Score, depth: ScoreType) -> Self {
        if depth <= MIN_ASPIRATION_DEPTH || score.is_mate() {
            // If the score is mate, we can't use the window as we would expect search results to fluctuate.
            // Set it to a full window and search again.
            // We also want to do a full search on the first iteration (i.e. depth == 1);
            return Self::infinite();
        } else {
            let window = Self::window_size(depth);
            Self {
                alpha: (score - window).max(Score::ALPHA),
                beta: (score + window).min(Score::BETA),
                alpha_fails: 0,
                beta_fails: 0,
            }
        }
    }

    pub(crate) fn widen_down(&mut self, score: Score, depth: ScoreType) {
        let margin = Self::window_size(depth) << (self.alpha_fails + 1);
        self.alpha = (score - margin).max(Score::ALPHA);
        // save that this was a fail low
        self.alpha_fails += 1;
    }

    pub(crate) fn widen_up(&mut self, score: Score, depth: ScoreType) {
        // Note that we do not alter alpha here, as we are widening the window upwards.
        let margin = Self::window_size(depth) << (self.beta_fails + 1);
        let new_beta = (score.0 as i32 + margin.0 as i32).min(Score::BETA.0 as i32);
        self.beta = Score::new(new_beta as ScoreType);
        // save that this was a fail high
        self.beta_fails += 1;
    }

    fn window_size(depth: ScoreType) -> Score {
        let window = (ASPIRATION_WINDOW + (50 / depth)).max(MIN_ASPIRATION_WINDOW);
        Score::new(window)
    }
}
