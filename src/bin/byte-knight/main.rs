/*
 * main.rs
 * Part of the byte-knight project
 * Created Date: Wednesday, August 14th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified:
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

mod bench;
mod defs;
mod engine;
mod evaluation;
mod psqt;
mod score;
mod search;
mod tt_table;

use defs::About;
use engine::ByteKnight;
use search::SearchParameters;
use uci_parser::{UciCommand, UciInfo, UciMove, UciOption, UciResponse};

use std::{process::exit, str::FromStr};

use chess::{board::Board, moves::Move, pieces::SQUARE_NAME};
use clap::{Parser, Subcommand};
use std::io::{self, BufRead, Write};

#[derive(Parser)]
#[command(
    version = About::VERSION, about = About::SHORT_DESCRIPTION, long_about = About::SHORT_DESCRIPTION
)]
struct Options {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
#[command(about = "Available commands")]
enum Command {
    #[command(about = "Run fixed depth search")]
    Bench {
        #[arg(short, long, default_value = "6")]
        depth: u8,

        #[arg(short, long)]
        epd_file: Option<String>,
    },
}

fn square_index_to_uci_square(square: u8) -> uci_parser::Square {
    uci_parser::Square::from_str(SQUARE_NAME[square as usize]).unwrap()
}

fn move_to_uci_move(mv: &Move) -> UciMove {
    let promotion = mv.promotion_piece().map(|p| p.as_char());

    match promotion {
        Some(promotion) => UciMove {
            src: square_index_to_uci_square(mv.from()),
            dst: square_index_to_uci_square(mv.to()),
            promote: Some(uci_parser::Piece::from_str(&promotion.to_string()).unwrap()),
        },
        None => UciMove {
            src: square_index_to_uci_square(mv.from()),
            dst: square_index_to_uci_square(mv.to()),
            promote: None,
        },
    }
}

fn run_uci() {
    let stdin: io::Stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut input = stdin.lock().lines();
    let mut board = Board::default_board();

    let mut engine = ByteKnight::new();
    writeln!(stdout, "{}", About::BANNER).unwrap();

    loop {
        if let Some(Ok(line)) = input.next() {
            let command = UciCommand::from_str(line.as_str());
            match command {
                Ok(UciCommand::Uci) => {
                    let id = UciResponse::Id {
                        name: About::NAME,
                        author: About::AUTHORS,
                    };

                    let options = vec![
                        UciOption::spin("Hash", 16, 1, 1024),
                        UciOption::spin("Threads", 1, 1, 1),
                    ];
                    // TODO: Actually implement the hash option
                    for option in options {
                        writeln!(stdout, "{}", UciResponse::Option(option)).unwrap();
                    }
                    writeln!(stdout, "{}", id).unwrap();
                    writeln!(stdout, "{}", UciResponse::<String>::UciOk).unwrap();
                }
                Ok(UciCommand::Quit) => {
                    exit(0);
                }
                Ok(UciCommand::UciNewGame) => {
                    board = Board::default_board();
                }
                Ok(UciCommand::IsReady) => {
                    writeln!(stdout, "{}", UciResponse::<String>::ReadyOk).unwrap();
                }
                Ok(UciCommand::Position { fen, moves }) => {
                    match fen {
                        None => {
                            board = Board::default_board();
                        }
                        Some(fen) => {
                            board = Board::from_fen(fen.as_str()).unwrap();
                        }
                    }

                    for mv in moves {
                        board.make_uci_move(&mv.to_string()).unwrap();
                    }

                    // TODO: String output of board
                    // writeln!(stdout, "{}", Board::to_string(&board)).unwrap();
                }
                Ok(UciCommand::Go(search_options)) => {
                    let info = UciInfo::default().string(format!("searching {}", board.to_fen()));
                    writeln!(stdout, "{}", UciResponse::info(info)).unwrap();
                    let search_params = SearchParameters::new(&search_options, &board);
                    let best_move = engine.think(&mut board, &search_params);
                    let move_output = UciResponse::BestMove {
                        bestmove: best_move.map_or(None, |bot_move| {
                            Some(move_to_uci_move(&bot_move).to_string())
                        }),
                        ponder: None,
                    };
                    writeln!(
                        stdout,
                        "{}",
                        // TODO: Ponder
                        move_output
                    )
                    .unwrap();
                }
                _ => (),
            }

            stdout.flush().unwrap();
        }
    }
}

fn main() {
    let args = Options::parse();
    match args.command {
        Some(command) => match command {
            Command::Bench { depth, epd_file } => {
                bench::bench(depth, &epd_file);
            }
        },
        None => run_uci(),
    }
}
