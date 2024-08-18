pub type Square = u8;

pub fn to_square(file: u8, rank: u8) -> Square {
    rank * 8 + file
}
