#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

cp target/wasm32-unknown-unknown/release/*.wasm ../res/kulapad_staking.wasm

cp target/wasm32-unknown-unknown/release/*.wasm res/kulapad_staking.wasm

read -p "Press any key to continue..."