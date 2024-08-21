use crate::pieces::Piece;

#[derive(Debug)]
struct Move {
    /// File, Rank
    from: (u8, u8),
    /// File, Rank
    to: (u8, u8),
    /// Optional promotion piece if the move is a promotion
    promotion: Option<Piece>,
}
