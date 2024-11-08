use std::{
    fmt::{self, Display, Formatter},
    ops::{AddAssign, Neg},
};

use uci_parser::UciScore;

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

impl Into<UciScore> for Score {
    fn into(self) -> UciScore {
        UciScore::cp(self.0 as i32)
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.0.abs() >= Score::MATE.0.abs() {
            write!(f, "mate {}", (self.0 - Score::MATE.0) / 2)
        } else {
            write!(f, "cp {}", self.0)
        }
    }
}

impl Neg for Score {
    type Output = Score;

    fn neg(self) -> Score {
        Score(-self.0)
    }
}

impl AddAssign for Score {
    fn add_assign(&mut self, other: Score) {
        self.0 += other.0;
    }
}

impl AddAssign<i64> for Score {
    fn add_assign(&mut self, other: i64) {
        self.0 += other;
    }
}
