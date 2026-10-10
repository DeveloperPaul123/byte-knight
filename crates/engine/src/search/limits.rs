use std::{fmt::Display, time::Duration};

use chess::board::Board;
use uci_parser::UciSearchOptions;

use crate::defs::MAX_DEPTH;

pub const UCI_OVERHEAD_MS: u64 = 100;
const BEST_MOVE_TIME_BASE: f32 = 1.1;
const BEST_MOVE_TIME_FACTOR: f32 = 0.05;
const BEST_MOVE_TIME_MIN_SCALE: f32 = 0.5;

const EVAL_STABILITY_BASE: f32 = 1.2664;
const EVAL_STABILITY_FACTOR: f32 = 0.0416;
const EVAL_STABILITY_MIN_SCALE: f32 = 0.8642;

const NODE_TM_BASE: f32 = 1.45;
const NODE_TM_SCALE: f32 = 1.35;
pub(crate) const NODE_TM_MIN_DEPTH: i32 = 4;

/// Per-iteration signals that scale the soft time limit. A `None` field leaves
/// the limit unscaled by that signal.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct SoftLimitInputs {
    pub best_move_stability: Option<u64>,
    pub eval_stability: Option<u64>,
    pub best_move_node_fraction: Option<f32>,
}

/// Input parameters for the search.
#[derive(Clone, Debug, Copy)]
pub struct SearchLimits {
    pub max_depth: u8,
    pub soft_timeout: Duration,
    pub hard_timeout: Duration,
    pub max_nodes: u64,
}

impl Default for SearchLimits {
    fn default() -> Self {
        SearchLimits {
            max_depth: MAX_DEPTH,
            soft_timeout: Duration::MAX,
            hard_timeout: Duration::MAX,
            max_nodes: u64::MAX,
        }
    }
}

impl SearchLimits {
    /// Creates a new set of search parameters from the UCI options and the current board.
    pub fn new(uci_options: &UciSearchOptions, board: &Board) -> Self {
        let mut params = Self::default();
        if let Some(depth) = uci_options.depth {
            params.max_depth = depth as u8;
        }

        if let Some(nodes) = uci_options.nodes {
            params.max_nodes = nodes as u64;
        }

        if let Some(time) = uci_options.movetime {
            params.soft_timeout = time;
            params.hard_timeout = time;
        } else {
            // Get the time and increment
            let (time, increment) = if board.side_to_move().is_white() {
                (uci_options.wtime, uci_options.winc)
            } else {
                (uci_options.btime, uci_options.binc)
            };

            // Calculate our soft/hard limits
            if let Some(time) = time {
                let inc = increment.unwrap_or(Duration::ZERO);
                let (hard, soft) = SearchLimits::calculate_time_limits(time, inc);
                params.soft_timeout = soft;
                params.hard_timeout = hard;
            }
        }

        params
    }

    pub(crate) fn scaled_soft_limit(&self, inputs: SoftLimitInputs) -> Option<Duration> {
        // No time control set (depth/nodes-only search): don't scale.
        if self.soft_timeout.is_zero() || self.soft_timeout == Duration::MAX {
            return None;
        }

        let best_move_stability_scale = match inputs.best_move_stability {
            Some(value) => Self::best_move_stability_scale(value),
            None => 1.0,
        };

        let eval_stability_scale = match inputs.eval_stability {
            Some(value) => Self::eval_stability_scale(value),
            None => 1.0,
        };

        let node_tm_stability_scale = match inputs.best_move_node_fraction {
            Some(value) => Self::node_stability_scale(value),
            None => 1.0,
        };

        let scaled = self.soft_timeout.as_secs_f32()
            * best_move_stability_scale
            * eval_stability_scale
            * node_tm_stability_scale;

        Some(Duration::from_secs_f32(scaled))
    }

    fn best_move_stability_scale(best_move_stability: u64) -> f32 {
        (BEST_MOVE_TIME_BASE - BEST_MOVE_TIME_FACTOR * best_move_stability as f32)
            .max(BEST_MOVE_TIME_MIN_SCALE)
    }

    fn eval_stability_scale(eval_stability: u64) -> f32 {
        (EVAL_STABILITY_BASE - EVAL_STABILITY_FACTOR * eval_stability as f32)
            .max(EVAL_STABILITY_MIN_SCALE)
    }

    fn node_stability_scale(best_move_fraction: f32) -> f32 {
        (NODE_TM_BASE - best_move_fraction) * NODE_TM_SCALE
    }

