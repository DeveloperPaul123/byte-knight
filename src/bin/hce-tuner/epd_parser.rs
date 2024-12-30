use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::tuner::Position;

pub(crate) fn parse_epd_file(file_path: &str) -> Vec<Position> {
    let mut positions = Vec::new();
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        // TODO
    }
    positions
}

fn parse_epd_line(line: &str) -> Result<Position> {
    let mut parts = line.rsplitn(2, ' ');
    if parts.try_len().unwrap() < 2 {
        return Err(anyhow!("Failed to parse EPD line"));
    }

    // EPD result
    let mut game_result_part = parts.next();
    let fen_part = parts.next();

    let game_result = get_game_result(game_result_part.unwrap())?;
    Err(anyhow!("Not implemented"))
}

fn get_game_result(part: &str) -> Result<f64> {
    // first sanitize the string
    let part = part.trim();
    // remove any brackets, braces, parenthesis, and semicolons
    let part = part.replace(&['[', ']', '{', '}', '(', ')', ';'][..], "");

    if part.starts_with("draw") || part.starts_with("1/2") {
        return Ok(0.5);
    } else if part.starts_with("1-0") {
        return Ok(1.0);
    } else if part.starts_with("0-1") {
        return Ok(0.0);
    } else {
        // try to parse as f64 direcly
        part.parse::<f64>()
            .map_err(|_| anyhow!("Failed to parse game result"))
    }
}

#[cfg(test)]
mod tests {
    use crate::epd_parser::get_game_result;

    #[test]
    fn game_result() {
        let results = vec![
            "[0.75]",
            "0.75;",
            "[1/2-1/2]",
            "    1/2-1/2;",
            "[1-0]  ",
            " 1-0;",
        ];
        let values = vec![0.75, 0.75, 0.5, 0.5, 1.0, 1.0];
        for (i, &result) in results.iter().enumerate() {
            let game_result = get_game_result(result).unwrap();
            assert_eq!(game_result, values[i]);
        }
    }
}
