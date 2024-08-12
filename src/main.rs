mod engine;
use std::{fmt::{Display, Formatter}, slice::Iter, str::FromStr};

use crate::engine::EvilBot;
use chess::{Board, MoveGen};

use clap::{Parser, Subcommand};
use engine::ChessEngine;
use std::io::{self, BufRead, Write};
use vampirc_uci::{parse, UciMessage, UciTimeControl};

#[derive(Parser)]
#[command(
    version = "0.0.1", about = "ByteKnight is a UCI compliant chess engine", long_about = None
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
    Engines
}

fn engine_for_type(engine_type: EngineType) -> Box<dyn ChessEngine> {
    match engine_type {
        EngineType::EvilBot => Box::new(EvilBot{}),
        EngineType::ByteKnight => unimplemented!()
    }
}

fn run_uci(engine_name : &String) {
    let stdin: io::Stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut input = stdin.lock().lines();

    let mut board = Board::default();

    // already in UCI mode, so output the engine info
    writeln!(stdout, "{}", UciMessage::id_name("byte-knight")).unwrap();
    writeln!(stdout, "{}", UciMessage::id_author("Paul T (DeveloperPaul123)")).unwrap();
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
                        search_control,
                    } => {
                        // TODO: Implement time control

                        let best_move = engine.think(&board);
                        if let Some(bot_move) = best_move {
                            writeln!(stdout, "{}", UciMessage::best_move(bot_move).to_string())
                                .unwrap();
                        } else {
                            // Handle the case when best_move is None.
                            // Use the first legal move as a fallback
                            MoveGen::new_legal(&board).next().map(|m| {
                                writeln!(stdout, "{}", UciMessage::best_move(m).to_string())
                                    .unwrap()
                            });
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
    ByteKnight
}

impl EngineType {
    fn iter() -> Iter<'static, EngineType> {
        static ENGINES: [EngineType; 2] = [EngineType::EvilBot, EngineType::ByteKnight];
        return ENGINES.iter()
    }
}

impl FromStr for EngineType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EvilBot" => Ok(EngineType::EvilBot),
            "ByteKnight" => Ok(EngineType::ByteKnight),
            _ => Err("Invalid engine".to_string())
        }
    }
}

impl ToString for EngineType {
    fn to_string(&self) -> String {
        return match self {
            EngineType::EvilBot => "EvilBot".to_string(),
            EngineType::ByteKnight => "ByteKnight".to_string()
        }
    }
}

fn main() {
    let args = Options::parse();
    match args.command {
        Command::Uci { engine } => {
            run_uci(&engine);
        },
        Command::Engines {
        } => {
            println!("Available engines:");
            for engine in EngineType::iter() {
                println!("  🤖 {}", engine.to_string());
            }
        }
    }
}
