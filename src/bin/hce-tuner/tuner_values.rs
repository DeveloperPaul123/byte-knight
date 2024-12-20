use chess::{pieces::Piece, side::Side, square};
use engine::{phased_score::PhasedScore, score::ScoreType, traits::EvalValues};

use crate::{offsets::Offsets, tuner::TableType};

#[derive(Debug, Clone)]
pub(crate) struct TunerValues {
    offsets: Offsets,
    params: Vec<ScoreType>,
    temporary_param: Option<(usize, ScoreType)>,
}

impl<'a> TunerValues {
    pub(crate) fn new(offsets: Offsets, params: Vec<ScoreType>) -> Self {
        Self {
            offsets,
            params,
            temporary_param: None,
        }
    }

    pub(crate) fn params(&self) -> &Vec<ScoreType> {
        &self.params
    }

    pub(crate) fn increment_param(&mut self, index: usize, value: ScoreType) {
        let new_score = self.params[index] + value;
        self.temporary_param = Some((index, new_score));
    }

    pub(crate) fn decrement_param(&mut self, index: usize, value: ScoreType) {
        let new_score = self.params[index] - value;
        self.temporary_param = Some((index, new_score));
    }

    pub(crate) fn commit(&mut self) {
        if let Some((index, new_score)) = self.temporary_param {
            self.params[index] = new_score;
            self.temporary_param = None;
        }
    }

    pub(crate) fn discard(&mut self) {
        self.temporary_param = None;
    }
}

impl<'a> EvalValues for TunerValues {
    type ReturnScore = PhasedScore;

    fn psqt(&self, square: u8, piece: Piece, side: chess::side::Side) -> Self::ReturnScore {
        let mg_start_index = self
            .offsets
            .start_index_for_piece(piece, TableType::Midgame)
            .expect("Could not get MG index");
        let eg_start_index = self
            .offsets
            .start_index_for_piece(piece, TableType::Endgame)
            .expect("Could not get EG index");

        // check if we have a temporary param other wise build the index
        let mg_index = self.temporary_param.map_or(
            mg_start_index + square::flip_if(side == Side::White, square) as usize,
            |(index, _)| index,
        );

        // do the same for the endgame value
        let eg_index = self.temporary_param.map_or(
            eg_start_index + square::flip_if(side == Side::White, square) as usize,
            |(index, _)| index,
        );

        let mg = self.params[mg_index];
        let eg = self.params[eg_index];

        // construct the score and return it
        PhasedScore::new(mg, eg)
    }
}
