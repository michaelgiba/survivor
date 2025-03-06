#!/bin/sh

cargo install wasm-pack

wasm-pack build survivor-lib/ --target web
cargo build --package=survivor-app
