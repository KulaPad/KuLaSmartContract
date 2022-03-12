cd ..
# near deploy --wasmFile res/KuLaPad_ido.wasm --accountId v4-ido-kulapad.testnet --initFunction new --initArgs '{"owner_id": "v4-ido-kulapad.testnet"}'

near deploy --wasmFile res/KuLaPad_ido.wasm --accountId v4-ido-kulapad.testnet

read -p "Press any key to continue..."