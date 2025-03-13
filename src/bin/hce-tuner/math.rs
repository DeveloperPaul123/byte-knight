
pub(crate) fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-x))
}
