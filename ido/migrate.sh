cd ..

near deploy --wasmFile res/KuLaPad_ido.wasm --accountId ido-kulapad.testnet --initFunction migrate --initArgs '{"owner_id": "ido-kulapad.testnet"}'

read -p "Press any key to continue..."