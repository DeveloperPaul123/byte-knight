set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default: (build)

build config="debug":
    echo "Building the project..."
    cargo build --workspace --all-features {{ if config=="release" {"--release"} else {""} }}

test config="debug":
    echo "Running tests..."
    cargo test --workspace --all-features {{ if config=="release" {"--release"} else {""} }} -- --include-ignored

export RUSTFLAGS:="-Cinstrument-coverage"
export LLVM_PROFILE_FILE:="./target/coverage/byte_knight-%p-%m.profraw"
coverage: (build "debug")
    echo "Running tests with coverage..."
    mkdir -p target/coverage
    cargo test --workspace -- --skip "perft"
    grcov target/coverage engine/target/coverage chess/target/coverage -s . \
        --binary-path ./target/debug/ --output-types lcov -o ./target/coverage/byte-knight.lcov \
        --branch --keep-only "src/*" --keep-only "engine/*" --keep-only "chess/*" \
        --ignore "src/bin/byte-knight/*" --ignore "chess/src/perft*"

purge-coverage:
    echo "Purging coverage data..."
    rm -rf *.profraw
    rm -rf target/coverage
    rm -rf chess/target
    rm -rf engine/target
    rm -rf chess/*.profraw
    rm -rf engine/*.profraw

lint:
    cargo clippy --all --all-features --tests -- -D warnings

search-bench:
    echo "Running search benchmark..."
    cargo rustc --release --bin byte-knight -- -C target-cpu=native
    ./target/release/byte-knight bench

perft depth:
    echo "Running perft..."
    cargo run --release --bin perft -- -d {{ depth }}

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

release target:
    echo "Building release binaries..."
    cargo rustc --release --bin byte-knight --target={{ target }}

cache-main: (build "release")
    echo "Caching binary for testing..."
    cp target/release/byte-knight ./bk-main

compare-to-main engine1: (build "release")
    echo "Comparing {{ engine1 }} to bk-main"
    fastchess -engine cmd="{{ engine1 }}" name="dev" -engine cmd="./bk-main" name="bk-main" -openings file="./data/Pohl.epd" format=epd order=random -each tc=10+0.1 -rounds 200 -repeat -concurrency 8 -sprt elo0=0 elo1=5 alpha=0.05 beta=0.1 model=normalized -output format=cutechess
