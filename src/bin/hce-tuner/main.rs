use clap::Parser;
mod tuner;

#[derive(Parser, Debug)]
#[command(version, about="Texel tuner for HCE in byte-knight", long_about=None)]
struct Options {
    #[clap(short, long, help = "Filterd, marked EPD input data.")]
    input_data: String,
}

fn main() {
    let options = Options::parse();
    println!("Input data: {}", options.input_data);
}
