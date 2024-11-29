/*
 * score.rs
 * Part of the byte-knight project
 * Created Date: Thursday, November 14th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Fri Nov 29 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Neg},
};

use uci_parser::UciScore;

/// Represents a score in centipawns.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(pub i64);

impl Score {
    pub const DRAW: Score = Score(0);
    pub const MATE: Score = Score(i32::MAX as i64);
    /// We use i32 so we don't overflow
    pub const INF: Score = Score(i32::MAX as i64);

    pub fn new(score: i64) -> Score {
        Score(score)
    }
}

impl From<Score> for UciScore {
    fn from(value: Score) -> Self {
        UciScore::cp(value.0 as i32)
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

impl Add for Score {
    type Output = Score;

    fn add(self, other: Score) -> Score {
        Score(self.0 + other.0)
    }
}

impl Add<i64> for Score {
    type Output = Score;

    fn add(self, other: i64) -> Score {
        Score(self.0 + other)
    }
}
