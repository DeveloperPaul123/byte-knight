use clap::Parser;
mod epd_parser;
mod math;
mod offsets;
mod parameters;
mod tuner;
mod tuning_position;

#[derive(Parser, Debug)]
#[command(version, about="Texel tuner for HCE in byte-knight", long_about=None)]
struct Options {
    #[clap(short, long, help = "Filterd, marked EPD input data.")]
    input_data: String,
}

// fn print_table(indent: usize, table: &[ScoreType]) {
//     for rank in 0..8 {
//         for file in 0..8 {
//             let idx = rank * 8 + file;
//             if file == 0 {
//                 print!("{:indent$}", "", indent = indent);
//             }

//             print!(
//                 "S({:4}, {:4}), ",
//                 table[idx],
//                 table[NumberOf::SQUARES + idx]
//             );
//             if file == 7 {
//                 println!();
//             }
//         }
//     }
// }

// fn print_params(params: &Parameters) {
//     println!("Tuned parameters:");
//     println!("=================");
//     println!("pub const PSQTS : [[PhasedScore; NumberOf::SQUARES]; NumberOf::PIECE_TYPES] = [");
//     for i in (0..params.len()).step_by(NumberOf::SQUARES * 2) {
//         println!("    // {}", PIECE_NAMES[i / (NumberOf::SQUARES * 2)]);
//         println!("    [");

//         let table = &params[i..i + NumberOf::SQUARES * 2];
//         print_table(8, table);
//         println!("    ],");
//     }
//     println!("];");
// }

fn main() {
    let options = Options::parse();
    println!("Reading data from: {}", options.input_data);
    let positions = epd_parser::parse_epd_file(options.input_data.as_str());
    // let positions = get_positions();
    println!("Read {} positions", positions.len());

    // TODO: Fix this
    let mut tuner = tuner::Tuner::new(&positions);
    let tuned_result = tuner.tune();

    // print_params(&tuned_result);
}
