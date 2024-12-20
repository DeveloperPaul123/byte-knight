use crate::tuner::TableType;
use anyhow::Result;
use chess::{definitions::NumberOf, pieces::Piece};

/// Start
pub(crate) type Offset = usize;

#[derive(Debug, Clone)]
pub(crate) struct Offsets {
    table: Vec<Offset>,
}

const MG_PAWN_TABLE_INDEX: usize = 0;
const EG_PAWN_TABLE_INDEX: usize = 1;
const MG_KNIGHT_TABLE_INDEX: usize = 2;
const EG_KNIGHT_TABLE_INDEX: usize = 3;
const MG_BISHOP_TABLE_INDEX: usize = 4;
const EG_BISHOP_TABLE_INDEX: usize = 5;
const MG_ROOK_TABLE_INDEX: usize = 6;
const EG_ROOK_TABLE_INDEX: usize = 7;
const MG_QUEEN_TABLE_INDEX: usize = 8;
const EG_QUEEN_TABLE_INDEX: usize = 9;
const MG_KING_TABLE_INDEX: usize = 10;
const EG_KING_TABLE_INDEX: usize = 11;

impl Offsets {
    pub(crate) fn new() -> Self {
        Self {
            table: vec![
                0,  // MG_PAWN_TABLE start
                64,  // EG_PAWN_TABLE start
                128, // MG_KNIGHT_TABLE start
                192, // EG_KNIGHT_TABLE start
                256, // MG_BISHOP_TABLE start
                320, // EG_BISHOP_TABLE start
                384, // MG_ROOK_TABLE start
                448, // EG_ROOK_TABLE start
                512, // MG_QUEEN_TABLE start
                576, // EG_QUEEN_TABLE start
                640, // MG_KING_TABLE start
                704, // EG_KING_TABLE start
            ],
        }
    }

    pub(crate) fn total_size(&self) -> usize {
        self.table.last().unwrap() + NumberOf::SQUARES
    }

    /// Calculate the start index of a table given the piece and table type
    pub(crate) fn start_index_for_piece(
        &self,
        piece: Piece,
        table_type: TableType,
    ) -> Result<usize> {
        match piece {
            Piece::King => {
                if table_type == TableType::Midgame {
                    Ok(self.table[MG_KING_TABLE_INDEX])
                } else {
                    Ok(self.table[EG_KING_TABLE_INDEX])
                }
            }
            Piece::Queen => {
                if table_type == TableType::Midgame {
                    Ok(self.table[MG_QUEEN_TABLE_INDEX])
                } else {
                    Ok(self.table[EG_QUEEN_TABLE_INDEX])
                }
            }
            Piece::Rook => {
                if table_type == TableType::Midgame {
                    Ok(self.table[MG_ROOK_TABLE_INDEX])
                } else {
                    Ok(self.table[EG_ROOK_TABLE_INDEX])
                }
            }
            Piece::Bishop => {
                if table_type == TableType::Midgame {
                    Ok(self.table[MG_BISHOP_TABLE_INDEX])
                } else {
                    Ok(self.table[EG_BISHOP_TABLE_INDEX])
                }
            }
            Piece::Knight => {
                if table_type == TableType::Midgame {
                    Ok(self.table[MG_KNIGHT_TABLE_INDEX])
                } else {
                    Ok(self.table[EG_KNIGHT_TABLE_INDEX])
                }
            }
            Piece::Pawn => {
                if table_type == TableType::Midgame {
                    Ok(self.table[MG_PAWN_TABLE_INDEX])
                } else {
                    Ok(self.table[EG_PAWN_TABLE_INDEX])
                }
            }
            Piece::None => Err(anyhow::anyhow!("Invalid piece")),
        }
    }
}

impl Default for Offsets {
    fn default() -> Self {
        Self::new()
    }
}
