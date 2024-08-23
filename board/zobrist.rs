use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::definitions::NumberOf;

pub type ZobristHash = u64;

pub struct ZobristRandomValues {
    pub piece_values: [[[u64; NumberOf::SQUARES]; NumberOf::PIECE_TYPES]; NumberOf::SIDES],
    pub castling_values: [u64; NumberOf::CASTLING_OPTIONS],
    pub en_passant_values: [u64; NumberOf::SQUARES + 1],
    pub side_values: [u64; NumberOf::SIDES],
}

const RANDOM_SEED: [u8; 32] = [115; 32];

impl ZobristRandomValues {
    pub fn new() -> Self {
        let mut random = StdRng::from_seed(RANDOM_SEED);
        // initialize everything to 0
        let mut random_values = Self {
            piece_values: [[[0; NumberOf::SQUARES]; NumberOf::PIECE_TYPES]; NumberOf::SIDES],
            castling_values: [0; NumberOf::CASTLING_OPTIONS],
            en_passant_values: [0; NumberOf::SQUARES + 1],
            side_values: [0; NumberOf::SIDES],
        };

        random_values
            .piece_values
            .iter_mut()
            .for_each(|piece_values| {
                piece_values.iter_mut().for_each(|square_values| {
                    square_values.iter_mut().for_each(|value| {
                        *value = random.gen();
                    });
                });
            });

        random_values.castling_values.iter_mut().for_each(|value| {
            *value = random.gen();
        });

        random_values
            .en_passant_values
            .iter_mut()
            .for_each(|value| {
                *value = random.gen();
            });

        random_values.side_values.iter_mut().for_each(|value| {
            *value = random.gen();
        });

        return random_values;
    }

    pub fn get_piece_value(&self, piece: usize, side: usize, square: usize) -> u64 {
        return self.piece_values[side][piece][square];
    }

    pub fn get_castling_value(&self, castling: usize) -> u64 {
        return self.castling_values[castling];
    }

    pub fn get_en_passant_value(&self, square: Option<u8>) -> u64 {
        match square {
            None => return 0,
            Some(square) => return self.en_passant_values[square as usize],
        }
    }

    pub fn get_side_value(&self, side: usize) -> u64 {
        return self.side_values[side];
    }
}
