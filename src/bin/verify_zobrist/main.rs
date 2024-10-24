use std::{collections::HashMap, path::PathBuf};

use byte_board::{board::Board, fen};
use console::Emoji;
use indicatif::ParallelProgressIterator;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
mod utils;

static CHECK_BOX: Emoji = Emoji("✅", "");
static CROSS_MARK: Emoji = Emoji("❌", "");

fn main() {
    // load data
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("data/lichess_db_puzzle.csv");
    if !data_path.exists() {
        println!("Data file not found, decompressing from .zst file...");
        let mut zst_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        zst_path.push("data/lichess_db_puzzle.csv.zst");
        let file = std::fs::File::open(&zst_path)
            .unwrap_or_else(|_| panic!("Failed to open .zst file at {:?}", zst_path));

        let mut decoder = zstd::Decoder::new(file)
            .unwrap_or_else(|_| panic!("Failed to create zstd decoder for {:?}", zst_path));

        let mut data_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_folder.push("data");
        let mut output_path = data_folder.clone();
        output_path.push("lichess_db_puzzle.csv");

        let mut output_file = std::fs::File::create(&output_path)
            .unwrap_or_else(|_| panic!("Failed to create output file at {:?}", output_path));

        std::io::copy(&mut decoder, &mut output_file)
            .unwrap_or_else(|_| panic!("Failed to decompress .zst file to {:?}", output_path));

        println!("Decompressed data file.");
    }

    assert!(data_path.exists());
    println!("Reading test data...");
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
            let mut hashes: Vec<(u64, String)> = Vec::with_capacity(records.len());
            println!("Read {} records", records.len());
            println!("Calculating hashes...");
            records
                .par_iter()
                .progress_count(records.len() as u64)
                .map(|record| {
                    let board = Board::from_fen(&record.fen);
                    assert!(board.is_ok());
                    let board = board.unwrap();
                    let hash = board.zobrist_hash();
                    return (hash, record.fen.clone());
                })
                .collect_into_vec(&mut hashes);

            // Compare the hashes
            println!("Comparing hashes...");
            let mut hash_map: HashMap<u64, Vec<String>> = std::collections::HashMap::new();

            for (hash, fen) in hashes {
                if hash_map.contains_key(&hash) {
                    let vec = hash_map.get_mut(&hash).unwrap();
                    vec.push(fen);
                } else {
                    hash_map.insert(hash, vec![fen]);
                }
            }

            let mut duplicates = 0;
            for (hash, fens) in hash_map {
                if fens.len() > 1 {
                    let mut matched = false;
                    for i in 0..fens.len() {
                        for j in i + 1..fens.len() {
                            if fen_match(&fens[i], &fens[j]) {
                                matched = true;
                                break;
                            }
                        }
                        if matched {
                            break;
                        }
                    }

                    if !matched {
                        println!("Hash collision detected: {}", hash);
                        for fen in fens {
                            println!("{}", fen);
                        }
                        duplicates += 1;
                    }
                }
            }

            if duplicates == 0 {
                println!("{} No hash collisions detected!", CHECK_BOX.to_string());
            } else {
                println!("{} {} hash collisions detected", CROSS_MARK, duplicates);
            }
        }
        Err(e) => {
            println!("Failed to read records: {:?}", e);
            assert!(false);
        }
    }
}
