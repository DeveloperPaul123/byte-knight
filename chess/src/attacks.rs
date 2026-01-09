// Part of the byte-knight project.
// Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
// GNU General Public License v3.0 or later
// https://www.gnu.org/licenses/gpl-3.0-standalone.html

//! This module defines functions to define and retrieve attacks for all chess pieces.
//! To the extent possible, attacks are generated at compile time to avoid runtime computation.
//! Sliding piece attacks use so called "magic" numbers/bitboards unless the PEXT instruction
//! set is available and enabled.

use crate::{
    bitboard::Bitboard,
    bitboard_helpers,
    definitions::NumberOf,
    file::File,
    magics::{BISHOP_MAGICS, ROOK_MAGICS},
    side::Side,
};

#[allow(long_running_const_eval)]
pub(crate) static ROOK_ATTACKS: [Bitboard; 102400] = generate_rook_attacks();

#[allow(long_running_const_eval)]
pub(crate) static BISHOP_ATTACKS: [Bitboard; 5248] = generate_bishop_attacks();

/// Compile time helper function to generate all possible bishop attacks.
///
/// This will generate all possible attacks using the bishop magic numbers as defined in the magics module.
const fn generate_bishop_attacks() -> [Bitboard; 5248] {
    let mut table = [Bitboard::default(); 5248];
    let mut sq = 0u8;
    while sq < NumberOf::SQUARES as u8 {
        let magic = BISHOP_MAGICS[sq as usize];

        let mut subset = Bitboard::default();

        let attacks = diagonal_ray_attacks(sq, subset.as_number());
        let blockers = subset;
        let idx = magic.index(blockers);
        table[idx] = attacks;

        // Update the subset (Carry-Rippler method)
        subset = Bitboard::new(
            subset.as_number().wrapping_sub(magic.relevant_bits_mask) & magic.relevant_bits_mask,
        );

        // Repeat for all subsets until subset is zero
        while subset.as_number() != 0 {
            let attacks = diagonal_ray_attacks(sq, subset.as_number());
            let blockers = subset;
            let idx = magic.index(blockers);
            table[idx] = attacks;
            // Update the subset (Carry-Rippler method) - same as above
            subset = Bitboard::new(
                subset.as_number().wrapping_sub(magic.relevant_bits_mask)
                    & magic.relevant_bits_mask,
            );
        }

        sq += 1;
    }

    table
}

/// Compile time helper function to generate rook attacks.
///
/// This will generate all possible rook attacks using the rook magic numbers as defined in the magics module.
const fn generate_rook_attacks() -> [Bitboard; 102400] {
    let mut table = [Bitboard::default(); 102400];
    let mut sq = 0u8;
    while sq < NumberOf::SQUARES as u8 {
        let magic = ROOK_MAGICS[sq as usize];

        let mut subset = Bitboard::default();

        let attacks = orthogonal_ray_attacks(sq, subset.as_number());
        let blockers = subset;
        let idx = magic.index(blockers);
        table[idx] = attacks;

        // Update the subset (Carry-Rippler method)
        subset = Bitboard::new(
            subset.as_number().wrapping_sub(magic.relevant_bits_mask) & magic.relevant_bits_mask,
        );

        // Repeat for all subsets until subset is zero
        while subset.as_number() != 0 {
            let attacks = orthogonal_ray_attacks(sq, subset.as_number());
            let blockers = subset;
            let idx = magic.index(blockers);
            table[idx] = attacks;
            // Update the subset (Carry-Rippler method) - same as above
            subset = Bitboard::new(
                subset.as_number().wrapping_sub(magic.relevant_bits_mask)
                    & magic.relevant_bits_mask,
            );
        }

        sq += 1;
    }

    table
}

