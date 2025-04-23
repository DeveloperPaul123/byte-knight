use crate::tuneable::{LMR_OFFSET, LMR_SCALING_FACTOR};

pub(crate) fn formula(depth: usize, move_count: usize) -> f64 {
    let d_ln = (depth as f64).ln();
    let mvs_ln = (move_count as f64).ln();
    if d_ln.is_finite() && mvs_ln.is_finite() {
        LMR_OFFSET + (d_ln * mvs_ln) / LMR_SCALING_FACTOR
    } else {
        0_f64
    }
}
