#!/bin/sh

mode=${1:+"--release"}
out_dir=${1:-debug}

cp -r webclient/static .
cargo build --target wasm32-unknown-unknown -p webapp-webclient $mode
wasm-bindgen --target web --no-typescript --out-dir static/app --out-name webclient target/wasm32-unknown-unknown/${out_dir}/webapp-webclient.wasm