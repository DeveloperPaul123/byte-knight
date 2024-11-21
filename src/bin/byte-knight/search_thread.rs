/*
 * search_thread.rs
 * Part of the byte-knight project
 * Created Date: Thursday, November 21st 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Thu Nov 21 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::{
    io::Write,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender},
        Arc,
    },
    thread::JoinHandle,
};

use chess::{board::Board, moves::Move, pieces::SQUARE_NAME};
use uci_parser::{UciMove, UciResponse};

use crate::search::{Search, SearchParameters};

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

#[allow(clippy::large_enum_variant)]
pub(crate) enum SearchThreadValue {
    Params(Board, SearchParameters),
    Exit,
}

pub(crate) struct SearchThread {
    sender: Sender<SearchThreadValue>,
    handle: Option<JoinHandle<()>>,
    stop_search_flag: Arc<AtomicBool>,
}

impl SearchThread {
    pub(crate) fn new() -> SearchThread {
        let (sender, receiver) = mpsc::channel();
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_clone = stop_flag.clone();
        let handle = std::thread::spawn(move || {
            let mut stdout = std::io::stdout();
            'search_loop: loop {
                let value = receiver.recv().unwrap();
                match value {
                    SearchThreadValue::Params(mut board, params) => {
                        let flag = stop_flag.clone();
                        let result = Search::new(&params).search(&mut board, Some(flag));
                        let best_move = result.best_move;
                        let move_output = UciResponse::BestMove {
                            bestmove: best_move
                                .map(|bot_move| move_to_uci_move(&bot_move).to_string()),
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

                    SearchThreadValue::Exit => {
                        break 'search_loop;
                    }
                }
            }
        });
        SearchThread {
            sender,
            handle: Some(handle),
            stop_search_flag: stop_flag_clone,
        }
    }

    pub(crate) fn exit(&mut self) {
        self.stop_search();
        self.sender.send(SearchThreadValue::Exit).unwrap();
        self.handle.take().unwrap().join().unwrap();
    }
    pub(crate) fn stop_search(&self) {
        self.stop_search_flag.store(true, Ordering::Relaxed);
    }

    pub(crate) fn start_search(&self, board: &Board, params: SearchParameters) {
        self.stop_search_flag.store(false, Ordering::Relaxed);
        self.sender
            .send(SearchThreadValue::Params(board.clone(), params))
            .unwrap();
    }
}
