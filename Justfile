set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]
default: build

build:
    echo "Building the project..."
    cargo build --release

test:
    echo "Running tests..."
    cargo test --release --

lint:
    cargo clippy --all --all-features --tests -- -D warnings

search-bench:
    echo "Running search benchmark..."
    cargo run --release --bin byte-knight -- bench
    
perft depth:
    echo "Running perft..."
    cargo run --release --bin perft -- -d {{depth}}

perft-epd:
    echo "Running EPD perft test suite..."
    cargo run --release --bin perft -- --epd-file data/standard.epd

perft-bench:
    echo "Running perft benchmark..."
    cargo run --release --bin perft-bench -- -e data/standard.epd

magics:
    echo "Generating magics..."
    cargo run --release --bin generate_magics

verify-zobrist:
    echo "Verifying Zobrist hash..."
    cargo run --release --bin verify_zobrist
