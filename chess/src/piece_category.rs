/*
 * piece_category.rs
 * Part of the byte-knight project
 * Created Date: Thursday, April 24th 2025
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Thu Apr 24 2025
 * -----
 * Copyright (c) 2025 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use crate::{non_slider_piece::NonSliderPiece, pieces::Piece, slider_pieces::SliderPiece};

/// Categorical enum for [`Piece`] types
#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum PieceCategory {
    NonSlider(NonSliderPiece),
    Slider(SliderPiece),
}

impl PieceCategory {
    /// Returns the piece type of the piece category.
    pub fn piece_type(&self) -> u8 {
        match self {
            PieceCategory::NonSlider(piece) => piece.piece_type() as u8,
            PieceCategory::Slider(piece) => piece.piece_type() as u8,
        }
    }
}

impl From<Piece> for PieceCategory {
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::King | Piece::Knight | Piece::Pawn => {
                PieceCategory::NonSlider(NonSliderPiece::try_from(piece).unwrap())
            }
            Piece::Rook | Piece::Bishop | Piece::Queen => {
                PieceCategory::Slider(SliderPiece::try_from(piece).unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{non_slider_piece::NonSliderPiece, pieces::Piece, slider_pieces::SliderPiece};

    use super::PieceCategory;

    #[test]
    fn categorize_pieces() {
        let sliders = [Piece::Queen, Piece::Rook, Piece::Bishop];
        let non_sliders = [Piece::Pawn, Piece::King, Piece::Knight];

        for slider in sliders {
            let category = PieceCategory::from(slider);
            let slider_piece = SliderPiece::try_from(slider).unwrap();
            assert_eq!(category, PieceCategory::Slider(slider_piece));
        }

        for non_slider in non_sliders {
            let category = PieceCategory::from(non_slider);
            let non_slider_piece = NonSliderPiece::try_from(non_slider).unwrap();
            assert_eq!(category, PieceCategory::NonSlider(non_slider_piece));
        }
    }
}
