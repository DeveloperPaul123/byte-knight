/*
 * tuneable.rs
 * Part of the byte-knight project
 * Created Date: Wednesday, December 11th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Fri Apr 25 2025
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use crate::score::ScoreType;

pub(crate) const MIN_ASPIRATION_DEPTH: ScoreType = 1;
pub(crate) const ASPIRATION_WINDOW: ScoreType = 50;

pub(crate) const MAX_RFP_DEPTH: ScoreType = 4;
pub(crate) const RFP_MARGIN: ScoreType = 82;

pub(crate) const IIR_MIN_DEPTH: ScoreType = 4;
pub(crate) const IIR_DEPTH_REDUCTION: ScoreType = 1;

pub(crate) const NMP_MIN_DEPTH: ScoreType = 3;
pub(crate) const NMP_DEPTH_REDUCTION: ScoreType = 2;

pub(crate) const LMR_OFFSET: f64 = 0.2;
pub(crate) const LMR_SCALING_FACTOR: f64 = 2.0;

// Minimum threshold depth for LMP to be considered
pub(crate) const LMP_MIN_THRESHOLD_DEPTH: ScoreType = 6;

pub(crate) const FUTILITY_DEPTH: ScoreType = 8;
pub(crate) const FUTILITY_COEFF: ScoreType = 3;
pub(crate) const FUTILITY_OFFSET: ScoreType = 100;
