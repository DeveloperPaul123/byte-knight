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
mod score;
mod search;
mod tt_table;

use defs::About;
use engine::ByteKnight;
use search::SearchParameters;
use uci_parser::{UciCommand, UciInfo, UciMove, UciOption, UciResponse, UciScore};

use std::{process::exit, slice::Iter, str::FromStr};

use byte_board::{board::Board, move_generation::MoveGenerator, moves::Move, pieces::SQUARE_NAME};
use clap::{Parser, Subcommand};
use std::io::{self, BufRead, Write};

#[derive(Parser)]
#[command(
    version = About::VERSION, about = About::SHORT_DESCRIPTION, long_about = About::SHORT_DESCRIPTION
)]
struct Options {
    #[command(subcommand)]
    command: Option<Command>,

    #[arg(long, short, default_value = "ByteKnight")]
    engine: String,
}

#[derive(Subcommand)]
#[command(about = "Available commands")]
enum Command {
    #[command(about = "Run fixed depth search")]
    Bench {
        #[arg(short, long, default_value = "6")]
        depth: u8,
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
    let move_gen = MoveGenerator::new();

    writeln!(stdout, "{}", About::BANNER).unwrap();

    loop {
        if let Some(Ok(line)) = input.next() {
            let command = UciCommand::from_str(line.as_str()).unwrap();
            match command {
                UciCommand::Uci => {
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
                UciCommand::Quit => {
                    exit(0);
                }
                UciCommand::UciNewGame => {
                    board = Board::default_board();
                }
                UciCommand::IsReady => {
                    writeln!(stdout, "{}", UciResponse::<String>::ReadyOk).unwrap();
                }
                UciCommand::Position { fen, moves } => {
                    match fen {
                        None => {
                            board = Board::default_board();
                        }
                        Some(fen) => {
                            board = Board::from_fen(fen.as_str()).unwrap();
                        }
                    }

                    for mv in moves {
                        board.make_uci_move(&mv.to_string(), &move_gen).unwrap();
                    }

                    // TODO: String output of board
                    // writeln!(stdout, "{}", Board::to_string(&board)).unwrap();
                }
                UciCommand::Go(search_options) => {
                    let search_params = SearchParameters::new(&search_options, &board);

                    let info = UciInfo::default()
                        .score(UciScore::cp(20))
                        .depth(1)
                        .seldepth(1)
                        .time(1)
                        .nodes(1)
                        .nps(0);
                    writeln!(stdout, "{}", UciResponse::<String>::Info(Box::new(info))).unwrap();

                    let board_info =
                        UciInfo::default().string(format!("searching {}", board.to_fen()));
                    writeln!(
                        stdout,
                        "{}",
                        UciResponse::<String>::Info(Box::new(board_info))
                    )
                    .unwrap();

                    let best_move = engine.think(&mut board, &search_params);

                    if let Some(bot_move) = best_move {
                        writeln!(
                            stdout,
                            "{}",
                            // TODO: Ponder
                            UciResponse::BestMove {
                                bestmove: Some(move_to_uci_move(&bot_move).to_string()),
                                ponder: None
                            }
                        )
                        .unwrap();
                    } else {
                        // TODO Handle the case when best_move is None.
                    }
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
            Command::Bench { depth } => {
                bench::bench(depth);
            }
        },
        None => run_uci(),
    }
}
