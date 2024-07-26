mod engine;
use std::str::FromStr;

use crate::engine::EvilBot;
use chess::{Board, MoveGen};

use clap::{Parser, Subcommand};
use engine::ChessEngine;
use std::io::{self, BufRead, Write};
use vampirc_uci::{parse, UciMessage, UciTimeControl};

#[derive(Parser)]
#[command(version="0.0.1", about="ByteKnight is a UCI compliant chess engine", long_about=None)]
struct Options {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[command(about = "Available commands")]
enum Command {
    Uci {
        #[arg(long)]
        engine_name: String,
    },
}

fn main() {
    let args = Options::parse();

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut input = stdin.lock().lines();
    let evil_bot = EvilBot {};
    let mut board = Board::default();
    loop {
        if let Some(Ok(line)) = input.next() {
            let message = parse(&line);
            for msg in message.iter() {
                println!("{:?}", msg);
                match msg {
                    UciMessage::Uci => {
                        writeln!(stdout, "{}", UciMessage::id_name("byte-knight")).unwrap();
                        writeln!(
                            stdout,
                            "{}",
                            UciMessage::id_author("Paul T (DeveloperPaul123")
                        )
                        .unwrap();
                        writeln!(stdout, "{}", UciMessage::UciOk).unwrap();
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

                        println!("Position: {:?}", fen);
                        println!("Moves: {:?}", moves);
                        println!("Startpos: {:?}", startpos);
                    }
                    UciMessage::Go {
                        time_control,
                        search_control,
                    } => {
                        println!("{}", board.to_string());

                        // TODO: Implement time control

                        let best_move = evil_bot.think(&board);
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
