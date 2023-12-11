#!/bin/bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./files/out/ --target web ./target/wasm32-unknown-unknown/release/bevy-jam.wasm
cp -r files /home/jake/Desktop/