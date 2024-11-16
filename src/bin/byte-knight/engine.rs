use std::{
    io::{self, BufRead, Write},
    str::FromStr,
    sync::mpsc::{self, Receiver, Sender},
};

use chess::{board::Board, move_generation::MoveGenerator, moves::Move, pieces::SQUARE_NAME};
use uci_parser::{UciCommand, UciInfo, UciMove, UciOption, UciResponse};

use crate::{defs::About, search::SearchParameters};

use super::search;

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

pub struct ByteKnight {
    sender: Sender<UciCommand>,
    receiver: Receiver<UciCommand>,
    move_gen: MoveGenerator,
}

impl ByteKnight {
    pub fn new() -> ByteKnight {
        let (sender, receiver) = mpsc::channel();
        ByteKnight {
            sender,
            receiver,
            move_gen: MoveGenerator::new(),
        }
    }

    pub fn run(self: &mut Self, stdin: io::Stdin) -> anyhow::Result<()> {
        // spawn thread to handle UCI commands
        let sender = self.sender.clone();
        std::thread::spawn(move || {
            let mut input = stdin.lock().lines();
            loop {
                if let Some(Ok(line)) = input.next() {
                    let command = UciCommand::from_str(line.as_str());
                    if let Ok(command) = command {
                        sender.send(command).unwrap();
                    }
                }
            }
        });
        let stdout = io::stdout();
        let mut board = Board::default_board();
        while let Ok(command) = self.receiver.recv() {
            let mut stdout = stdout.lock();
            match command {
                UciCommand::Quit => {
                    !todo!("Implement a way to interupt the engine if we're searching");
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
                    let search_params = SearchParameters::new(&search_options, &board);

                    let board_info =
                        UciInfo::default().string(format!("searching {}", board.to_fen()));
                    writeln!(
                        stdout,
                        "{}",
                        UciResponse::<String>::Info(Box::new(board_info))
                    )
                    .unwrap();

                    let best_move = self.think(&mut board, &search_params);

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
                _ => {}
            }
        }

        Ok(())
    }
    pub(crate) fn think(
        &mut self,
        board: &mut Board,
        search_params: &SearchParameters,
    ) -> Option<Move> {
        let mut search = search::Search::new(*search_params);
        let result = search.search(board);
        result.best_move
    }
}

impl Default for ByteKnight {
    fn default() -> Self {
        ByteKnight::new()
    }
}
