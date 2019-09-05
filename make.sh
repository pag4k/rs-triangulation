#!/bin/bash
cargo +nightly build --release --target wasm32-unknown-unknown
rm src/rs_triangulation.wasm 
cp target/wasm32-unknown-unknown/release/rs_triangulation.wasm src/rs_triangulation.wasm

