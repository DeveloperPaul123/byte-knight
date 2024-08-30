pub const fn to_square(file: u8, rank: u8) -> u8 {
    rank * 8 + file
}

pub const fn from_square(square: u8) -> (u8, u8) {
    let rank = square / 8;
    let file = square % 8;
    return (file, rank);
}
