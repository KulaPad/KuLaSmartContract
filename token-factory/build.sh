#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/KuLaPad_token.wasm ../res 

read -p "Press any key to continue..."