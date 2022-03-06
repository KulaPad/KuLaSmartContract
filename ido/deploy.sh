cd ..
#near deploy --wasmFile res/KuLaPad_ido.wasm --accountId ido-kulapad.testnet --initFunction new --initArgs '{"owner_id": "ido-kulapad.testnet"}'

near deploy --wasmFile res/KuLaPad_ido.wasm --accountId ido-kulapad.testnet

read -p "Press any key to continue..."