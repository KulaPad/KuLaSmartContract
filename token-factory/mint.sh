source neardev/dev-account.env

# near call $CONTRACT_NAME storage_deposit '' --accountId $CONTRACT_NAME --amount 0.0125
# near call $CONTRACT_NAME new '{"owner_id": "'$CONTRACT_NAME'", "total_supply": "10000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "KulaPad Token", "symbol": "KULA", "decimals": 8 }}' --accountId $CONTRACT_NAME

# [claim_testnet_token]
# near call $CONTRACT_NAME storage_deposit --accountId astral.testnet --deposit 0.0125
# near call $CONTRACT_NAME claim_testnet_token '' --accountId astral.testnet
