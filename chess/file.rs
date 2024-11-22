use anyhow::Result;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl File {
    pub fn offset(&self, delta: i8) -> Option<Self> {
        let new_file = (*self as i8) + delta;
        if (0..=7).contains(&new_file) {
            return File::try_from(new_file as u8).ok();
        }
        None
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::A => 'a',
            Self::B => 'b',
            Self::C => 'c',
            Self::D => 'd',
            Self::E => 'e',
            Self::F => 'f',
            Self::G => 'g',
            Self::H => 'h',
        }
    }
}

impl TryFrom<u8> for File {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            3 => Ok(Self::D),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            6 => Ok(Self::G),
            7 => Ok(Self::H),
            _ => Err(anyhow::Error::msg(format!("Invalid file {}", value))),
        }
    }
}

impl TryFrom<char> for File {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self> {
        match value {
            'a' => Ok(Self::A),
            'b' => Ok(Self::B),
            'c' => Ok(Self::C),
            'd' => Ok(Self::D),
            'e' => Ok(Self::E),
            'f' => Ok(Self::F),
            'g' => Ok(Self::G),
            'h' => Ok(Self::H),
            _ => Err(anyhow::Error::msg(format!("Invalid file {}", value))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset() {
        assert_eq!(File::A.offset(1), Some(File::B));
        assert_eq!(File::A.offset(-1), None);
        assert_eq!(File::H.offset(1), None);
        assert_eq!(File::H.offset(-1), Some(File::G));
    }
}
