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
mod chess_board_helpers;
mod engine;
mod evaluation;
mod evil_bot;
mod search;
mod timer;

pub use base_engine::ChessEngine;
pub use evil_bot::EvilBot;
pub use timer::Timer;

use std::{slice::Iter, str::FromStr};

use chess::{Board, MoveGen};

use byte_board::definitions::About;
use clap::{Parser, Subcommand};
use std::io::{self, BufRead, Write};
use vampirc_uci::{parse, UciMessage, UciTimeControl};

#[derive(Parser)]
#[command(
    version = About::VERSION, about = About::SHORT_DESCRIPTION, long_about = About::SHORT_DESCRIPTION
)]
struct Options {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[command(about = "Available commands")]
enum Command {
    Uci {
        #[arg(long, short)]
        engine: String,
    },
    // prints out what engines are available
    Engines,
}

fn engine_for_type(engine_type: EngineType) -> Box<dyn ChessEngine> {
    match engine_type {
        EngineType::EvilBot => Box::new(EvilBot {}),
        EngineType::ByteKnight => unimplemented!(),
    }
}

fn run_uci(engine_name: &String) {
    let stdin: io::Stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut input = stdin.lock().lines();

    let mut board = Board::default();

    // already in UCI mode, so output the engine info
    writeln!(stdout, "{}", UciMessage::id_name(About::NAME)).unwrap();
    writeln!(stdout, "{}", UciMessage::id_author(About::AUTHORS)).unwrap();

    writeln!(stdout, "{}", UciMessage::UciOk).unwrap();
    let engine_type = EngineType::from_str(&engine_name).expect("Invalid engine name");
    let mut engine = engine_for_type(engine_type);

    loop {
        if let Some(Ok(line)) = input.next() {
            let message = parse(&line);
            for msg in message.iter() {
                match msg {
                    UciMessage::UciNewGame => {
                        board = Board::default();
                        engine = engine_for_type(engine_type);
                    }
                    UciMessage::IsReady => {
                        writeln!(stdout, "{}", UciMessage::ReadyOk).unwrap();
                    }
                    UciMessage::Position {
                        startpos,
                        fen,
                        moves,
                    } => {
                        match startpos {
                            true => {
                                board = Board::default();
                            }
                            false => {
                                if let Some(fen) = fen {
                                    board = Board::from_str(fen.as_str()).unwrap();
                                }
                            }
                        }

                        for m in moves {
                            let mut new_board = board.clone();
                            board.make_move(*m, &mut new_board);
                            board = new_board;
                        }

                        writeln!(stdout, "{}", Board::to_string(&board)).unwrap();
                    }
                    UciMessage::Go {
                        time_control,
                        search_control: _,
                    } => {
                        // TODO: Implement time control
                        let tc: &UciTimeControl = time_control.as_ref().unwrap();
                        let timer;
                        match tc {
                            UciTimeControl::TimeLeft {
                                white_time,
                                black_time,
                                white_increment: _,
                                black_increment: _,
                                moves_to_go: _,
                            } => match board.side_to_move() {
                                chess::Color::Black => {
                                    timer = Timer::new(black_time.unwrap().num_milliseconds());
                                }
                                chess::Color::White => {
                                    timer = Timer::new(white_time.unwrap().num_milliseconds());
                                }
                            },
                            _ => {
                                // TODO: Log error
                                timer = Timer::new(0);
                            }
                        }

                        let best_move = engine.think(&board, &timer);
                        if let Some(bot_move) = best_move {
                            writeln!(stdout, "{}", UciMessage::best_move(bot_move).to_string())
                                .unwrap();
                        } else {
                            // Handle the case when best_move is None.
                            // Use the first legal move as a fallback
                            MoveGen::new_legal(&board)
                                .next()
                                .map(|m| writeln!(stdout, "{}", UciMessage::best_move(m)).unwrap());
                        }
                    }
                    UciMessage::Quit => {
                        break;
                    }
                    _ => (),
                }
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
        Command::Uci { engine } => {
            run_uci(&engine);
        }
        Command::Engines {} => {
            println!("Available engines:");
            for engine in EngineType::iter() {
                println!("  🤖 {}", engine.to_string());
            }
        }
    }
}
