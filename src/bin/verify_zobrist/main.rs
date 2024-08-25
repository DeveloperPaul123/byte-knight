use std::path::PathBuf;

use byte_board::{board::Board, fen};
mod utils;

fn main() {
    // load data
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("data/lichess_db_puzzle.csv");
    if !data_path.exists() {
        print!("Data file not found, extracting from zip file...");
        // we need to unzip the data
        let mut zip_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        zip_path.push("data/lichess_db_puzzle.csv.zst");
        let file = std::fs::File::open(zip_path).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        let mut data_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_folder.push("data");
        let result = archive.extract(data_folder);
        if let Err(e) = result {
            panic!("Failed to extract zip file: {:?}", e);
        }

        print!("Extracted data file.")
    }

    assert!(data_path.exists());
    print!("Reading test data...");
    let records_result = utils::read_lichess_puzzles(data_path);

    // Compare two FEN strings for equality only using the first four parts
    let fen_match = |fen_left: &String, fen_right: &String| -> bool {
        let fen_left_result = fen::split_fen_string(fen_left);
        let fen_right_result = fen::split_fen_string(fen_right);
        if fen_left_result.is_err() || fen_right_result.is_err() {
            return false;
        }

        let fen_left_parts = fen_left_result.unwrap();
        let fen_right_parts = fen_right_result.unwrap();

        if fen_left_parts.len() != fen_right_parts.len() {
            return false;
        }

        for part in 0..4 {
            if fen_left_parts[part] != fen_right_parts[part] {
                return false;
            }
        }

        return true;
    };

    match records_result {
        Ok(records) => {
            println!("Read {} records", records.len());
            let mut hashes = Vec::with_capacity(records.len());
            for record in records {
                // the list does not contain unique FENs, so we need to check for duplicates
                if hashes
                    .iter()
                    .find(|(_, f)| fen_match(f, &record.fen))
                    .is_some()
                {
                    println!("Skipping duplicate FEN: {}", record.fen);
                    continue;
                }

                let board = Board::from_fen(&record.fen);
                assert!(board.is_ok());
                let board = board.unwrap();
                let hash = board.zobrist_hash();
                let found_hash = hashes.iter().find(|(h, _)| h == &hash);
                if let Some(loc) = found_hash {
                    println!("Duplicate hash: {}, {}", hash, record.fen);
                    println!("Duplicate of: {}, {}", loc.0, loc.1);
                    assert!(false);
                } else {
                    hashes.push((board.zobrist_hash(), record.fen));
                }
            }
        }
        Err(e) => {
            println!("Failed to read records: {:?}", e);
            assert!(false);
        }
    }
}
