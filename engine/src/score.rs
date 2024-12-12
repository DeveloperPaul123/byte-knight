/*
 * score.rs
 * Part of the byte-knight project
 * Created Date: Thursday, November 14th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Thu Dec 12 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::ops::{Div, DivAssign, Mul, MulAssign, Shl, Sub, SubAssign};
use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Neg},
};
use uci_parser::UciScore;

use crate::defs::MAX_DEPTH;

pub(crate) type ScoreType = i16;
pub(crate) type MoveOrderScoreType = i32;
/// Represents a score in centipawns.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Score(pub ScoreType);

impl Score {
    pub const DRAW: Score = Score(0);
    pub const MATE: Score = Score(ScoreType::MAX as ScoreType);
    pub const MINIMUM_MATE: Score = Score(Score::MATE.0 - MAX_DEPTH as ScoreType);
    pub const INF: Score = Score(ScoreType::MAX as ScoreType);
    pub const ALPHA: Score = Score(-Score::INF.0);
    pub const BETA: Score = Score::INF;

    // Max/min score for history heuristic
    // Must be lower then the minimum score for captures in MVV_LVA
    pub const MAX_HISTORY: MoveOrderScoreType = 16_384;

    pub fn new(score: ScoreType) -> Score {
        Score(score)
    }

    pub fn clamp(&self, min: ScoreType, max: ScoreType) -> Score {
        Score(self.0.clamp(min, max))
    }

    pub fn is_mate(&self) -> bool {
        self.0.abs() >= Score::MINIMUM_MATE.0.abs()
    }
}

impl From<Score> for UciScore {
    fn from(value: Score) -> Self {
        UciScore::cp(value.0.into())
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

impl AddAssign<ScoreType> for Score {
    fn add_assign(&mut self, other: ScoreType) {
        self.0 += other;
    }
}

impl Add for Score {
    type Output = Score;

    fn add(self, other: Score) -> Score {
        Score(self.0 + other.0)
    }
}

impl Add<ScoreType> for Score {
    type Output = Score;

    fn add(self, other: ScoreType) -> Score {
        Score(self.0 + other)
    }
}

impl Sub for Score {
    type Output = Score;
    fn sub(self, other: Score) -> Score {
        Score(self.0 - other.0)
    }
}

impl Sub<ScoreType> for Score {
    type Output = Score;
    fn sub(self, other: ScoreType) -> Score {
        Score(self.0 - other)
    }
}

impl SubAssign for Score {
    fn sub_assign(&mut self, other: Score) {
        self.0 -= other.0;
    }
}

impl SubAssign<ScoreType> for Score {
    fn sub_assign(&mut self, rhs: ScoreType) {
        self.0 -= rhs;
    }
}

impl Div<ScoreType> for Score {
    type Output = Score;
    fn div(self, rhs: ScoreType) -> Score {
        Score(self.0 / rhs)
    }
}

impl Div<Score> for Score {
    type Output = Score;
    fn div(self, rhs: Score) -> Score {
        Score(self.0 / rhs.0)
    }
}

impl DivAssign<ScoreType> for Score {
    fn div_assign(&mut self, rhs: ScoreType) {
        self.0 /= rhs;
    }
}

impl DivAssign<Score> for Score {
    fn div_assign(&mut self, rhs: Score) {
        self.0 /= rhs.0;
    }
}

impl Mul<ScoreType> for Score {
    type Output = Score;
    fn mul(self, rhs: ScoreType) -> Score {
        Score(self.0 * rhs)
    }
}

impl Mul<Score> for Score {
    type Output = Score;
    fn mul(self, rhs: Score) -> Score {
        Score(self.0 * rhs.0)
    }
}

impl MulAssign<ScoreType> for Score {
    fn mul_assign(&mut self, rhs: ScoreType) {
        self.0 *= rhs;
    }
}

impl MulAssign<Score> for Score {
    fn mul_assign(&mut self, rhs: Score) {
        self.0 *= rhs.0;
    }
}

impl Shl<u32> for Score {
    type Output = Score;
    fn shl(self, rhs: u32) -> Score {
        Score(self.0 << rhs)
    }
}
