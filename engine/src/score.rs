/*
 * score.rs
 * Part of the byte-knight project
 * Created Date: Thursday, November 14th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Mon Dec 02 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::ops::{Sub, SubAssign};
use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Neg},
};
use uci_parser::UciScore;

pub(crate) type ScoreType = i16;
/// Represents a score in centipawns.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Score(pub ScoreType);

impl Score {
    pub const DRAW: Score = Score(0);
    pub const MATE: Score = Score(ScoreType::MAX as ScoreType);
    pub const INF: Score = Score(ScoreType::MAX as ScoreType);
    
    pub fn new(score: ScoreType) -> Score {
        Score(score)
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
