use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};
use chess::{bitboard_helpers, board::Board, pieces::Piece, side::Side};
use engine::psqt::GAMEPHASE_INC;

use crate::{offsets::Offsets, tuning_position::TuningPosition};

pub(crate) fn parse_epd_file(file_path: &str) -> Vec<TuningPosition> {
    let mut positions = Vec::new();
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let pos = parse_epd_line(line.as_str());
        if let Ok(pos) = pos {
            positions.push(pos);
        }
    }
    positions
}

fn process_epd_line(line: &str) -> Result<(Board, f64)> {
    let mut parts = line.rsplitn(2, ' ');
    // EPD result
    let game_result_part = parts.next();
    let game_result = get_game_result(game_result_part.unwrap())?;

    // FEN
    let fen_part = parts.next();
    let fen = fen_part.unwrap().trim();
    let board = Board::from_fen(fen)?;

    Ok((board, game_result))
}

fn parse_epd_line(line: &str) -> Result<TuningPosition> {
    let (board, game_result) = process_epd_line(line)?;
    let mut w_indexes = Vec::new();
    let mut b_indexes = Vec::new();
    // loop through all pieces on the board and calculate the index into the parameter array
    // for each piece
    let mut phase = 0;
    let offsets = Offsets::new();
    for piece in Piece::iter() {
        let mut w_bb = *board.piece_bitboard(piece, Side::White);
        let mut b_bb = *board.piece_bitboard(piece, Side::Black);

        // update game phase
        phase += w_bb.as_number().count_ones() as usize * GAMEPHASE_INC[piece as usize] as usize;
        phase += b_bb.as_number().count_ones() as usize * GAMEPHASE_INC[piece as usize] as usize;
        while w_bb.as_number() > 0 {
            let sq = bitboard_helpers::next_bit(&mut w_bb);
            let mg_start_index =
                offsets.start_index_for_piece(piece, crate::tuner::TableType::Midgame)?;
            let eg_start_index =
                offsets.start_index_for_piece(piece, crate::tuner::TableType::Endgame)?;

            w_indexes.push(mg_start_index + sq);
            w_indexes.push(eg_start_index + sq);
        }
        // repeat for black
        while b_bb.as_number() > 0 {
            let sq = bitboard_helpers::next_bit(&mut b_bb);
            let mg_start_index =
                offsets.start_index_for_piece(piece, crate::tuner::TableType::Midgame)?;
            let eg_start_index =
                offsets.start_index_for_piece(piece, crate::tuner::TableType::Endgame)?;
            b_indexes.push(mg_start_index + sq);
            b_indexes.push(eg_start_index + sq);
        }
    }

    let tuning_pos = TuningPosition::new(
        w_indexes,
        b_indexes,
        phase,
        game_result,
        board.side_to_move(),
    );

    Ok(tuning_pos)
}

/// Parse the game result from part of the EPD line.
/// The game result can be in the following formats:
/// - 0.0
/// - 1.0
/// - 0.5
/// - 1-0
/// - 0-1
/// - 1/2-1/2
/// - [0.75]
/// - 0.75;
/// - [1/2-1/2]
/// - 1/2-1/2;
/// - [1-0]
/// - 1-0;
/// - [0-1]
/// - 0-1;
/// - [draw]
/// - draw;
///
/// The function will return the game result as a f64.
///
/// # Arguments
/// - `part` - A part of the EPD line that contains the game result.
///
/// # Returns
/// A f64 representing the game result. 0.0 for a loss, 0.5 for a draw, and 1.0 for a win.
fn get_game_result(part: &str) -> Result<f64> {
    // first sanitize the string
    let part = part.trim();
    // remove any brackets, braces, parenthesis, semicolons, and double quotes
    let part = part.replace(&['[', ']', '{', '}', '(', ')', ';', '"'][..], "");

    if part.starts_with("draw") || part.starts_with("1/2") {
        Ok(0.5)
    } else if part.starts_with("1-0") {
        Ok(1.0)
    } else if part.starts_with("0-1") {
        Ok(0.0)
    } else {
        // try to parse as f64 direcly
        part.parse::<f64>()
            .map_err(|_| anyhow!("Failed to parse game result"))
    }
}

#[cfg(test)]
mod tests {
    use chess::side::Side;

    use crate::epd_parser::{get_game_result, process_epd_line};

    #[test]
    fn game_result() {
        let results = [
            "[0.75]",
            "0.75;",
            "[1/2-1/2]",
            "    1/2-1/2;",
            "[1-0]  ",
            " 1-0;",
            "\"0-1\"",
        ];
        let values = [0.75, 0.75, 0.5, 0.5, 1.0, 1.0, 0.0];
        for (i, &result) in results.iter().enumerate() {
            let game_result = get_game_result(result).unwrap();
            assert_eq!(game_result, values[i]);
        }
    }

    #[test]
    fn epd_line() {
        let epd_lines = vec![
            // from lichess big3
            "5r2/p4pk1/2pb4/8/1p2rN2/4p3/PPPB4/3K4 w - - 0 3 [0.0]",
            "r2q1rk1/3n1p2/2pp3p/1pb1p1p1/p3P3/P1NP1N1P/RPP2PP1/5QK1 b - - 0 2 [0.0]",
            "rn2r2k/p1R4p/4bp2/8/1Q6/6P1/1P3P1P/6K1 w - - 0 1 [0.0]",
            "1r4k1/6p1/7p/4p3/R7/3rPNP1/1b3P1P/5RK1 b - - 0 1 [1.0]",
            "1nn3kr/1R1p2pp/5p2/N1p5/3PP3/3B4/P1P2PPP/R5K1 b - - 0 3 [1.0]",
            "6k1/1p2b1pp/p4p2/4pb2/1P1pN3/P2P1P1P/2r3P1/1R3NK1 w - - 0 1 [0.0]",
            "rn1q2k1/ppp2ppp/3p1n2/2bb4/8/5NP1/PPP1NPBP/R4RK1 w - - 0 1 [0.0]",
            "3r1rk1/pR3pbp/2p1pnp1/4q3/2P4P/P3P1P1/2Q2PB1/2B2RK1 b - - 0 4 [0.0]",
            "3b4/5k2/6r1/3pP3/p1pP1p1p/P1P2P1P/1PR3P1/6K1 b - - 0 1 [0.0]",
            "r2q1rk1/ppp1npbp/4b1p1/1P3nN1/2Pp4/3P4/PB1NBPPP/R2QR1K1 b - - 0 1 [0.0]",
        ];

        const EXPECTED_GAME_PHASES: [usize; 10] = [7, 18, 12, 10, 10, 8, 17, 20, 5, 24];
        const EXPECTED_GAME_RESULTS: [f64; 10] = [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        for (i, line) in epd_lines.iter().enumerate() {
            let position = super::parse_epd_line(line);
            assert!(position.is_ok());
            let pos = position.unwrap();
            let (board, _) = process_epd_line(line).unwrap();
            let total_piece_count = board.all_pieces().as_number().count_ones();
            assert_eq!(
                pos.parameter_indexes[Side::White as usize].len()
                    + pos.parameter_indexes[Side::Black as usize].len(),
                total_piece_count as usize * 2
            );
            assert_eq!(pos.phase, EXPECTED_GAME_PHASES[i]);
            assert_eq!(pos.game_result, EXPECTED_GAME_RESULTS[i]);
        }
    }
}