    fn calculate_time_limits(time: Duration, inc: Duration) -> (Duration, Duration) {
        let overhead = Duration::from_millis(UCI_OVERHEAD_MS);
        let budget = time.saturating_sub(overhead);
        let soft = budget / 20 + inc / 2;
        let hard = (budget / 5 + inc / 2).min(budget).max(soft);

        (hard, soft)
    }
}

impl Display for SearchLimits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "max depth {} soft_timeout {:?} hard_timeout {:?}",
            self.max_depth, self.soft_timeout, self.hard_timeout
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_basic_fischer_limits(time: Duration, inc: Duration) -> SearchLimits {
        let board = Board::default_board();
        let opts = UciSearchOptions {
            wtime: Some(time),
            winc: Some(inc),
            btime: Some(time),
            binc: Some(inc),
            ..Default::default()
        };

        SearchLimits::new(&opts, &board)
    }

    #[test]
    fn time_limits_typical() {
        let (hard, soft) = SearchLimits::calculate_time_limits(
            Duration::from_millis(100),
            Duration::from_millis(100),
        );
        assert!(soft > Duration::ZERO);
        assert!(hard > Duration::ZERO);
        assert!(hard >= soft);
    }

    #[test]
    fn time_limits_no_increment() {
        let (hard, soft) =
            SearchLimits::calculate_time_limits(Duration::from_millis(10), Duration::ZERO);
        assert!(hard >= soft);
    }

    #[test]
    fn time_limits_zero_time() {
        // Must not panic; both limits may be zero but that is acceptable.
        let (hard, soft) = SearchLimits::calculate_time_limits(Duration::ZERO, Duration::ZERO);
        let _ = (hard, soft);
    }

    #[test]
    fn stability_scale_initial() {
        assert!(
            (SearchLimits::best_move_stability_scale(0) - BEST_MOVE_TIME_BASE).abs() < f32::EPSILON
        );
    }

    #[test]
    fn stability_scale_middle() {
        // stability=5: 1.1 - 0.05*5 = 0.85 (neutral point)
        assert!((SearchLimits::best_move_stability_scale(5) - 0.85).abs() < f32::EPSILON);
    }

    #[test]
    fn initial_scaled_soft_limits() {
        let limits = create_basic_fischer_limits(Duration::from_secs(10), Duration::from_secs(1));
        // Make sure our initial scaled soft limits make sense at the start of a search where we have no data.
        let duration = limits.scaled_soft_limit(SoftLimitInputs::default());
        // Should be no scaling without any data.
        assert!(duration.is_some_and(|val| {
            (val.as_secs_f32() - limits.soft_timeout.as_secs_f32()).abs() < f32::EPSILON
        }));
    }

    #[test]
    fn scaled_soft_limits_with_stability() {
        let limits = create_basic_fischer_limits(Duration::from_secs(10), Duration::from_secs(1));
        // Same scaling as stability_scale_middle test, so we expect a 15% reduction in time.
        let duration = limits.scaled_soft_limit(SoftLimitInputs {
            best_move_stability: Some(5),
            ..Default::default()
        });
        assert!(duration.is_some_and(|val| {
            (val.as_secs_f32() - limits.soft_timeout.as_secs_f32() * 0.85).abs() < f32::EPSILON
        }));
    }

    #[test]
    fn scaled_soft_limits_with_eval_stability() {
        let limits = create_basic_fischer_limits(Duration::from_secs(10), Duration::from_secs(1));
        let duration = limits.scaled_soft_limit(SoftLimitInputs {
            eval_stability: Some(12),
            ..Default::default()
        });
        assert!(duration.is_some_and(|val| {
            (val.as_secs_f32() - limits.soft_timeout.as_secs_f32() * 0.8642).abs() < f32::EPSILON
        }));
    }

    #[test]
    fn stability_scale_clamp() {
        // At stability=100 the raw value would be deeply negative; must clamp to floor.
        assert!(SearchLimits::best_move_stability_scale(100) >= BEST_MOVE_TIME_MIN_SCALE);
    }

    #[test]
    fn eval_stability_scale_values() {
        // stability=0: no stable iterations yet, so the full base scale.
        assert!((SearchLimits::eval_stability_scale(0) - EVAL_STABILITY_BASE).abs() < 1e-5);
        // stability=5: 1.2664 - 0.0416*5 = 1.0584
        assert!((SearchLimits::eval_stability_scale(5) - 1.0584).abs() < 1e-5);
        // stability=100: clamped to the floor.
        assert!(
            (SearchLimits::eval_stability_scale(100) - EVAL_STABILITY_MIN_SCALE).abs()
                < f32::EPSILON
        );
    }

    #[test]
    fn scaled_soft_limits_with_both_stabilities() {
        let limits = create_basic_fischer_limits(Duration::from_secs(10), Duration::from_secs(1));
        let duration = limits.scaled_soft_limit(SoftLimitInputs {
            best_move_stability: Some(5),
            eval_stability: Some(5),
            ..Default::default()
        });
        let expected = limits.soft_timeout.as_secs_f32()
            * SearchLimits::best_move_stability_scale(5)
            * SearchLimits::eval_stability_scale(5);
        assert!(duration.is_some_and(|val| (val.as_secs_f32() - expected).abs() < 1e-4));
    }

    #[test]
    fn node_stability_scale_values() {
        // (1.45 - f) * 1.35
        assert!((SearchLimits::node_stability_scale(0.0) - 1.9575).abs() < 1e-5);
        assert!((SearchLimits::node_stability_scale(0.5) - 1.2825).abs() < 1e-5);
        assert!((SearchLimits::node_stability_scale(1.0) - 0.6075).abs() < 1e-5);
    }

    #[test]
    fn scaled_soft_limits_with_all_inputs() {
        let limits = create_basic_fischer_limits(Duration::from_secs(10), Duration::from_secs(1));
        let duration = limits.scaled_soft_limit(SoftLimitInputs {
            best_move_stability: Some(5),
            eval_stability: Some(5),
            best_move_node_fraction: Some(0.8),
        });
        let expected = limits.soft_timeout.as_secs_f32()
            * SearchLimits::best_move_stability_scale(5)
            * SearchLimits::eval_stability_scale(5)
            * SearchLimits::node_stability_scale(0.8);
        assert!(duration.is_some_and(|val| (val.as_secs_f32() - expected).abs() < 1e-4));
    }

    #[test]
    fn scaled_soft_limits_with_node_fraction_only() {
        let limits = create_basic_fischer_limits(Duration::from_secs(10), Duration::from_secs(1));
        let duration = limits.scaled_soft_limit(SoftLimitInputs {
            best_move_node_fraction: Some(0.5),
            ..Default::default()
        });
        let expected = limits.soft_timeout.as_secs_f32() * 1.2825;
        assert!(duration.is_some_and(|val| (val.as_secs_f32() - expected).abs() < 1e-4));
    }

    #[test]
    fn new_with_time_no_increment_gives_finite_limits() {
        use chess::board::Board;
        use uci_parser::UciSearchOptions;

        let board = Board::default_board();
        let opts = UciSearchOptions {
            wtime: Some(Duration::from_secs(10)),
            ..Default::default()
        };
        // winc intentionally left as None

        let limits = SearchLimits::new(&opts, &board);
        assert_ne!(
            limits.soft_timeout,
            Duration::MAX,
            "soft_timeout should be finite"
        );
        assert_ne!(
            limits.hard_timeout,
            Duration::MAX,
            "hard_timeout should be finite"
        );
    }

    #[test]
    fn new_with_all_params() {
        use chess::board::Board;
        use uci_parser::UciSearchOptions;

        let board = Board::default_board();
        let opts = UciSearchOptions {
            movetime: Some(Duration::from_secs(10)),
            depth: Some(15),
            nodes: Some(30_000),
            ..Default::default()
        };

        let limits = SearchLimits::new(&opts, &board);
        assert_eq!(limits.max_nodes, 30_000);
        assert_eq!(limits.max_depth, 15);
        // These should be equal because we used movetime
        assert_eq!(limits.hard_timeout, limits.soft_timeout);
    }

    #[test]
    fn new_fischer_time_same_both_sides() {
        use chess::board::Board;
        use uci_parser::UciSearchOptions;

        let mut board = Board::default_board();
        let opts = UciSearchOptions {
            wtime: Some(Duration::from_secs(10)),
            winc: Some(Duration::from_secs(1)),
            btime: Some(Duration::from_secs(10)),
            binc: Some(Duration::from_secs(1)),
            ..Default::default()
        };

        let limits_w = SearchLimits::new(&opts, &board);
        // Make a move to change side to move
        board.make_uci_move("e2e4").unwrap();

        let limits_b = SearchLimits::new(&opts, &board);

        assert_eq!(limits_w.soft_timeout, limits_b.soft_timeout);
        assert_eq!(limits_w.hard_timeout, limits_b.hard_timeout);
    }
}
