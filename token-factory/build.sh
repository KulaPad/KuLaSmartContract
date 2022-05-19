#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/kulapad_token.wasm ../res 

cp target/wasm32-unknown-unknown/release/kulapad_token.wasm ../staking-pool/res 

read -p "Press any key to continue..."