/*
 * tuneable.rs
 * Part of the byte-knight project
 * Created Date: Wednesday, December 11th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Thu Dec 12 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use crate::score::ScoreType;

pub(crate) const MIN_ASPIRATION_DEPTH: ScoreType = 1;
pub(crate) const ASPIRATION_WINDOW: ScoreType = 50;