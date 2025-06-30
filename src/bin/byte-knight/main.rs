/*
 * main.rs
 * Part of the byte-knight project
 * Created Date: Wednesday, August 14th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Fri Apr 11 2025
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

mod bench;

use clap::{Parser, Subcommand};
use engine::defs::About;
use engine::engine::ByteKnight;
use std::process::exit;

#[derive(Parser)]
#[command(
    version = About::VERSION, about = About::SHORT_DESCRIPTION, long_about = About::SHORT_DESCRIPTION
)]
struct Options {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
#[command(about = "Available commands")]
enum Command {
    #[command(about = "Run fixed depth search")]
    Bench {
        #[arg(short, long, default_value = "8")]
        depth: u8,

        #[arg(short, long)]
        epd_file: Option<String>,
    },
}

fn run_uci() {
    let mut engine = ByteKnight::new();
    let engine_run_result = engine.run();
    match engine_run_result {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error running engine: {e}");
            exit(1);
        }
    }
}

fn main() {
    let args = Options::parse();
    match args.command {
        Some(command) => match command {
            Command::Bench { depth, epd_file } => {
                bench::bench(depth, &epd_file);
            }
        },
        None => run_uci(),
    }
}
