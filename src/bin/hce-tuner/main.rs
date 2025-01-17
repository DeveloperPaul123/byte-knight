use chess::{board::Board, definitions::NumberOf, pieces::PIECE_NAMES};
use clap::Parser;
use engine::score::ScoreType;
mod epd_parser;
mod offsets;
mod tuner;
mod tuner_values;

#[derive(Parser, Debug)]
#[command(version, about="Texel tuner for HCE in byte-knight", long_about=None)]
struct Options {
    #[clap(short, long, help = "Filterd, marked EPD input data.")]
    input_data: String,
}

#[allow(dead_code)]
fn get_positions() -> Vec<tuner::Position> {
    vec![
        tuner::Position {
            board: Board::from_fen("5r2/p4pk1/2pb4/8/1p2rN2/4p3/PPPB4/3K4 w - - 0 3").unwrap(),
            game_result: 0.0,
        },
        tuner::Position {
            board: Board::from_fen(
                "r2q1rk1/3n1p2/2pp3p/1pb1p1p1/p3P3/P1NP1N1P/RPP2PP1/5QK1 b - - 0 2",
            )
            .unwrap(),
            game_result: 0.0,
        },
        tuner::Position {
            board: Board::from_fen("rn2r2k/p1R4p/4bp2/8/1Q6/6P1/1P3P1P/6K1 w - - 0 1").unwrap(),
            game_result: 0.0,
        },
        tuner::Position {
            board: Board::from_fen("1r4k1/6p1/7p/4p3/R7/3rPNP1/1b3P1P/5RK1 b - - 0 1").unwrap(),
            game_result: 1.0,
        },
        tuner::Position {
            board: Board::from_fen("1nn3kr/1R1p2pp/5p2/N1p5/3PP3/3B4/P1P2PPP/R5K1 b - - 0 3")
                .unwrap(),
            game_result: 1.0,
        },
        tuner::Position {
            board: Board::from_fen("6k1/1p2b1pp/p4p2/4pb2/1P1pN3/P2P1P1P/2r3P1/1R3NK1 w - - 0 1")
                .unwrap(),
            game_result: 0.0,
        },
        tuner::Position {
            board: Board::from_fen("rn1q2k1/ppp2ppp/3p1n2/2bb4/8/5NP1/PPP1NPBP/R4RK1 w - - 0 1")
                .unwrap(),
            game_result: 0.0,
        },
        tuner::Position {
            board: Board::from_fen("3r1rk1/pR3pbp/2p1pnp1/4q3/2P4P/P3P1P1/2Q2PB1/2B2RK1 b - - 0 4")
                .unwrap(),
            game_result: 0.0,
        },
        tuner::Position {
            board: Board::from_fen("3b4/5k2/6r1/3pP3/p1pP1p1p/P1P2P1P/1PR3P1/6K1 b - - 0 1")
                .unwrap(),
            game_result: 0.0,
        },
        tuner::Position {
            board: Board::from_fen(
                "r2q1rk1/ppp1npbp/4b1p1/1P3nN1/2Pp4/3P4/PB1NBPPP/R2QR1K1 b - - 0 1",
            )
            .unwrap(),
            game_result: 0.0,
        },
        tuner::Position {
            board: Board::from_fen("8/k7/n3n1p1/R1p1P2p/2PP3r/4K3/8/8 w - - 0 1").unwrap(),
            game_result: 0.0,
        },
        tuner::Position {
            board: Board::from_fen("r3kb1Q/p2qn3/1pp3p1/3p4/3P1B2/2N5/PPP2PPP/4R1K1 w - - 0 1")
                .unwrap(),
            game_result: 1.0,
        },
        tuner::Position {
            board: Board::from_fen("8/4n3/2k3p1/5p1p/Pp3P1P/1P3KP1/3r4/8 w - - 0 1").unwrap(),
            game_result: 0.0,
        },
        tuner::Position {
            board: Board::from_fen("6k1/1p4pp/1r4b1/1p2N3/8/7P/1P3PP1/6K1 w - - 0 3").unwrap(),
            game_result: 0.0,
        },
        tuner::Position {
            board: Board::from_fen("8/7k/4R3/4p2b/4P3/2r2P2/4BK2/8 b - - 0 1").unwrap(),
            game_result: 0.5,
        },
        tuner::Position {
            board: Board::from_fen("6k1/8/3R3P/6K1/4r2P/8/8/8 b - - 0 1").unwrap(),
            game_result: 0.5,
        },
        tuner::Position {
            board: Board::from_fen("7k/8/3p3P/2nP2p1/2P5/P1b1rP2/K5P1/3R1B1R b - - 0 1").unwrap(),
            game_result: 1.0,
        },
        tuner::Position {
            board: Board::from_fen("r4rk1/ppq3pp/4p3/2b2p1Q/P7/1bP3P1/1P3P1P/R1B2RK1 w - - 0 1")
                .unwrap(),
            game_result: 0.0,
        },
        tuner::Position {
            board: Board::from_fen(
                "r2q1rk1/p3b3/1p1p1p1B/2pPp2n/2P1P1P1/P2B1N2/1P1Q1P2/R4RK1 b - - 0 1",
            )
            .unwrap(),
            game_result: 1.0,
        },
        tuner::Position {
            board: Board::from_fen("8/pp3kpp/5p2/3P4/5P1P/1P4K1/3r4/2r5 w - - 0 1").unwrap(),
            game_result: 0.0,
        },
    ]
}

fn print_table(indent: usize, table: &[ScoreType]) {
    for rank in 0..8 {
        for file in 0..8 {
            let idx = rank * 8 + file;
            if file == 0 {
                print!("{:indent$}", "", indent = indent);
            }

            print!(
                "S({:4}, {:4}), ",
                table[idx],
                table[NumberOf::SQUARES + idx]
            );
            if file == 7 {
                println!();
            }
        }
    }
}

fn print_params(params: &Vec<ScoreType>) {
    println!("Tuned parameters:");
    println!("=================");
    println!("pub const PSQTS : [[PhasedScore; NumberOf::SQUARES]; NumberOf::PIECE_TYPES] = [");
    for i in (0..params.len()).step_by(NumberOf::SQUARES * 2) {
        println!("    // {}", PIECE_NAMES[i / (NumberOf::SQUARES * 2)]);
        println!("    [");

        let table = &params[i..i + NumberOf::SQUARES * 2];
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

    // TODO: Fix this
    // let mut tuner = tuner::Tuner::new(&positions);
    // let tuned_result = tuner.tune();

    // print_params(&tuned_result);
}
