use byte_board::{
    board::Board,
    definitions::DEFAULT_FEN,
    move_generation::MoveGenerator,
    perft::{self},
};
use clap::Parser;

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
}

fn main() {
    let args = Args::parse();
    let mut board = Board::from_fen(&args.fen).unwrap();
    let move_generation = MoveGenerator::new();
    let result = if args.split_perft {
        let move_results = perft::split_perft(&mut board, &move_generation, args.depth).unwrap();
        for res in &move_results {
            println!("{}: {}", res.mv.to_short_algebraic(), res.nodes);
        }
        println!();
        // print the total nodes
        println!("{}", move_results.iter().map(|r| r.nodes).sum::<u64>());
    } else {
        let nodes = perft::perft(&mut board, &move_generation, args.depth, false).unwrap();
        println!("{}", nodes);
    };
}
