use crate::{
    bitboard::Bitboard,
    magics::{BISHOP_ATTACKS, BISHOP_MAGICS, ROOK_ATTACKS, ROOK_MAGICS},
};

/// Calculate diagonal ray attacks for a given square and occupancy.
/// # Arguments
/// * `square` - The square to calculate attacks for (0-63).
/// * `occupied` - The occupancy bitboard.
///
/// # Returns
/// * A [`Bitboard`] representing the diagonal ray attacks from the given square.
pub(crate) const fn diagonal_ray_attacks(square: u8, occupied: u64) -> Bitboard {
    let mut attacks = 0u64;
    let bb = square as u64;

    // Northeast
    let mut ray = bb;
    while ray % 8 < 7 && ray / 8 < 7 {
        ray += 9;
        let ray_bb = 1u64 << ray;
        attacks |= ray_bb;
        if ray_bb & occupied != 0 {
            break;
        }
    }

    // Northwest
    let mut ray = bb;
    while ray % 8 > 0 && ray / 8 < 7 {
        ray += 7;
        let ray_bb = 1u64 << ray;
        attacks |= ray_bb;
        if ray_bb & occupied != 0 {
            break;
        }
    }

    // Southeast
    let mut ray = bb;
    while ray % 8 < 7 && ray / 8 >= 1 {
        ray -= 7;
        let ray_bb = 1u64 << ray;
        attacks |= ray_bb;
        if ray_bb & occupied != 0 {
            break;
        }
    }

    // Southwest
    let mut ray = bb;
    while ray % 8 > 0 && ray / 8 >= 1 {
        ray -= 9;
        let ray_bb = 1u64 << ray;
        attacks |= ray_bb;
        if ray_bb & occupied != 0 {
            break;
        }
    }

    Bitboard::new(attacks)
}

/// Calculate orthogonal ray attacks for a given square and occupancy.
/// # Arguments
/// * `square` - The square to calculate attacks for (0-63).
/// * `occupied` - The occupancy bitboard.
///
/// # Returns
/// * A [`Bitboard`] representing the orthogonal ray attacks from the given square.
///
/// # Examples
///
/// ```
/// use chess::bitboard::Bitboard;
/// use chess::attacks::orthogonal_ray_attacks;
/// let square = 36; // e5
/// let occupied = Bitboard::from_square(20).as_number(); // e3 is
/// let attacks = orthogonal_ray_attacks(square, occupied);
/// println!("{}", attacks);
/// assert_eq!(attacks.as_number(), 1157443723186929664);
/// ```
#[allow(long_running_const_eval)]
pub(crate) const fn orthogonal_ray_attacks(square: u8, occupied: u64) -> Bitboard {
    let mut attacks = 0u64;
    let bb = square as u64;

    let mut ray = bb;
    // North
    while ray / 8 < 7 {
        ray += 8;
        attacks |= 1 << ray;
        if occupied & (1u64 << ray) != 0 {
            break;
        }
    }

    // South
    let mut ray = bb;
    while ray / 8 >= 1 {
        ray -= 8;
        attacks |= 1 << ray;
        if (1 << ray) & occupied != 0 {
            break;
        }
    }

    // East
    let mut ray = bb;
    while ray % 8 < 7 {
        ray += 1;
        attacks |= 1 << ray;
        if (1 << ray) & occupied != 0 {
            break;
        }
    }

    // West
    let mut ray = bb;
    while ray % 8 > 0 {
        ray -= 1;
        attacks |= 1 << ray;
        if (1 << ray) & occupied != 0 {
            break;
        }
    }

    Bitboard::new(attacks)
}

