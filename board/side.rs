#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    White = 0,
    Black = 1,
    Both = 2,
}

impl Side {}

impl Side {
    pub fn opposite(side: Side) -> Side {
        match side {
            Side::White => Side::Black,
            Side::Black => Side::White,
            _ => Side::Both,
        }
    }

    /// Returns `true` if the side is [`WHITE`].
    ///
    /// [`WHITE`]: Side::WHITE
    #[must_use]
    pub fn is_white(&self) -> bool {
        matches!(self, Self::White)
    }

    /// Returns `true` if the side is [`BLACK`].
    ///
    /// [`BLACK`]: Side::BLACK
    #[must_use]
    pub fn is_black(&self) -> bool {
        matches!(self, Self::Black)
    }

    /// Returns `true` if the side is [`BOTH`].
    ///
    /// [`BOTH`]: Side::BOTH
    #[must_use]
    pub fn is_both(&self) -> bool {
        matches!(self, Self::Both)
    }
}

impl Default for Side {
    fn default() -> Self {
        Self::White
    }
}

impl TryFrom<u8> for Side {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::White),
            1 => Ok(Self::Black),
            2 => Ok(Self::Both),
            _ => Err(()),
        }
    }
}
