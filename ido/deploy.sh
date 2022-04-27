cd ..
# near deploy --wasmFile res/kulapad_ido.wasm --accountId v4-ido-kulapad.testnet --initFunction new --initArgs '{"owner_id": "v4-ido-kulapad.testnet", "staking_contract_id": "token-kulapad.testnet", "test_mode_enabled": true}'

near deploy --wasmFile res/kulapad_ido.wasm --accountId v4-ido-kulapad.testnet

read -p "Press any key to continue..."