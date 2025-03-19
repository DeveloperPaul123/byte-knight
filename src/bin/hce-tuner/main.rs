use std::process::exit;

use chess::{definitions::NumberOf, pieces::PIECE_NAMES};
use clap::Parser;
use indicatif::ParallelProgressIterator;
use parameters::Parameters;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use textplots::{Chart, Plot, Shape};
use tuner::Tuner;
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
    #[clap(
        long,
        action,
        default_value_t = false,
        help = "Plot k versus error for the given dataset"
    )]
    plot_k: bool,
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

fn plot_k(tuner: &Tuner) {
    let mut points = Vec::new();
    let data_point_count = 10_000;
    let k_min = -0.1;
    let k_max = 0.1;
    (0..data_point_count)
        .into_par_iter()
        .progress_count(data_point_count as u64)
        .map(|val| {
            let k = val as f64 / data_point_count as f64 * (k_max - k_min) + k_min;
            let error = tuner.mean_square_error(k);
            (k as f32, error as f32)
        })
        .collect_into_vec(&mut points);

    Chart::new(180, 60, k_min as f32, k_max as f32)
        .lineplot(&Shape::Points(points.as_slice()))
        .display();
}

fn main() {
    let options = Options::parse();
    println!("Reading data from: {}", options.input_data);
    let positions = epd_parser::parse_epd_file(options.input_data.as_str());
    // let positions = get_positions();
    println!("Read {} positions", positions.len());

    let epochs = options.epochs.unwrap_or(10_000);
    let parameters = Parameters::create_from_engine_values();
    let mut tuner = tuner::Tuner::new(parameters, &positions, epochs);

    if options.plot_k {
        plot_k(&tuner);
        exit(0);
    }

    let tuned_result = tuner.tune();

    print_params(tuned_result);
}
