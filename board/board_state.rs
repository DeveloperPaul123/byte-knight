use crate::{
    definitions::{CastlingAvailability, Side},
    moves::Move,
    zobrist::ZobristHash,
};

#[derive(Debug, Clone, Copy)]
pub struct BoardState {
    pub half_move_clock: u32,
    pub full_move_number: u32,
    pub side_to_move: Side,
    pub en_passant_square: Option<u8>,
    pub castling_rights: u8,
    pub zobrist_hash: ZobristHash,
    pub next_move: Move,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            half_move_clock: 0,
            full_move_number: 1,
            side_to_move: Side::White,
            en_passant_square: None,
            castling_rights: CastlingAvailability::NONE,
            zobrist_hash: 0,
            next_move: Move::default(),
        }
    }
}
