set -e
cargo fmt --check
cargo clippy
cargo test
cargo check
cargo check --release
