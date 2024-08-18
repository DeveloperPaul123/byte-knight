use std::fmt::{Display, Formatter};

use crate::{
    board::Board,
    definitions::{CastlingAvailability, Side},
    pieces::{Pieces, SQUARE_NAME},
    square::to_square,
};

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

impl Display for FenError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(parts) = &self.offending_parts {
            write!(f, " Offending parts: ")?;
            for part in parts {
                write!(f, "{} ", part)?;
            }
        }
        Ok(())
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

                let square = to_square(file as u8, rank);
                board.set_piece_square(piece as usize, side, square as usize);

                file += 1;
            }
        }
    }

    return Ok(());
}

pub(crate) fn piece_placement_to_fen(board: &Board) -> String {
    let mut fen = String::new();

    for rank in (0..8).rev() {
        let mut empty_squares = 0;
        for file in 0..8 {
            let square = to_square(file, rank);
            let maybe_piece = board.piece_on_square(square as usize);
            if let Some(piece) = maybe_piece {
                if empty_squares > 0 {
                    fen.push_str(&empty_squares.to_string());
                    empty_squares = 0;
                }
                let p = piece.0;
                let color = piece.1;

                let symbol = match p as u8 {
                    Pieces::PAWN => 'P',
                    Pieces::KNIGHT => 'N',
                    Pieces::BISHOP => 'B',
                    Pieces::ROOK => 'R',
                    Pieces::QUEEN => 'Q',
                    Pieces::KING => 'K',
                    _ => panic!("Invalid piece"),
                };

                let side = if color == Side::WHITE {
                    symbol
                } else {
                    symbol.to_ascii_lowercase()
                };

                fen.push(side);
            } else {
                empty_squares += 1;
            }
        }

        if empty_squares > 0 {
            fen.push_str(&empty_squares.to_string());
        }

        if rank > 0 {
            fen.push('/');
        }
    }

    return fen;
}

fn parse_active_color(board: &mut Board, part: &str) -> FenResult {
    if part.len() != 1 {
        return Err(FenError::new(&format!(
            "Active color length is invalid in FEN part {}",
            FenPart::ActiveColor,
        )));
    }
    if !['w', 'b'].contains(&part.chars().next().unwrap()) {
        return Err(FenError::new(&format!(
            "Invalid active color found in FEN part {}",
            FenPart::ActiveColor,
        )));
    }

    match part.trim() {
        "w" => board.set_side_to_move(Side::WHITE),
        "b" => board.set_side_to_move(Side::BLACK),
        _ => {
            return Err(FenError::new(&format!(
                "Invalid active color found in FEN part {}",
                FenPart::ActiveColor,
            )));
        }
    }
    return Ok(());
}

pub(crate) fn active_color_to_fen(board: &Board) -> String {
    return match board.side_to_move() {
        Side::WHITE => "w".to_string(),
        Side::BLACK => "b".to_string(),
        _ => panic!("Invalid side"),
    };
}

fn parse_en_passant_target_square(board: &mut Board, part: &str) -> FenResult {
    let part_length = part.len();

    // any dash present was previously converted to DASH
    if part_length == 1 && part.chars().next().unwrap() == DASH {
        board.set_en_passant_square(None);
        return Ok(());
    }

    if part_length != 2 {
        return Err(FenError::new(&format!(
            "Invalid en passant target square length in FEN part {}",
            FenPart::EnPassantTargetSquare,
        )));
    }

    let search_part = part.to_lowercase();
    if SQUARE_NAME.contains(&search_part.trim()) {
        let index = SQUARE_NAME
            .iter()
            .position(|&r| r == part.trim().to_lowercase())
            .unwrap();
        board.set_en_passant_square(Some(index as u8));
        return Ok(());
    }

    return Err(FenError::new(&format!(
        "Invalid en passant target square found in FEN part {}",
        FenPart::EnPassantTargetSquare,
    )));
}

pub(crate) fn en_passant_target_square_to_fen(board: &Board) -> String {
    return match board.en_passant_square() {
        Some(square) => SQUARE_NAME[square as usize].to_string(),
        None => "-".to_string(),
    };
}

fn parse_castling_availability(board: &mut Board, part: &str) -> FenResult {
    if part.is_empty() {
        return Err(FenError::new(&format!(
            "Empty castling availability found in FEN part {}",
            FenPart::CastlingAvailability,
        )));
    }

    if part.len() == 1 && part.trim().chars().next().unwrap() == DASH {
        return Ok(());
    }

    let mut castle_rights = CastlingAvailability::NONE;

    for c in part.chars() {
        match c {
            'K' => castle_rights |= CastlingAvailability::WHITE_KINGSIDE,
            'Q' => castle_rights |= CastlingAvailability::WHITE_QUEENSIDE,
            'k' => castle_rights |= CastlingAvailability::BLACK_KINGSIDE,
            'q' => castle_rights |= CastlingAvailability::BLACK_QUEENSIDE,
            _ => {
                return Err(FenError::new(&format!(
                    "Invalid castling availability found in FEN part {}",
                    FenPart::CastlingAvailability,
                )));
            }
        }
    }

    board.set_castling_rights(castle_rights);

    return Ok(());
}

pub(crate) fn castling_availability_to_fen(board: &Board) -> String {
    let mut fen = String::new();

    if board.castling_rights() == CastlingAvailability::NONE {
        return "-".to_string();
    }

    if board.castling_rights() & CastlingAvailability::WHITE_KINGSIDE != 0 {
        fen.push('K');
    }
    if board.castling_rights() & CastlingAvailability::WHITE_QUEENSIDE != 0 {
        fen.push('Q');
    }
    if board.castling_rights() & CastlingAvailability::BLACK_KINGSIDE != 0 {
        fen.push('k');
    }
    if board.castling_rights() & CastlingAvailability::BLACK_QUEENSIDE != 0 {
        fen.push('q');
    }

    return fen;
}

fn parse_halfmove_clock(board: &mut Board, part: &str) -> FenResult {
    let halfmove_clock = part.trim().parse::<u32>().unwrap();
    board.set_half_move_clock(halfmove_clock);
    return Ok(());
}

pub(crate) fn halfmove_clock_to_fen(board: &Board) -> String {
    return board.half_move_clock().to_string();
}

fn parse_fullmove_number(board: &mut Board, part: &str) -> FenResult {
    let fullmove_number = part.trim().parse::<u32>().unwrap();
    board.set_full_move_number(fullmove_number);
    return Ok(());
}

pub(crate) fn fullmove_number_to_fen(board: &Board) -> String {
    return board.full_move_number().to_string();
}
