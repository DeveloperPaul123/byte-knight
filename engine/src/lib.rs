#![deny(clippy::unused_result_ok)]
#![deny(clippy::panic)]
#![deny(clippy::expect_used)]

pub mod aspiration_window;
pub mod defs;
pub mod engine;
pub mod evaluation;
pub mod hce_values;
pub mod history_table;
mod inplace_incremental_sort;
pub mod input_handler;
mod lmr;
pub mod log_level;
mod move_order;
pub(crate) mod node_types;
pub mod pawn_structure;
pub mod phased_score;
pub(crate) mod principle_variation;
pub mod score;
pub mod search;
pub mod search_thread;
pub(crate) mod table;
pub mod traits;
pub mod ttable;
pub mod tuneable;
