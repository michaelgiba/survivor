#!/bin/sh

cargo install wasm-pack

wasm-pack build survivor-wasm/ --target web
cargo build --package=survivor-app
cargo build --package=survivor-simulation
