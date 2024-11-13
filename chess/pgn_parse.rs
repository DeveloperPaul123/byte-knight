use std::str::FromStr;

use crate::pgn::{GameResult, PgnGame, PgnMove};

impl PgnGame {
    pub fn parse(input: &str) -> Result<Vec<PgnGame>, String> {
        let mut games = Vec::new();
        let mut current_game = PgnGame::new();

        for line in input.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                match parse_tag(trimmed, &mut current_game) {
                    Ok((key, value)) => {
                        if key.to_ascii_lowercase() == "result" {
                            current_game.result = GameResult::from_str(&value)?;
                        }

                        if key.to_ascii_lowercase() == "event" {
                            // potentially a new game to parse
                            if current_game.tags.len() > 0 && current_game.moves.len() > 0 {
                                games.push(current_game);
                                current_game = PgnGame::new();
                            }
                        }
                    }
                    Err(e) => return Err(e),
                }
            } else if !trimmed.starts_with('[') {
                parse_moves(trimmed, &mut current_game)?;
            } else if trimmed.is_empty() {
                continue;
            }
        }

        if !current_game.tags.is_empty() || !current_game.moves.is_empty() {
            // sort the moves by number just in case
            current_game.moves.sort_by(|a, b| a.number.cmp(&b.number));
            games.push(current_game);
        }

        Ok(games)
    }
}

fn parse_tag(line: &str, game: &mut PgnGame) -> Result<(String, String), String> {
    let content = line[1..line.len() - 1].trim();
    if let Some(space_idx) = content.find(' ') {
        let key = content[..space_idx].trim();
        let value = content[space_idx..].trim();

        if value.starts_with('"') && value.ends_with('"') {
            let clean_value = value[1..value.len() - 1].to_string();
            game.tags.insert(key.to_string(), clean_value.clone());
            Ok((key.to_string(), clean_value))
        } else {
            Err(format!("Invalid tag value format: {}", line))
        }
    } else {
        Err(format!("Invalid tag format: {}", line))
    }
}

fn parse_moves(line: &str, game: &mut PgnGame) -> Result<(), String> {
    let mut current_number = 0;
    let mut current_white = String::new();
    let mut in_comment = false;
    let mut current_comment = String::new();

    for token in line.split_whitespace() {
        if token.ends_with('.') {
            if let Ok(num) = token[..token.len() - 1].parse::<u32>() {
                if !current_white.is_empty() {
                    game.moves.push(PgnMove {
                        number: current_number,
                        white: current_white.clone(),
                        black: None,
                        comment: None,
                    });
                    current_white.clear();
                }
                current_number = num;
            }
        } else if token.starts_with('{') {
            in_comment = true;
            current_comment.push_str(&token[1..]);
        } else if token.ends_with('}') {
            in_comment = false;
            current_comment.push_str(" ");
            current_comment.push_str(&token[..token.len() - 1]);

            if !current_white.is_empty() {
                game.moves.push(PgnMove {
                    number: current_number,
                    white: current_white.clone(),
                    black: None,
                    comment: Some(current_comment.trim().to_string()),
                });
                current_white.clear();
                current_comment.clear();
            }
        } else if in_comment {
            current_comment.push_str(" ");
            current_comment.push_str(token);
        } else if let Ok(result) = GameResult::from_str(token) {
            game.result = result;
        } else {
            if current_white.is_empty() {
                current_white = token.to_string();
            } else {
                game.moves.push(PgnMove {
                    number: current_number,
                    white: current_white.clone(),
                    black: Some(token.to_string()),
                    comment: None,
                });
                current_white.clear();
            }
        }
    }

    if !current_white.is_empty() {
        game.moves.push(PgnMove {
            number: current_number,
            white: current_white,
            black: None,
            comment: None,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::pgn::PgnGame;

    #[test]
    fn basic_parse() {
        let pgn_content = r#"[Event "Example Game"]
            [Site "Chess.com"]
            [Date "2024.01.01"]
            [White "Player1"]
            [Black "Player2"]
            [Result "1-0"]

            1. e4 e5 2. Nf3 Nc6 3. Bb5 {Ruy Lopez} a6 1-0"#;

        match PgnGame::parse(&pgn_content) {
            Ok(games) => {
                assert!(games.len() == 1);
                for game in games {
                    for (key, value) in &game.tags {
                        println!("{}: {}", key, value);
                    }
                    println!("Moves: {:?}", game.moves);
                    println!("Result: {:?}", game.result);
                }
            }
            Err(e) => println!("Error parsing PGN: {}", e),
        }
    }
}
