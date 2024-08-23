use crate::bitboard::Bitboard;

pub fn next_bit(bitboard: &mut Bitboard) -> usize {
    let square = bitboard.as_number().trailing_zeros();
    *bitboard ^= 1u64 << square;
    square as usize
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_next_bit() {
        use super::*;
        let mut bb = Bitboard::new(0x8000000000000001);
        assert_eq!(next_bit(&mut bb), 0);
        assert_eq!(next_bit(&mut bb), 63);
        assert_eq!(bb.as_number(), 0);

        {
            let mut bb = Bitboard::new(0xFFFFFFFFFFFFFFFF);
            for i in 0..64 {
                assert_eq!(next_bit(&mut bb), i);
            }
        }
    }
}
