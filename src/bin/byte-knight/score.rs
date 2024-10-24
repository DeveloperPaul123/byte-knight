use std::ops::Neg;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(pub i64);

impl Score {
    pub const DRAW: Score = Score(0);
    pub const MATE: Score = Score(10_000);
    pub const INF: Score = Score(i32::MAX as i64);

    pub fn new(score: i64) -> Score {
        Score(score)
    }
}

impl Neg for Score {
    type Output = Score;

    fn neg(self) -> Score {
        Score(-self.0)
    }
}
