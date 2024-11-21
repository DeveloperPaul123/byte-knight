/*
 * main.rs
 * Part of the byte-knight project
 * Created Date: Wednesday, August 14th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Thu Nov 21 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

mod bench;
mod defs;
mod engine;
mod evaluation;
mod input_handler;
mod psqt;
mod score;
mod search;
mod search_thread;
mod tt_table;
mod worker_thread;

use defs::About;
use engine::ByteKnight;

use std::process::exit;

use clap::{Parser, Subcommand};
use std::io::{self, Write};

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
        #[arg(short, long, default_value = "6")]
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
            writeln!(io::stderr(), "Error running engine: {}", e).unwrap();
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
