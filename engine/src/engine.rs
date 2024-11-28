/*
 * engine.rs
 * Part of the byte-knight project
 * Created Date: Friday, November 15th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Thu Nov 28 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

use chess::board::Board;
use uci_parser::{UciCommand, UciInfo, UciOption, UciResponse};

use crate::{
    defs::About, input_handler::InputHandler, search::SearchParameters,
    search_thread::SearchThread, tt_table::TranspositionTable,
};

pub struct ByteKnight {
    input_handler: InputHandler,
    search_thread: SearchThread,
}

impl ByteKnight {
    pub fn new() -> ByteKnight {
        ByteKnight {
            input_handler: InputHandler::new(),
            search_thread: SearchThread::new(),
        }
    }

    /// Run the engine loop. This will block until the engine is told to quit by the input handler.
    pub fn run(&mut self) -> anyhow::Result<()> {
        println!("{}", About::BANNER);
        println!(
            "{} {} by {} <{}>",
            About::NAME,
            About::VERSION,
            About::AUTHORS,
            About::EMAIL
        );
        let stdout: io::Stdout = io::stdout();
        let mut board = Board::default_board();
        let tt: Arc<Mutex<TranspositionTable>> = Arc::default();
        'engine_loop: while let Ok(command) = &self.input_handler.receiver().recv() {
            let mut stdout = stdout.lock();
            match command {
                UciCommand::Quit => {
                    // clean up
                    self.search_thread.exit();
                    self.input_handler.exit();
                    break 'engine_loop;
                }
                UciCommand::IsReady => {
                    writeln!(stdout, "{}", UciResponse::<String>::ReadyOk).unwrap();
                }
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
                UciCommand::UciNewGame => {
                    board = Board::default_board();
                    if let Ok(tt) = tt.lock().as_mut() {
                        tt.clear();
                    }
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
                        board.make_uci_move(&mv.to_string()).unwrap();
                    }
                }
                UciCommand::Go(search_options) => {
                    if self.search_thread.is_searching() {
                        eprintln!("Attempting to start a search while already searching");
                        self.search_thread.stop_search();
                    }

                    let info = UciInfo::default().string(format!("searching {}", board.to_fen()));
                    writeln!(stdout, "{}", UciResponse::info(info)).unwrap();

                    // create the search parameters
                    let search_params = SearchParameters::new(search_options, &board);
                    // send them and the current board to the search thread
                    self.search_thread
                        .start_search(&board, search_params, tt.clone());
                }
                UciCommand::Stop => {
                    self.search_thread.stop_search();
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl Default for ByteKnight {
    fn default() -> Self {
        ByteKnight::new()
    }
}
