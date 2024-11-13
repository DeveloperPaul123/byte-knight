use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct PgnGame {
    pub tags: HashMap<String, String>,
    pub moves: Vec<PgnMove>,
    pub result: GameResult,
}

#[derive(Debug, Clone)]
pub struct PgnMove {
    pub number: u32,
    pub white: String,
    pub black: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameResult {
    WhiteWin,
    BlackWin,
    Draw,
    Unknown,
}

impl FromStr for GameResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1-0" => Ok(GameResult::WhiteWin),
            "0-1" => Ok(GameResult::BlackWin),
            "1/2-1/2" => Ok(GameResult::Draw),
            "*" => Ok(GameResult::Unknown),
            _ => Err(format!("Invalid game result: {}", s)),
        }
    }
}

impl PgnGame {
    pub fn new() -> Self {
        PgnGame {
            tags: HashMap::new(),
            moves: Vec::new(),
            result: GameResult::Unknown,
        }
    }
}
