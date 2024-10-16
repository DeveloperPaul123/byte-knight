use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

use rayon::prelude::*;

use byte_board::{
    board::Board,
    definitions::DEFAULT_FEN,
    move_generation::MoveGenerator,
    perft::{self},
};
use clap::Parser;
use console::Emoji;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 6)]
    depth: usize,
    #[arg(
        short,
        long,
        default_value_t = DEFAULT_FEN.to_string()
    )]
    fen: String,
    #[arg(short, long)]
    split_perft: bool,

    #[arg(short, long, default_value_t = false)]
    print_moves: bool,

    #[arg(short, long)]
    epd_file: Option<String>,
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| l.unwrap()).collect())
}

static CHECK_BOX: Emoji = Emoji("✅", "");
static CROSS_MARK: Emoji = Emoji("❌", "");

fn main() {
    let args = Args::parse();
    let mut board = Board::from_fen(&args.fen).unwrap();
    let move_generation = MoveGenerator::new();
    let mut all_failures = Vec::new();
    if args.epd_file.is_some() {
        let path = args.epd_file.as_ref().unwrap();
        let lines = read_lines(path).unwrap();
        let now = std::time::Instant::now();
        lines
            .par_iter()
            .map(|line| {
                let parts: Vec<&str> = line.split(";").collect();
                let fen = parts[0];
                let mut failures = Vec::new();
                for part in parts.iter().skip(1) {
                    let parts = part.split_whitespace().collect::<Vec<&str>>();
                    let depth = parts[0].replace("D", "").parse::<usize>().unwrap();
                    let expected_nodes = parts[1].parse::<u64>().unwrap();
                    let mut board = Board::from_fen(fen).unwrap();
                    let nodes = perft::perft(&mut board, &move_generation, depth, false).unwrap();
                    if expected_nodes != nodes {
                        println!(
                            "{:<30}: {:2} {:^10} != {:^10} {}",
                            fen, depth, expected_nodes, nodes, CROSS_MARK
                        );
                        failures.push((fen.to_string(), depth, expected_nodes, nodes));
                    } else {
                        println!(
                            "{:<30}: {:2} {:^10} == {:^10} {}",
                            fen, depth, expected_nodes, nodes, CHECK_BOX
                        );
                    }
                }

                failures
            })
            .collect_into_vec(&mut all_failures);
        let elapsed = now.elapsed();

        println!(
            "Summary:\n\t{} failed\n\t{:.2} seconds",
            all_failures.iter().map(|f| f.len()).sum::<usize>(),
            elapsed.as_secs_f64()
        );

        for (fen, depth, expected, actual) in all_failures.iter().flatten() {
            println!(
                "{:<30}: {:2} {:^10} != {:^10} {}",
                fen, depth, expected, actual, CROSS_MARK
            );
        }
    } else if args.split_perft {
        println!("running split perft at depth {}", args.depth);
        let move_results =
            perft::split_perft(&mut board, &move_generation, args.depth, args.print_moves).unwrap();
        for res in &move_results {
            println!("{}: {}", res.mv.to_long_algebraic(), res.nodes);
        }
        println!();
        // print the total nodes
        println!("{}", move_results.iter().map(|r| r.nodes).sum::<u64>());
    } else {
        let nodes = perft::perft(&mut board, &move_generation, args.depth, false).unwrap();
        println!("{}", nodes);
    };

    // println!("{:?}", result);
}
