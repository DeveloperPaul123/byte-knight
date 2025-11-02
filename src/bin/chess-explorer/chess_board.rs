use chess::{board::Board, side::Side, square};
use gpui::{div, img, prelude::*, px, rgb, rgba, Context, Rgba, Window};

use chess::pieces::Piece;

#[derive(Clone, Copy, Debug)]
struct ChessPiece {
    piece: Piece,
    side: Side,
}

impl ChessPiece {
    fn new(piece: Piece, side: Side) -> Self {
        Self { piece, side }
    }

    fn to_svg_path(&self) -> String {
        let piece_char = match self.side {
            Side::White => self.piece.as_char().to_ascii_lowercase(),
            Side::Black => self.piece.as_char().to_ascii_uppercase(),
        };

        format!(
            "pieces/{}{}.svg",
            format!("{}", self.side).to_ascii_lowercase(),
            piece_char
        )
    }
}

// Chess board state
pub(crate) struct ChessBoard {
    board: Board,
    selected_square: Option<(usize, usize)>,
}

impl ChessBoard {
    pub(crate) fn new(board: Board) -> Self {
        let board = Self {
            board: board,
            selected_square: None,
        };
        board
    }

    fn update_selected_square(&mut self, rank: usize, file: usize, cx: &mut Context<Self>) {
        self.selected_square = Some((rank, file));
        cx.notify();
    }

    fn clear_selected_square(&mut self, cx: &mut Context<Self>) {
        self.selected_square = None;
        cx.notify();
    }
}

fn fen_title(fen: String) -> impl IntoElement {
    div()
        .text_sm()
        .text_color(rgb(0xaaaaaa))
        .child(format!("FEN: {}", fen))
}

impl Render for ChessBoard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // TODO: Calculate this based on the window size
        let square_size = px(60.0);
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x2d2d2d))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(fen_title(self.board.to_fen()))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .border_2()
                            .justify_center()
                            .content_center()
                            .border_color(rgb(0x444444))
                            .children((0..8).map(|rank| {
                                div()
                                    .flex()
                                    .flex_row()
                                    // Draw the board
                                    .children((0..8).map(|file| {
                                        let is_light = (rank + file) % 2 == 0;
                                        let is_selected =
                                            self.selected_square == Some((rank, file));
                                        let selected_color: Rgba = if is_selected {
                                            rgba(0x0000007f)
                                        } else {
                                            rgba(0x00000000)
                                        };

                                        let bg_color = if is_light {
                                            rgb(0xf0d9b5)
                                        } else {
                                            rgb(0xb58863)
                                        };

                                        let piece = self.board.piece_on_square(square::to_square(
                                            file as u8, rank as u8,
                                        ));

                                        div()
                                            .size(square_size)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .bg(bg_color)
                                            .border_6()
                                            .border_color(selected_color)
                                            .text_color(if is_light {
                                                rgb(0x000000)
                                            } else {
                                                rgb(0x000000)
                                            })
                                            .text_size(px(40.0))
                                            .cursor_pointer()
                                            .on_mouse_down(
                                                gpui::MouseButton::Left,
                                                _cx.listener(move |this, _event, _win, cx| {
                                                    // Only allow selecting squares with pieces for now
                                                    if piece.is_some() {
                                                        this.update_selected_square(rank, file, cx);
                                                    } else {
                                                        // clear selection
                                                        this.clear_selected_square(cx);
                                                    }
                                                }),
                                            )
                                            .when_some(piece, |div, (p, side)| {
                                                let chess_p = ChessPiece::new(p, side);
                                                div.child(
                                                    img(chess_p.to_svg_path())
                                                        .size_11()
                                                        .flex_shrink(),
                                                )
                                            })
                                    }))
                            })),
                    ),
            )
    }
}
