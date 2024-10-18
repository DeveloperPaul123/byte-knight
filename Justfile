default: build

build:
    echo "Building the project..."
    cargo build --release

test:
    echo "Running tests..."
    cargo test --

lint:
    cargo clippy

perft depth:
    echo "Running perft..."
    cargo run --release --bin perft -- $(depth)

perft-epd:
    echo "Running EPD perft test suite..."
    cargo run --release --bin perft -- --epd-file data/standard.epd

magics:
    echo "Generating magics..."
    cargo run --release --bin generate_magics

verify-zobrist:
    echo "Verifying Zobrist hash..."
    cargo run --release --bin verify_zobrist
