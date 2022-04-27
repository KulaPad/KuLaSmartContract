cd ..

near dev-deploy --wasmFile res/kulapad_ido.wasm --initFunction new --initArgs '{"owner_id": "ido-kulapad.testnet", "staking_contract_id": "token-kulapad.testnet", "test_mode_enabled": true}'

read -p "Press any key to continue..."