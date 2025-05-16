use anyhow::Result;
use arrayvec::ArrayVec;
use chess::{definitions::MAX_MOVES, moves::Move};

/// Represents the [Principle Variation](https://www.chessprogramming.org/Principal_Variation) during a search.
#[derive(Debug)]
pub(crate) struct PrincipleVariation {
    data: ArrayVec<Move, MAX_MOVES>,
}

impl Default for PrincipleVariation {
    fn default() -> Self {
        Self::new()
    }
}

impl PrincipleVariation {
    /// Create a new, empty ['PrincipleVariation'].
    pub(crate) fn new() -> Self {
        Self {
            data: ArrayVec::new(),
        }
    }

    /// Push a move to the ['PrincipleVariation'].
    ///
    /// # Arguments:
    ///
    /// - m: The new move to add to the principle variation.
    ///
    /// # Returns:
    ///
    /// Empty Result<> on success or an error if the underlying ArrayVec
    /// is full before trying to push.
    pub(crate) fn push(&mut self, m: Move) -> Result<()> {
        let push_result = self.data.try_push(m);
        Ok(push_result?)
    }

    /// Extend the current [PrincipleVariation] with the given move and another principle variation.
    ///
    /// This will clear the current PV and append the given move and [PrincipleVariation].
    ///
    /// # Arguments:
    ///
    /// - m: The new move to add to the principle variation.
    /// - pv: The principle variation to append to the current variation.
    #[inline(always)]
    pub(crate) fn extend(&mut self, m: Move, pv: &Self) -> Result<()> {
        self.clear();
        self.push(m)?;
        Ok(self.data.try_extend_from_slice(pv.data.as_slice())?)
    }

    /// Clear the principle variation.
    pub(crate) fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns an iterator to the underlying data of the [PrincipleVariation].
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Move> {
        self.data.iter()
    }
}

#[cfg(test)]
mod tests {
    use chess::{definitions::Squares, moves::MoveDescriptor, pieces::Piece, square::Square};

    use super::*;

    fn make_moves() -> (Move, Move) {
        let move1 = Move::new(
            &Square::from_square_index(Squares::E2),
            &Square::from_square_index(Squares::E4),
            MoveDescriptor::PawnTwoUp,
            Piece::Pawn,
            None,
            None,
        );

        let move2 = Move::new(
            &Square::from_square_index(Squares::E7),
            &Square::from_square_index(Squares::E5),
            MoveDescriptor::PawnTwoUp,
            Piece::Pawn,
            None,
            None,
        );

        (move1, move2)
    }

    #[test]
    fn add_moves_to_principle_variation() {
        let mut pv = PrincipleVariation::new();

        let (move1, move2) = make_moves();

        // Push moves to the principle variation
        assert!(pv.push(move1).is_ok());
        assert!(pv.push(move2).is_ok());

        // Check that the moves are in the principle variation
        assert_eq!(pv.data.len(), 2);
    }

    #[test]
    fn extend_principle_variation() {
        let mut pv = PrincipleVariation::new();

        let (move1, move2) = make_moves();

        // Push the first move to the principle variation
        assert!(pv.push(move1).is_ok());

        // Create a new principle variation to extend from
        let mut pv2 = PrincipleVariation::new();
        assert!(pv2.push(move2).is_ok());

        let pv_len_before = pv.data.len();
        // Extend the original principle variation with the new one
        assert!(pv.extend(move1, &pv2).is_ok());

        // Check that the moves are in the principle variation
        assert_eq!(pv.data.len(), pv_len_before + 1 + pv2.data.len());
    }

    #[test]
    fn extending_or_pushing_move_past_max_size_fails() {
        let (move1, move2) = make_moves();
        let mut pv = PrincipleVariation::new();

        // Fill the principle variation to its maximum size
        for _ in 0..MAX_MOVES {
            assert!(pv.push(move1).is_ok());
        }

        // Attempt to push another move, which should fail
        assert!(pv.push(move2).is_err());

        // reset
        pv = PrincipleVariation::new();
        let mut pv2 = PrincipleVariation::new();

        // Fill the principle variation to its maximum size
        for _ in 0..MAX_MOVES - 10 {
            assert!(pv.push(move1).is_ok());
        }

        for _ in 0..10 {
            assert!(pv2.push(move2).is_ok());
        }

        // Attempt to extend the principle variation with another one that would exceed the max size
        assert!(pv.extend(move2, &pv2).is_err());
    }
}
