#!/bin/sh

cargo build --package=survivor-lib --target wasm32-unknown-unknown
cargo build --package=survivor-app