/// Get rook attacks for a given "from" square and board occupancy.
///
/// # Arguments
/// * `square` - The square the rook currently occupies.
/// * `occupancy` - The current occupancy of the board.
///
/// # Returns
/// * A [`Bitboard`] representing all the valid attacks for a rook at the given `square` with the given occupancy.
///
pub const fn rook(square: u8, occupancy: Bitboard) -> Bitboard {
    let magic = ROOK_MAGICS[square as usize];
    let index = magic.index(occupancy);
    ROOK_ATTACKS[index]
}

/// Get bishop attacks for a given "from" square and board occupancy.
///
/// # Arguments
/// * `square` - The square the bishop currently occupies.
/// * `occupancy` - The current occupancy of the board.
///
/// # Returns
/// * A [`Bitboard`] representing all the valid attacks for a bishop at the given square with the given occupancy.
pub const fn bishop(square: u8, occupancy: Bitboard) -> Bitboard {
    let magic = BISHOP_MAGICS[square as usize];
    let index = magic.index(occupancy);
    BISHOP_ATTACKS[index]
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::NumberOf,
        magics::{BISHOP_ATTACKS, BISHOP_MAGICS, ROOK_ATTACKS, ROOK_MAGICS},
        move_generation::MoveGenerator,
    };

    #[test]
    fn test_diagonal_ray_attacks() {
        const EXPECTED_ATTACKS: [u64; NumberOf::SQUARES] = [
            0x8040201008040200,
            0x80402010080500,
            0x804020110a00,
            0x8041221400,
            0x182442800,
            0x10204885000,
            0x102040810a000,
            0x102040810204000,
            0x4020100804020002,
            0x8040201008050005,
            0x804020110a000a,
            0x804122140014,
            0x18244280028,
            0x1020488500050,
            0x102040810a000a0,
            0x204081020400040,
            0x2010080402000204,
            0x4020100805000508,
            0x804020110a000a11,
            0x80412214001422,
            0x1824428002844,
            0x102048850005088,
            0x2040810a000a010,
            0x408102040004020,
            0x1008040200020408,
            0x2010080500050810,
            0x4020110a000a1120,
            0x8041221400142241,
            0x182442800284482,
            0x204885000508804,
            0x40810a000a01008,
            0x810204000402010,
            0x804020002040810,
            0x1008050005081020,
            0x20110a000a112040,
            0x4122140014224180,
            0x8244280028448201,
            0x488500050880402,
            0x810a000a0100804,
            0x1020400040201008,
            0x402000204081020,
            0x805000508102040,
            0x110a000a11204080,
            0x2214001422418000,
            0x4428002844820100,
            0x8850005088040201,
            0x10a000a010080402,
            0x2040004020100804,
            0x200020408102040,
            0x500050810204080,
            0xa000a1120408000,
            0x1400142241800000,
            0x2800284482010000,
            0x5000508804020100,
            0xa000a01008040201,
            0x4000402010080402,
            0x2040810204080,
            0x5081020408000,
            0xa112040800000,
            0x14224180000000,
            0x28448201000000,
            0x50880402010000,
            0xa0100804020100,
            0x40201008040201,
        ];

        for (sq, &expected) in EXPECTED_ATTACKS.iter().enumerate() {
            let attacks: crate::bitboard::Bitboard = super::diagonal_ray_attacks(sq as u8, 0);
            println!("Square: {}\nAttacks:\n{}", sq, attacks);
            assert_eq!(attacks.as_number(), expected);
        }
    }

    #[test]
    fn test_orthogonal_ray_attacks() {
        const EXPECTED_ATTACKS: [u64; NumberOf::SQUARES] = [
            0x1010101010101fe,
            0x2020202020202fd,
            0x4040404040404fb,
            0x8080808080808f7,
            0x10101010101010ef,
            0x20202020202020df,
            0x40404040404040bf,
            0x808080808080807f,
            0x10101010101fe01,
            0x20202020202fd02,
            0x40404040404fb04,
            0x80808080808f708,
            0x101010101010ef10,
            0x202020202020df20,
            0x404040404040bf40,
            0x8080808080807f80,
            0x101010101fe0101,
            0x202020202fd0202,
            0x404040404fb0404,
            0x808080808f70808,
            0x1010101010ef1010,
            0x2020202020df2020,
            0x4040404040bf4040,
            0x80808080807f8080,
            0x1010101fe010101,
            0x2020202fd020202,
            0x4040404fb040404,
            0x8080808f7080808,
            0x10101010ef101010,
            0x20202020df202020,
            0x40404040bf404040,
            0x808080807f808080,
            0x10101fe01010101,
            0x20202fd02020202,
            0x40404fb04040404,
            0x80808f708080808,
            0x101010ef10101010,
            0x202020df20202020,
            0x404040bf40404040,
            0x8080807f80808080,
            0x101fe0101010101,
            0x202fd0202020202,
            0x404fb0404040404,
            0x808f70808080808,
            0x1010ef1010101010,
            0x2020df2020202020,
            0x4040bf4040404040,
            0x80807f8080808080,
            0x1fe010101010101,
            0x2fd020202020202,
            0x4fb040404040404,
            0x8f7080808080808,
            0x10ef101010101010,
            0x20df202020202020,
            0x40bf404040404040,
            0x807f808080808080,
            0xfe01010101010101,
            0xfd02020202020202,
            0xfb04040404040404,
            0xf708080808080808,
            0xef10101010101010,
            0xdf20202020202020,
            0xbf40404040404040,
            0x7f80808080808080,
        ];

        for sq in 0..64_u8 {
            let attacks: crate::bitboard::Bitboard = super::orthogonal_ray_attacks(sq, 0);
            println!("Square: {}\nAttacks:\n{}", sq, attacks);
            assert_eq!(attacks.as_number(), EXPECTED_ATTACKS[sq as usize])
        }
    }

    #[test]
    fn validate_rook_attack_table() {
        for sq in 0..64_u8 {
            let magic = ROOK_MAGICS[sq as usize];
            let relevant_bits = MoveGenerator::relevant_rook_bits(sq);
            let blockers_list = MoveGenerator::create_blocker_permutations(relevant_bits);
            for blockers in blockers_list {
                let idx = magic.index(blockers);
                let table_attack = ROOK_ATTACKS[idx];
                let expected_attack = super::orthogonal_ray_attacks(sq, blockers.as_number());
                println!(
                    "Square: {}, Blockers: {:064b}\nTable Attack:\n{}\nExpected Attack:\n{}",
                    sq,
                    blockers.as_number(),
                    table_attack,
                    expected_attack
                );
                assert_eq!(
                    table_attack,
                    expected_attack,
                    "Mismatch on square {} with blockers {:064b}\nTable Attack:\n{}\nExpected Attack:\n{}",
                    sq,
                    blockers.as_number(),
                    table_attack,
                    expected_attack
                );
            }
        }
    }

    #[test]
    fn validate_bishop_attack_table() {
        for sq in 0..64_u8 {
            let magic = BISHOP_MAGICS[sq as usize];
            let relevant_bits = MoveGenerator::relevant_bishop_bits(sq);
            let blockers_list = MoveGenerator::create_blocker_permutations(relevant_bits);
            for blockers in blockers_list {
                let idx = magic.index(blockers);
                let table_attack = BISHOP_ATTACKS[idx];
                let expected_attack = super::diagonal_ray_attacks(sq, blockers.as_number());
                println!(
                    "Square: {}, Blockers: {:064b}\nTable Attack:\n{}\nExpected Attack:\n{}",
                    sq,
                    blockers.as_number(),
                    table_attack,
                    expected_attack
                );
                assert_eq!(
                    table_attack,
                    expected_attack,
                    "Mismatch on square {} with blockers {:064b}\nTable Attack:\n{}\nExpected Attack:\n{}",
                    sq,
                    blockers.as_number(),
                    table_attack,
                    expected_attack
                );
            }
        }
    }
}
