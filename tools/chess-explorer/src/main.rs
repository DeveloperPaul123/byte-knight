use std::path::PathBuf;

use chess::board::Board;
use gpui::{App, AppContext, Application, Bounds, WindowBounds, WindowOptions, px, size};

mod assets;
mod chess_board;

use chess_board::ChessBoard;
use clap::Parser;

use crate::assets::Assets;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = None)]
    fen: Option<String>,
}

// Main entry point
fn main() {
    let cli = Cli::parse();
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    Application::new()
        .with_assets(Assets {
            base_directory: base_dir,
        })
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(600.0), px(600.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    app_id: Some("Chess Board".to_string()),
                    is_resizable: false,
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_cx| {
                        let fen_str = cli.fen.unwrap_or(
                            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - -".to_string(),
                        );

                        let maybe_board = Board::from_fen(&fen_str);
                        if maybe_board.is_err() {
                            panic!("Invalid FEN string: {}", fen_str);
                        }

                        let board = maybe_board.unwrap();
                        println!("{}", board);

                        ChessBoard::new(board)
                    })
                },
            )
            .unwrap();
        });
}
