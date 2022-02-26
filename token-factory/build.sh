#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/token_factory.wasm token_factory.wasm 

read -p "Press any key to continue..."