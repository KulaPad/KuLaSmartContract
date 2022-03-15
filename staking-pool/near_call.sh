MY_ACCOUNT="'$MY_ACCOUNT'"

near call staking-kulapad.testnet new_default_config '{"owner_id": "staking-kulapad.testnet","ft_contract_id": "token-kulapad.testnet"}' --accountId staking-kulapad.testnet

near view staking-kulapad.testnet get_account_reward '{"account_id": "'$MY_ACCOUNT'"}' --accountId $MY_ACCOUNT


near call staking-kulapad.testnet storage_deposit '{"account_id": "'$MY_ACCOUNT'"}' --accountId $MY_ACCOUNT --depositYocto 1770000000000000000000

near call token-kulapad.testnet ft_transfer_call '{"receiver_id": "staking-kulapad.testnet", "amount": "10000000000", "msg": ""}' --accountId $MY_ACCOUNT

near call staking-kulapad.testnet storage_deposit --accountId staking-kulapad.testnet --deposit 0.0125

near call staking-kulapad.testnet storage_balance_of '{"account_id": "'$MY_ACCOUNT'"}' --accountId $MY_ACCOUNT

near call token-kulapad.testnet storage_deposit --accountId staking-kulapad.testnet --deposit 0.0125

near deploy --accountId staking-kulapad.testnet --wasmFile res/staking-contract.wasm

near call token-kulapad.testnet ft_transfer_call '{"receiver_id": "staking-kulapad.testnet", "amount": "1000000000", "msg": "Stake KULA"}' --accountId $MY_ACCOUNT --depositYocto 1 --gas 50000000000000


near call staking-kulapad.testnet get_account_info '{"account_id": "'$MY_ACCOUNT'"}' --accountId $MY_ACCOUNT
near call staking-kulapad.testnet get_total_pending_reward --accountId staking-kulapad.testnet
near call staking-kulapad.testnet lock '{"amount": "500000000", "locked_time": 864000000000000}' --accountId   $MY_ACCOUNT


near call staking-kulapad.testnet unlock '{"amount": "500000000", "locked_time": 864000000000000}' --accountId   $MY_ACCOUNT
