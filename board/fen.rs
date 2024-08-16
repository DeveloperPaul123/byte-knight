use std::fmt::{Display, Formatter};

use crate::{board::Board, definitions::Side, pieces::Pieces};

#[derive(Debug)]
pub enum FenPart {
    PiecePlacement = 1,
    ActiveColor = 2,
    CastlingAvailability = 3,
    EnPassantTargetSquare = 4,
    HalfmoveClock = 5,
    FullmoveNumber = 6,
}

impl Display for FenPart {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            FenPart::PiecePlacement => write!(f, "Piece Placement"),
            FenPart::ActiveColor => write!(f, "Active Color"),
            FenPart::CastlingAvailability => write!(f, "Castling Availability"),
            FenPart::EnPassantTargetSquare => write!(f, "En Passant Target Square"),
            FenPart::HalfmoveClock => write!(f, "Halfmove Clock"),
            FenPart::FullmoveNumber => write!(f, "Fullmove Number"),
        }
    }
}

#[derive(Debug)]
pub struct FenError {
    offending_parts: Option<Vec<FenPart>>,
    message: String,
}

impl FenError {
    pub fn new(message: &str) -> FenError {
        FenError {
            offending_parts: None,
            message: message.to_string(),
        }
    }

    pub fn with_offending_parts(message: &str, offending_parts: Vec<FenPart>) -> FenError {
        FenError {
            offending_parts: Some(offending_parts),
            message: message.to_string(),
        }
    }
}

pub type FenResult = Result<(), FenError>;
pub type FenParseResult = Result<Vec<String>, FenError>;
pub type SplitFenStringResult = Result<Vec<String>, FenError>;
pub(crate) type FenPartParser = fn(board: &mut Board, part: &str) -> Result<(), FenError>;

pub(crate) const FEN_PART_PARSERS: [FenPartParser; 6] = [
    parse_piece_placement,
    parse_active_color,
    parse_castling_availability,
    parse_en_passant_target_square,
    parse_halfmove_clock,
    parse_fullmove_number,
];

const DASH: char = '-';
const EM_DASH: char = '–';

pub fn split_fen_string(fen: &str) -> SplitFenStringResult {
    if fen.is_empty() {
        return Err(FenError::new("FEN string is empty"));
    }

    let mut parts = fen
        .replace(EM_DASH, DASH.encode_utf8(&mut [0; 4]))
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();

    if parts.len() == 4 {
        parts.append(&mut vec![String::from("0"), String::from("1")]);
    }

    if parts.len() != 6 {
        return Err(FenError::new("FEN string does not have 6 parts"));
    }

    return Ok(parts);
}

fn parse_piece_placement(board: &mut Board, part: &str) -> FenResult {
    let mut rank = 7;
    let mut file = 0;

    for c in part.chars() {
        match c {
            '/' => {
                if rank == 0 {
                    return Err(FenError::new(&format!(
                        "Extra / found in FEN part {}",
                        FenPart::PiecePlacement,
                    )));
                }
                rank -= 1;
                file = 0;
            }
            '1'..='8' => {
                let num_empty_squares = c.to_digit(10).unwrap() as usize;
                file += num_empty_squares;
            }
            _ => {
                let piece = match c {
                    'P' | 'p' => Pieces::PAWN,
                    'N' | 'n' => Pieces::KNIGHT,
                    'B' | 'b' => Pieces::BISHOP,
                    'R' | 'r' => Pieces::ROOK,
                    'Q' | 'q' => Pieces::QUEEN,
                    'K' | 'k' => Pieces::KING,
                    _ => panic!("Invalid piece"),
                };

                let side = if c.is_ascii_uppercase() {
                    Side::WHITE
                } else {
                    Side::BLACK
                };

                let square = rank * 8 + file;
                board.set_piece_square(piece as usize, side, square);

                file += 1;
            }
        }
    }

    return Ok(());
}

fn parse_active_color(board: &mut Board, part: &str) -> FenResult {
    // TODO
    return Ok(());
}
fn parse_en_passant_target_square(board: &mut Board, part: &str) -> FenResult {
    // TODO
    return Ok(());
}
fn parse_castling_availability(board: &mut Board, part: &str) -> FenResult {
    // TODO
    return Ok(());
}
fn parse_halfmove_clock(board: &mut Board, part: &str) -> FenResult {
    // TODO
    return Ok(());
}
fn parse_fullmove_number(board: &mut Board, part: &str) -> FenResult {
    // TODO
    return Ok(());
}
