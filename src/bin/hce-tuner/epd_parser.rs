use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Result, anyhow};
use chess::{bitboard_helpers, board::Board, pieces::Piece, side::Side, square};
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
    for piece in Piece::iter() {
        let mut w_bb = *board.piece_bitboard(piece, Side::White);
        let mut b_bb = *board.piece_bitboard(piece, Side::Black);

        // update game phase
        phase += w_bb.as_number().count_ones() as usize * GAMEPHASE_INC[piece as usize] as usize;
        phase += b_bb.as_number().count_ones() as usize * GAMEPHASE_INC[piece as usize] as usize;
        while w_bb.as_number() > 0 {
            let sq = bitboard_helpers::next_bit(&mut w_bb);
            // note we still have to flip the square for the white side
            let index =
                Offsets::offset_for_piece_and_square(square::flip(sq as u8) as usize, piece);
            w_indexes.push(index);
        }
        // repeat for black
        while b_bb.as_number() > 0 {
            let sq = bitboard_helpers::next_bit(&mut b_bb);
            let index = Offsets::offset_for_piece_and_square(sq, piece);
            b_indexes.push(index);
        }
    }

    let stm = match board.side_to_move() {
        Side::White => 1f64,
        Side::Black => -1f64,
        Side::Both => panic!("Side to move cannot be both."),
    };

    let result = match board.side_to_move() {
        Side::White => game_result,
        Side::Black => 1.0 - game_result,
        Side::Both => panic!("Side to move cannot be both."),
    };

    let tuning_pos = TuningPosition::new(w_indexes, b_indexes, phase, result, stm);

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
        // try to parse as f64 directly
        part.parse::<f64>()
            .map_err(|_| anyhow!("Failed to parse game result"))
    }
}

#[cfg(test)]
mod tests {
    use chess::side::Side;
    use engine::{evaluation::ByteKnightEvaluation, traits::Eval};

    use crate::{
        epd_parser::{get_game_result, process_epd_line},
        parameters::Parameters,
    };

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
        let epd_lines = [
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
        let eval = ByteKnightEvaluation::default();
        let params = Parameters::create_from_engine_values();

        for (i, line) in epd_lines.iter().enumerate() {
            let position = super::parse_epd_line(line);
            assert!(position.is_ok());
            let pos = position.unwrap();
            let (board, _) = process_epd_line(line).unwrap();
            let total_piece_count = board.all_pieces().as_number().count_ones();
            assert_eq!(
                pos.parameter_indexes[Side::White as usize].len()
                    + pos.parameter_indexes[Side::Black as usize].len(),
                total_piece_count as usize
            );
            assert_eq!(pos.phase, EXPECTED_GAME_PHASES[i]);
            assert_eq!(pos.game_result, EXPECTED_GAME_RESULTS[i]);
            // also verify that the evaluation matches
            let expected_value = eval.eval(&board);
            let val = pos.evaluate(&params);
            println!("{} // {}", expected_value, val);
            assert!((expected_value.0 as f64 - val).abs().round() <= 1.0)
        }
    }

    #[test]
    fn gedas_epd_lines() {
        let epd_lines = [
            "8/8/7p/1P2k2P/4p1P1/1p1r4/1R2K3/8 b - - ce 0.7306",
            "2r3k1/1pr2qp1/p2bpp1p/3p1n2/3P1PP1/2P2N2/RQ2NP1P/4R2K b - - ce 0.8325",
            "r2q1rk1/p1bb1ppp/P1n1pn2/2p5/1pN5/1Q1P1NP1/1P1BPPBP/2R2RK1 b - - ce 0.4102",
            "4k3/3n1p2/p3p1rp/4P1B1/3p1P2/b1p1q3/P1R3PP/2RQ3K w - - ce 0.2295",
            "3q1rk1/3bpp1p/6p1/1Bn1P1P1/3p1B2/8/2P3PP/Q4RK1 w - - ce 0.4457",
            "2rk4/3r2p1/p1pb1p1p/P3p3/1PR4P/3RP1P1/3BKP2/8 b - - ce 0.4194",
            "1R6/2p3pk/3n1q1p/2Q1p3/2p1P3/6P1/4K2P/8 w - - ce 0.5295",
            "4r1k1/Rb5p/5pp1/3pn3/3N1B2/4P2P/5PP1/6K1 b - - ce 0.4183",
            "8/4k3/R2b4/4pp2/5r2/2P2P1P/1P3KB1/8 b - - ce 0.2446",
            "2R5/r3p1kp/5pp1/pR2n3/4P3/PP2K1PP/1r6/5B2 w - - ce 0.4323",
            "2b1r1k1/4q2p/4P1p1/3NQp2/1p1Rn3/5BP1/PP5P/1K6 b - - ce 0.2024",
            "8/6k1/5p2/2pPqB1P/2Pp2K1/8/5Q1b/8 b - - ce 0.4936",
            "2r1k3/8/b2p1p2/p3p1r1/Pp2P1Bp/1Pq4P/2P2R2/2Q1R2K b - - ce 0.6721",
        ];

        for line in epd_lines.iter() {
            let position = super::parse_epd_line(line);
            assert!(position.is_ok());
            let pos = position.unwrap();
            let (board, _) = process_epd_line(line).unwrap();
            let total_piece_count = board.all_pieces().as_number().count_ones();
            assert_eq!(
                pos.parameter_indexes[Side::White as usize].len()
                    + pos.parameter_indexes[Side::Black as usize].len(),
                total_piece_count as usize
            );
        }
    }
}
