set -e
cargo fmt --check
cargo clippy
cargo test
cargo check
cargo check --release
cargo check --target wasm32-unknown-unknown
