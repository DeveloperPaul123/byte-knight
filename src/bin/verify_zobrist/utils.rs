use std::{error::Error, path::PathBuf};

#[derive(Debug, serde::Deserialize)]
pub struct LichessPuzzleRecord {
    #[serde(rename = "FEN")]
    pub(crate) fen: String,
}

pub fn read_lichess_puzzles(path_buf: PathBuf) -> Result<Vec<LichessPuzzleRecord>, Box<dyn Error>> {
    let reader = csv::Reader::from_path(path_buf);
    let records = reader?
        .deserialize()
        .collect::<Result<Vec<LichessPuzzleRecord>, _>>()?;

    Ok(records)
}