/// Calculate diagonal ray attacks for a given square and occupancy.
/// # Arguments
/// - `square` - The square to calculate attacks for (0-63).
/// - `occupied` - The occupancy bitboard.
///
/// # Returns
/// - A [`Bitboard`] representing the diagonal ray attacks from the given square.
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
    while !ray.is_multiple_of(8) && ray / 8 < 7 {
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
    while !ray.is_multiple_of(8) && ray / 8 >= 1 {
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
/// - `square` - The square to calculate attacks for (0-63).
/// - `occupied` - The occupancy bitboard.
///
/// # Returns
/// - A [`Bitboard`] representing the orthogonal ray attacks from the given square.
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
    while !ray.is_multiple_of(8) {
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
/// - `square` - The square the rook currently occupies.
/// - `occupancy` - The current occupancy of the board.
///
/// # Returns
/// - A [`Bitboard`] representing all the valid attacks for a rook at the given `square` with the given occupancy.
///
pub const fn rook(square: u8, occupancy: Bitboard) -> Bitboard {
    let magic = ROOK_MAGICS[square as usize];
    let index = magic.index(occupancy);
    ROOK_ATTACKS[index]
}

/// Get bishop attacks for a given "from" square and board occupancy.
///
/// # Arguments
/// - `square` - The square the bishop currently occupies.
/// - `occupancy` - The current occupancy of the board.
///
/// # Returns
/// - A [`Bitboard`] representing all the valid attacks for a bishop at the given square with the given occupancy.
pub const fn bishop(square: u8, occupancy: Bitboard) -> Bitboard {
    let magic = BISHOP_MAGICS[square as usize];
    let index = magic.index(occupancy);
    BISHOP_ATTACKS[index]
}

/// Get queen attacks for a given square and occupancy.
///
/// # Arguments
/// - `square` - The square the queen currently occupies.
/// - `occupancy` - The current occupancy of the board.
///
/// # Returns
/// - A [`Bitboard`] representing all the valid attacks for a queen at the given square with the given occupancy.
pub fn queen(square: u8, occupancy: Bitboard) -> Bitboard {
    rook(square, occupancy) | bishop(square, occupancy)
}

/// Get pawn attacks for a given square and side.
///
/// # Arguments
/// - `square` - The square the pawn currently occupies.
/// - `side` - Which side is the attacking pawn on?
///
/// # Returns
/// - A [`Bitboard`] representing all the valid attacks for a pawn at the given square on the given side.
pub fn pawn(square: u8, side: Side) -> Bitboard {
    let bb = Bitboard::from_square(square);
    match side {
        Side::White => bitboard_helpers::north_west(bb) | bitboard_helpers::north_east(bb),
        Side::Black => bitboard_helpers::south_west(bb) | bitboard_helpers::south_east(bb),
    }
}

const KING_ATTACKS: [Bitboard; NumberOf::SQUARES] = [
    Bitboard::new(0x302),
    Bitboard::new(0x704),
    Bitboard::new(0xe0a),
    Bitboard::new(0x1c14),
    Bitboard::new(0x3828),
    Bitboard::new(0x7050),
    Bitboard::new(0xe020),
    Bitboard::new(0xc140),
    Bitboard::new(0x30283),
    Bitboard::new(0x70407),
    Bitboard::new(0xe0a0e),
    Bitboard::new(0x1c141c),
    Bitboard::new(0x382838),
    Bitboard::new(0x705070),
    Bitboard::new(0xe020e0),
    Bitboard::new(0xc140c0),
    Bitboard::new(0x3028300),
    Bitboard::new(0x7040700),
    Bitboard::new(0xe0a0e00),
    Bitboard::new(0x1c141c00),
    Bitboard::new(0x38283800),
    Bitboard::new(0x70507000),
    Bitboard::new(0xe020e000),
    Bitboard::new(0xc140c000),
    Bitboard::new(0x302830000),
    Bitboard::new(0x704070000),
    Bitboard::new(0xe0a0e0000),
    Bitboard::new(0x1c141c0000),
    Bitboard::new(0x3828380000),
    Bitboard::new(0x7050700000),
    Bitboard::new(0xe020e00000),
    Bitboard::new(0xc140c00000),
    Bitboard::new(0x30283000000),
    Bitboard::new(0x70407000000),
    Bitboard::new(0xe0a0e000000),
    Bitboard::new(0x1c141c000000),
    Bitboard::new(0x382838000000),
    Bitboard::new(0x705070000000),
    Bitboard::new(0xe020e0000000),
    Bitboard::new(0xc140c0000000),
    Bitboard::new(0x3028300000000),
    Bitboard::new(0x7040700000000),
    Bitboard::new(0xe0a0e00000000),
    Bitboard::new(0x1c141c00000000),
    Bitboard::new(0x38283800000000),
    Bitboard::new(0x70507000000000),
    Bitboard::new(0xe020e000000000),
    Bitboard::new(0xc140c000000000),
    Bitboard::new(0x302830000000000),
    Bitboard::new(0x704070000000000),
    Bitboard::new(0xe0a0e0000000000),
    Bitboard::new(0x1c141c0000000000),
    Bitboard::new(0x3828380000000000),
    Bitboard::new(0x7050700000000000),
    Bitboard::new(0xe020e00000000000),
    Bitboard::new(0xc140c00000000000),
    Bitboard::new(0x283000000000000),
    Bitboard::new(0x407000000000000),
    Bitboard::new(0xa0e000000000000),
    Bitboard::new(0x141c000000000000),
    Bitboard::new(0x2838000000000000),
    Bitboard::new(0x5070000000000000),
    Bitboard::new(0x20e0000000000000),
    Bitboard::new(0x40c0000000000000),
];

/// Get king attacks for a given square.
///
/// # Arguments
/// - `square` - The square the king currently occupies.
///
/// # Returns
/// - A [`Bitboard`] representing all the valid attacks for a king at the given square.
pub fn king(square: u8) -> Bitboard {
    assert!(square < NumberOf::SQUARES as u8);
    KING_ATTACKS[square as usize]
}

const KNIGHT_ATTACKS: [Bitboard; NumberOf::SQUARES] = [
    Bitboard::new(0x20400),
    Bitboard::new(0x50800),
    Bitboard::new(0xa1100),
    Bitboard::new(0x142200),
    Bitboard::new(0x284400),
    Bitboard::new(0x508800),
    Bitboard::new(0xa01000),
    Bitboard::new(0x402000),
    Bitboard::new(0x2040004),
    Bitboard::new(0x5080008),
    Bitboard::new(0xa110011),
    Bitboard::new(0x14220022),
    Bitboard::new(0x28440044),
    Bitboard::new(0x50880088),
    Bitboard::new(0xa0100010),
    Bitboard::new(0x40200020),
    Bitboard::new(0x204000402),
    Bitboard::new(0x508000805),
    Bitboard::new(0xa1100110a),
    Bitboard::new(0x1422002214),
    Bitboard::new(0x2844004428),
    Bitboard::new(0x5088008850),
    Bitboard::new(0xa0100010a0),
    Bitboard::new(0x4020002040),
    Bitboard::new(0x20400040200),
    Bitboard::new(0x50800080500),
    Bitboard::new(0xa1100110a00),
    Bitboard::new(0x142200221400),
    Bitboard::new(0x284400442800),
    Bitboard::new(0x508800885000),
    Bitboard::new(0xa0100010a000),
    Bitboard::new(0x402000204000),
    Bitboard::new(0x2040004020000),
    Bitboard::new(0x5080008050000),
    Bitboard::new(0xa1100110a0000),
    Bitboard::new(0x14220022140000),
    Bitboard::new(0x28440044280000),
    Bitboard::new(0x50880088500000),
    Bitboard::new(0xa0100010a00000),
    Bitboard::new(0x40200020400000),
    Bitboard::new(0x204000402000000),
    Bitboard::new(0x508000805000000),
    Bitboard::new(0xa1100110a000000),
    Bitboard::new(0x1422002214000000),
    Bitboard::new(0x2844004428000000),
    Bitboard::new(0x5088008850000000),
    Bitboard::new(0xa0100010a0000000),
    Bitboard::new(0x4020002040000000),
    Bitboard::new(0x400040200000000),
    Bitboard::new(0x800080500000000),
    Bitboard::new(0x1100110a00000000),
    Bitboard::new(0x2200221400000000),
    Bitboard::new(0x4400442800000000),
    Bitboard::new(0x8800885000000000),
    Bitboard::new(0x100010a000000000),
    Bitboard::new(0x2000204000000000),
    Bitboard::new(0x4020000000000),
    Bitboard::new(0x8050000000000),
    Bitboard::new(0x110a0000000000),
    Bitboard::new(0x22140000000000),
    Bitboard::new(0x44280000000000),
    Bitboard::new(0x88500000000000),
    Bitboard::new(0x10a00000000000),
    Bitboard::new(0x20400000000000),
];

#[allow(dead_code)]
/// Helper to generate the knight attacks. This was used to generate the `KNIGHT_ATTACKS` constant at compile time (see above).
fn generate_knight(square: u8) -> Bitboard {
    const NORTH_NORTH_EAST: u64 = 17;
    const WEST_NORTH_WEST: u64 = 6;
    const NORTH_NORTH_WEST: u64 = 15;
    const EAST_NORTH_EAST: u64 = 10;
    const SOUTH_SOUTH_WEST: u64 = 17;
    const WEST_SOUTH_WEST: u64 = 10;
    const SOUTH_SOUTH_EAST: u64 = 15;
    const EAST_SOUTH_EAST: u64 = 6;

    let bb = Bitboard::from_square(square);
    let mut attacks_bb = Bitboard::default();
    // With our Bitboard setup, "east" means right, and "west" means left
    // so this means east means we move more towards the MSB, so shift.
    // So all the east and north moves are shifted left, all south and west moves are shifted right
    let not_h_file = !File::H.to_bitboard();
    let not_gh_file = !File::G.to_bitboard() & !File::H.to_bitboard();
    let not_ab_file = !File::A.to_bitboard() & !File::B.to_bitboard();
    let not_a_file = !File::A.to_bitboard();

    attacks_bb |= (bb & not_h_file) << NORTH_NORTH_EAST;
    attacks_bb |= (bb & not_gh_file) << EAST_NORTH_EAST;
    attacks_bb |= (bb & not_a_file) << NORTH_NORTH_WEST;
    attacks_bb |= (bb & not_ab_file) << WEST_NORTH_WEST;

    attacks_bb |= (bb & not_h_file) >> SOUTH_SOUTH_EAST;
    attacks_bb |= (bb & not_gh_file) >> EAST_SOUTH_EAST;
    attacks_bb |= (bb & not_a_file) >> SOUTH_SOUTH_WEST;
    attacks_bb |= (bb & not_ab_file) >> WEST_SOUTH_WEST;

    attacks_bb
}

/// Get knight attacks for a given square.
///
/// # Arguments
/// - `square` - The square the knight currently occupies.
///
/// # Returns
/// - A [`Bitboard`] representing all the valid attacks for a knight at the given square.
pub fn knight(square: u8) -> Bitboard {
    assert!(square < NumberOf::SQUARES as u8);
    KNIGHT_ATTACKS[square as usize]
}

#[cfg(test)]
mod tests {
    use crate::{
        attacks::{self, BISHOP_ATTACKS, ROOK_ATTACKS},
        bitboard::Bitboard,
        definitions::NumberOf,
        magics::{BISHOP_MAGICS, ROOK_MAGICS},
        move_generation::MoveGenerator,
    };

    const EXPECTED_ORTHOGONAL_ATTACKS: [u64; NumberOf::SQUARES] = [
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

    const EXPECTED_DIAGONAL_ATTACKS: [u64; NumberOf::SQUARES] = [
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

    const EXPECTED_KNIGHT_ATTACKS: [u64; NumberOf::SQUARES] = [
        0x20400,
        0x50800,
        0xa1100,
        0x142200,
        0x284400,
        0x508800,
        0xa01000,
        0x402000,
        0x2040004,
        0x5080008,
        0xa110011,
        0x14220022,
        0x28440044,
        0x50880088,
        0xa0100010,
        0x40200020,
        0x204000402,
        0x508000805,
        0xa1100110a,
        0x1422002214,
        0x2844004428,
        0x5088008850,
        0xa0100010a0,
        0x4020002040,
        0x20400040200,
        0x50800080500,
        0xa1100110a00,
        0x142200221400,
        0x284400442800,
        0x508800885000,
        0xa0100010a000,
        0x402000204000,
        0x2040004020000,
        0x5080008050000,
        0xa1100110a0000,
        0x14220022140000,
        0x28440044280000,
        0x50880088500000,
        0xa0100010a00000,
        0x40200020400000,
        0x204000402000000,
        0x508000805000000,
        0xa1100110a000000,
        0x1422002214000000,
        0x2844004428000000,
        0x5088008850000000,
        0xa0100010a0000000,
        0x4020002040000000,
        0x400040200000000,
        0x800080500000000,
        0x1100110a00000000,
        0x2200221400000000,
        0x4400442800000000,
        0x8800885000000000,
        0x100010a000000000,
        0x2000204000000000,
        0x4020000000000,
        0x8050000000000,
        0x110a0000000000,
        0x22140000000000,
        0x44280000000000,
        0x88500000000000,
        0x10a00000000000,
        0x20400000000000,
    ];

    #[test]
    fn test_generate_bishop_attacks() {
        let generated = super::generate_bishop_attacks();
        for (i, &attack) in BISHOP_ATTACKS.iter().enumerate() {
            assert_eq!(attack, generated[i], "Mismatch at index {}", i);
        }
    }

    #[test]
    fn test_generate_rook_attacks() {
        let generated = super::generate_rook_attacks();
        for (i, &attack) in ROOK_ATTACKS.iter().enumerate() {
            assert_eq!(attack, generated[i], "Mismatch at index {}", i);
        }
    }

    #[test]
    fn test_knight_attacks() {
        for sq in 0..NumberOf::SQUARES as u8 {
            let attacks = attacks::knight(sq);
            println!("Bitboard::new({:#x}),", attacks.as_number());
            assert_eq!(attacks, Bitboard::new(EXPECTED_KNIGHT_ATTACKS[sq as usize]))
        }
    }

    #[test]
    fn test_king_attacks() {
        for sq in 0..NumberOf::SQUARES as u8 {
            let attacks = attacks::king(sq);
            // println!("{}", attacks);
            println!("Bitboard::new({:#x}),", attacks.as_number());
        }
    }

    #[test]
    fn test_queen_attacks() {
        for sq in 0..NumberOf::SQUARES as u8 {
            let attacks = attacks::queen(sq, Bitboard::default());
            println!("{}", attacks);
            assert_eq!(
                attacks,
                Bitboard::new(EXPECTED_DIAGONAL_ATTACKS[sq as usize])
                    | Bitboard::new(EXPECTED_ORTHOGONAL_ATTACKS[sq as usize])
            )
        }
    }

    #[test]
    fn test_diagonal_ray_attacks() {
        for (sq, &expected) in EXPECTED_DIAGONAL_ATTACKS.iter().enumerate() {
            let attacks: crate::bitboard::Bitboard = super::diagonal_ray_attacks(sq as u8, 0);
            println!("Square: {}\nAttacks:\n{}", sq, attacks);
            assert_eq!(attacks.as_number(), expected);
        }
    }

    #[test]
    fn test_orthogonal_ray_attacks() {
        for sq in 0..64_u8 {
            let attacks: crate::bitboard::Bitboard = super::orthogonal_ray_attacks(sq, 0);
            println!("Square: {}\nAttacks:\n{}", sq, attacks);
            assert_eq!(
                attacks.as_number(),
                EXPECTED_ORTHOGONAL_ATTACKS[sq as usize]
            )
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

    #[test]
    fn test_pawn_attacks() {
        for sq in 0..NumberOf::SQUARES as u8 {
            let white_attacks = attacks::pawn(sq, crate::side::Side::White);
            let black_attacks = attacks::pawn(sq, crate::side::Side::Black);
            println!(
                "Square: {}\nWhite Pawn Attacks:\n{}\nBlack Pawn Attacks:\n{}",
                sq, white_attacks, black_attacks
            );
            assert_ne!(white_attacks, black_attacks);
        }
    }
}
