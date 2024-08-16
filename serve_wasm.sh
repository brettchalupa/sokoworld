set -e

# requires Ruby to be installed
# TODO: use a simple Rust web server

./build_wasm.sh
cd web
PORT=5577 ruby -run -e httpd . -p ${PORT:=5577}
