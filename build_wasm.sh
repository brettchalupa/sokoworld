cargo build --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/sokoworld.wasm web
cp -r assets web/assets
