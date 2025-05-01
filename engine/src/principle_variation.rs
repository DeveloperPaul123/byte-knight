use anyhow::Result;
use arrayvec::ArrayVec;
use chess::{definitions::MAX_MOVES, moves::Move};

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
    pub(crate) fn new() -> Self {
        Self {
            data: ArrayVec::new(),
        }
    }

    pub(crate) fn push(&mut self, m: Move) -> Result<()> {
        let push_result = self.data.try_push(m);
        Ok(push_result?)
    }

    pub(crate) fn extend(&mut self, m: Move, pv: &Self) -> Result<()> {
        self.push(m)?;
        Ok(self.data.try_extend_from_slice(pv.data.as_slice())?)
    }

    pub(crate) fn clear(&mut self) {
        self.data.clear();
    }
}
