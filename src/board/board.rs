use crate::definitions::NumberOf;

use super::bitboard::Bitboard;

struct Board {
    bitboards: [Bitboard; NumberOf::PIECE_TYPES * NumberOf::SIDES],
}

impl Board {
    fn new() -> Self {
        Board {
            bitboards: [
                Bitboard::new(0x00000000000000FF), // Pawns
                Bitboard::new(0x0000000000000000), // Knights
                Bitboard::new(0x0000000000000000), // Bishops
                Bitboard::new(0x0000000000000000), // Rooks
                Bitboard::new(0x0000000000000000), // Queens
                Bitboard::new(0x0000000000000000), // Kings
                Bitboard::new(0xFF00000000000000), // Pawns
                Bitboard::new(0x0000000000000000), // Knights
                Bitboard::new(0x0000000000000000), // Bishops
                Bitboard::new(0x0000000000000000), // Rooks
                Bitboard::new(0x0000000000000000), // Queens
                Bitboard::new(0x0000000000000000), // Kings
            ],
        }
    }
}
