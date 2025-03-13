use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

use crate::score::{LargeScoreType, ScoreType};

/// Represents a phased score in centipawns meaning that the score holds 2 values. One for midgame and one for endgame.
///
/// The mg score is stored in the upper 16 bits and the eg score in the lower 16 bits.
/// MSB mmmmmmmm mmmmmmmm eeeeeeee eeeeeeee LSB
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
#[must_use]
pub struct PhasedScore {
    value: LargeScoreType,
}

pub type PhaseType = i32;
const BITS: usize = ScoreType::BITS as usize;

impl PhasedScore {
    pub const fn new(mg: ScoreType, eg: ScoreType) -> Self {
        // TODO(PT): Check if scores are valid
        Self {
            value: (((mg as LargeScoreType) << BITS) + eg as LargeScoreType),
        }
    }

    pub fn mg(&self) -> ScoreType {
        // shift 16 bits right
        ((self.value + (1 << (BITS - 1))) >> BITS) as ScoreType
    }

    pub fn eg(&self) -> ScoreType {
        // only use the first 16 bits
        (self.value & 0xFFFF) as ScoreType
    }

    pub fn taper(&self, phase: PhaseType, max_phase: PhaseType) -> ScoreType {
        let mg_phase = phase.min(max_phase);
        let eg_phase = max_phase - mg_phase;
        ((self.mg() as PhaseType * mg_phase + self.eg() as PhaseType * eg_phase) / max_phase)
            as ScoreType
    }

    pub fn sqrt(&self) -> Self {
        Self::new(self.mg().isqrt(), self.eg().isqrt())
    }
}

impl Add<PhasedScore> for PhasedScore {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.mg() + rhs.mg(), self.eg() + rhs.eg())
    }
}

impl AddAssign<PhasedScore> for PhasedScore {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Add<f64> for PhasedScore {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self::new(
            (self.mg() as f64 + rhs) as ScoreType,
            (self.eg() as f64 + rhs) as ScoreType,
        )
    }
}

impl AddAssign<f64> for PhasedScore {
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl Sub<PhasedScore> for PhasedScore {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.mg() - rhs.mg(), self.eg() - rhs.eg())
    }
}

impl SubAssign<PhasedScore> for PhasedScore {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<f64> for PhasedScore {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(
            (self.mg() as f64 * rhs) as ScoreType,
            (self.eg() as f64 * rhs) as ScoreType,
        )
    }
}

impl Mul<PhasedScore> for f64 {
    type Output = PhasedScore;

    fn mul(self, rhs: PhasedScore) -> Self::Output {
        rhs * self
    }
}

impl Mul<PhasedScore> for PhasedScore {
    type Output = PhasedScore;

    fn mul(self, rhs: PhasedScore) -> Self::Output {
        PhasedScore::new(self.mg() * rhs.mg(), self.eg() * rhs.eg())
    }
}

impl Div<PhasedScore> for PhasedScore {
    type Output = PhasedScore;

    fn div(self, rhs: PhasedScore) -> Self::Output {
        PhasedScore::new(self.mg() / rhs.mg(), self.eg() / rhs.eg())
    }
}

const fn phase_score(mg: ScoreType, eg: ScoreType) -> PhasedScore {
    PhasedScore::new(mg, eg)
}

#[allow(non_snake_case)]
pub const fn S(mg: ScoreType, eg: ScoreType) -> PhasedScore {
    phase_score(mg, eg)
}

impl Display for PhasedScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mg: {}, eg: {}", self.mg(), self.eg())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn phased_score() {
        use super::PhasedScore;

        let ps = PhasedScore::new(100, 50);
        assert_eq!(ps.mg(), 100);
        assert_eq!(ps.eg(), 50);

        let phase = 50;
        assert_eq!(ps.taper(phase, 100), 75);

        let ps: PhasedScore = PhasedScore::new(40, 80);
        assert_eq!(ps.mg(), 40);
        assert_eq!(ps.eg(), 80);

        let phase = 12;
        assert_eq!(ps.taper(phase, 24), 60);

        let phase = 24;
        let ps = PhasedScore::new(56, -26);
        assert_eq!(ps.mg(), 56);
        assert_eq!(ps.eg(), -26);
        assert_eq!(ps.taper(phase, 24), 56);
    }
}
