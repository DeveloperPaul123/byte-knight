use std::fmt::Display;
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

impl Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "state {{ half_move_clock: {}, full_move_number: {}, side_to_move: {:?}, en_passant_square: {:?}, castling_rights: {:?}, zobrist_hash: {}, next_move: {} }}",
            self.half_move_clock,
            self.full_move_number,
            self.side_to_move,
            self.en_passant_square,
            self.castling_rights,
            self.zobrist_hash,
            self.next_move.to_long_algebraic()
        )
    }
}
