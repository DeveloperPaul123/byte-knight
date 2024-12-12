use crate::{
    score::{Score, ScoreType},
    tuneable::{INITIAL_ASPIRATION_WINDOW, MIN_ASPIRATION_DEPTH, MIN_ASPIRATION_WINDOW},
};

pub(crate) struct AspirationWindow {
    alpha: Score,
    beta: Score,
    value: Score,
    alpha_fails: u32,
    beta_fails: u32,
}

impl AspirationWindow {
    pub(crate) fn infinite() -> Self {
        Self {
            alpha: Score::ALPHA,
            beta: Score::BETA,
            value: Score::default(),
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

    pub(crate) fn around(score: Score, depth: ScoreType) -> Self {
        if depth > MIN_ASPIRATION_DEPTH || score.is_mate() {
            // If the score is mate, we can't use the window as we would expect search results to fluctuate.
            // Set it to a full window and search again.
            return Self::infinite();
        } else {
            Self {
                alpha: (score - Self::window_size(depth)).max(Score::ALPHA),
                beta: (score + Self::window_size(depth)).min(Score::BETA),
                value: score,
                alpha_fails: 0,
                beta_fails: 0,
            }
        }
    }

    pub(crate) fn widen_down(&mut self, score: Score, depth: ScoreType) {
        self.value = score;
        let margin = Self::window_size(depth) << (self.alpha_fails + 1);
        self.alpha = self.value - margin;
        // reset beta to be (alpha + beta / 2)
        self.beta = (self.alpha + self.beta) / 2;
        // save that this was a fail low
        self.alpha_fails += 1;
    }

    pub(crate) fn widen_up(&mut self, score: Score, depth: ScoreType) {
        self.value = score;
        let margin = Self::window_size(depth).0 << (self.beta_fails + 1);
        self.beta = self.value + margin;
        self.beta_fails += 1;
        // Note that we do not alter alpha here, as we are widening the window upwards.
    }

    fn window_size(depth: ScoreType) -> Score {
        let window = ((INITIAL_ASPIRATION_WINDOW << 2) / depth).max(MIN_ASPIRATION_WINDOW);
        Score::new(window)
    }
}
