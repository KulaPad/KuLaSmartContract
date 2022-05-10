near call dev-1646718348240-77300820554298 new_default_config '{"owner_id": "dev-1646718348240-77300820554298","ft_contract_id": "token-kulapad.testnet"}' --accountId dev-1646718348240-77300820554298

near call dev-1646718348240-77300820554298 new_default_config '{"owner_id": "dev-1646718348240-77300820554298","ft_contract_id": "token-kulapad.testnet"}' --accountId dev-1646718348240-77300820554298


near view dev-1646718348240-77300820554298 get_account_reward '{"account_id": "hoangtheanh.testnet"}' --accountId hoangtheanh.testnet


near call dev-1646718348240-77300820554298 storage_deposit '{"account_id": "hoangtheanh.testnet"}' --accountId hoangtheanh.testnet --depositYocto 1770000000000000000000

near call token-kulapad.testnet ft_transfer_call '{"receiver_id": "dev-1646718348240-77300820554298", "amount": "10000000000", "msg": ""}' --accountId hoangtheanh.testnet

near call dev-1646718348240-77300820554298 storage_deposit --accountId staking-kulapad.testnet --deposit 0.0125

near call dev-1646718348240-77300820554298 storage_balance_of '{"account_id": "hoangtheanh.testnet"}' --accountId hoangtheanh.testnet

near call token-kulapad.testnet storage_deposit --accountId dev-1646718348240-77300820554298 --deposit 0.0125

near deploy --accountId dev-1646718348240-77300820554298 --wasmFile res/kulapad-staking.wasm

near call token-kulapad.testnet ft_transfer_call '{"receiver_id": "dev-1646718348240-77300820554298", "amount": "1000000000", "msg": "2222221231232"}' --accountId hoangtheanh.testnet --depositYocto 1 --gas 50000000000000


near call dev-1646718348240-77300820554298 get_account_info '{"account_id": "hoangtheanh.testnet"}' --accountId hoangtheanh.testnet
near call dev-1646718348240-77300820554298 get_total_pending_reward --accountId dev-1646718348240-77300820554298
near call dev-1646718348240-77300820554298 lock '{"amount": "500000000", "locked_time": 864000000000000}' --accountId   hoangtheanh.testnet


near call dev-1646718348240-77300820554298 unlock '{"amount": "500000000", "locked_time": 864000000000000}' --accountId   hoangtheanh.testnet


