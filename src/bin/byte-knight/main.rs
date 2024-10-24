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

mod base_engine;
mod engine;
mod evaluation;
mod evil_bot;
mod score;
mod search;
mod timer;

pub use base_engine::ChessEngine;
use engine::ByteKnight;
pub use evil_bot::EvilBot;
pub use timer::Timer;
use uci_parser::{UciCommand, UciInfo, UciMove, UciResponse, UciScore};

use std::{process::exit, slice::Iter, str::FromStr};

use byte_board::{
    board::Board, definitions::About, move_generation::MoveGenerator, moves::Move,
    pieces::SQUARE_NAME, side::Side,
};
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
    // prints out what engines are available
    Engines,
}

fn engine_for_type(engine_type: EngineType) -> Box<dyn ChessEngine> {
    match engine_type {
        EngineType::EvilBot => Box::new(EvilBot::default()),
        EngineType::ByteKnight => Box::new(ByteKnight::default()),
    }
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

fn run_uci(engine_name: &String) {
    let stdin: io::Stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut input = stdin.lock().lines();
    let mut board = Board::default_board();

    let engine_type = EngineType::from_str(&engine_name).expect("Invalid engine name");
    let mut engine = engine_for_type(engine_type);
    let move_gen = MoveGenerator::new();
    loop {
        if let Some(Ok(line)) = input.next() {
            let command = UciCommand::from_str(line.as_str()).unwrap();
            match command {
                UciCommand::Uci => {
                    let id = UciResponse::Id {
                        name: About::NAME,
                        author: About::AUTHORS,
                    };
                    writeln!(stdout, "{}", id).unwrap();
                    writeln!(stdout, "{}", UciResponse::<String>::UciOk).unwrap();
                }
                UciCommand::Quit => {
                    exit(0);
                }
                UciCommand::UciNewGame => {
                    board = Board::default_board();
                    engine = engine_for_type(engine_type);
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
                    let timer = match board.side_to_move() {
                        Side::Black => {
                            assert!(
                                search_options.btime.is_some(),
                                "btime is required for black side move"
                            );
                            Timer::new(search_options.btime.unwrap().as_millis() as i64)
                        }
                        Side::White => {
                            assert!(
                                search_options.wtime.is_some(),
                                "wtime is required for white side move"
                            );
                            Timer::new(search_options.wtime.unwrap().as_millis() as i64)
                        }
                        _ => {
                            panic!("Invalid side to move");
                        }
                    };

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

                    let best_move = engine.think(&mut board, &timer);

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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum EngineType {
    EvilBot,
    ByteKnight,
}

impl EngineType {
    fn iter() -> Iter<'static, EngineType> {
        static ENGINES: [EngineType; 2] = [EngineType::EvilBot, EngineType::ByteKnight];
        return ENGINES.iter();
    }
}

impl FromStr for EngineType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EvilBot" => Ok(EngineType::EvilBot),
            "ByteKnight" => Ok(EngineType::ByteKnight),
            _ => Err("Invalid engine".to_string()),
        }
    }
}

impl ToString for EngineType {
    fn to_string(&self) -> String {
        match self {
            EngineType::EvilBot => "EvilBot".to_string(),
            EngineType::ByteKnight => "ByteKnight".to_string(),
        }
    }
}

fn main() {
    let args = Options::parse();
    match args.command {
        Some(command) => match command {
            Command::Engines => {
                println!("Available engines:");
                for engine in EngineType::iter() {
                    println!("  🤖 {}", engine.to_string());
                }
                exit(0);
            }
        },
        None => run_uci(&args.engine),
    }
}
