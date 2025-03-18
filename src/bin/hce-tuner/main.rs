use chess::{definitions::NumberOf, pieces::PIECE_NAMES};
use clap::Parser;
use parameters::Parameters;
use tuner_score::TuningScore;
mod epd_parser;
mod math;
mod offsets;
mod parameters;
mod tuner;
mod tuner_score;
mod tuning_position;

#[derive(Parser, Debug)]
#[command(version, about="Texel tuner for HCE in byte-knight", long_about=None)]
struct Options {
    #[clap(short, long, help = "Filterd, marked EPD input data.")]
    input_data: String,
    #[clap(short, long, help = "Number of epochs to run.")]
    epochs: Option<usize>,
}

fn print_table(indent: usize, table: &[TuningScore]) {
    for rank in 0..8 {
        for file in 0..8 {
            let idx = rank * 8 + file;
            if file == 0 {
                print!("{:indent$}", "", indent = indent);
            }
            let val = table[idx];
            print!("{:?}, ", val);
            if file == 7 {
                println!();
            }
        }
    }
}

fn print_params(params: &Parameters) {
    println!("Tuned parameters:");
    println!("=================");
    println!("pub const PSQTS : [[PhasedScore; NumberOf::SQUARES]; NumberOf::PIECE_TYPES] = [");
    for i in (0..params.len()).step_by(NumberOf::SQUARES) {
        println!("    // {}", PIECE_NAMES[i / (NumberOf::SQUARES)]);
        println!("    [");

        let table = &params.as_slice()[i..i + NumberOf::SQUARES];
        print_table(8, table);
        println!("    ],");
    }
    println!("];");
}

fn main() {
    let options = Options::parse();
    println!("Reading data from: {}", options.input_data);
    let positions = epd_parser::parse_epd_file(options.input_data.as_str());
    // let positions = get_positions();
    println!("Read {} positions", positions.len());

    let epochs = options.epochs.unwrap_or(10_000);
    let mut tuner = tuner::Tuner::new(&positions, epochs);
    let tuned_result = tuner.tune();

    print_params(tuned_result);
}
