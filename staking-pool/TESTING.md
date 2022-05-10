# Staking KULA-FT Contract 

## Information
- FT contract id: ``` token-kulapad.testnet ```
- Staking contract id: ``` staking.kulapad-contract.testnet ```
- Staking contract owner id: ``` kulapad-contract.testnet ```
- User account id: ``` alice.kulapad-contract.testnet ```

## Deployment

### Deploy & initialize contract
```
near deploy --wasmFile .\res\kulapad_staking.wasm --accountId staking.kulapad-contract.testnet --initFunction new_default_config --initArgs '{""owner_id"": ""kulapad-contract.testnet"", ""ft_contract_id"": ""token-kulapad.testnet""}'
```

#### Result
```
Transaction Id FvGQ5vs7yKNYUidAPqPCxcbkoyuyjRjEn1fM182Mdq18
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/FvGQ5vs7yKNYUidAPqPCxcbkoyuyjRjEn1fM182Mdq18
Done deploying and initializing staking.kulapad-contract.testnet
```

## Owner

### Set tier config
```
near call staking.kulapad-contract.testnet set_tier_config '{""tier"": ""Tier1"", ""config"": {""min_point"": 10000000000}}' --accountId kulapad-contract.testnet
```

#### Result
```
Scheduling a call: staking.kulapad-contract.testnet.set_tier_config({"tier": "Tier1", "config": {"min_point": 10000000000}})
Doing account.functionCall()
Transaction Id 73STFqyrJPCcP2YAZoUvVHnMW8mKSV7UNEpxACr8o1mZ
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/73STFqyrJPCcP2YAZoUvVHnMW8mKSV7UNEpxACr8o1mZ
''
```

### Reset log
```
near call staking.kulapad-contract.testnet reset_lock '{""account_id"": ""alice.kulapad-contract.testnet""}' --accountId kulapad-contract.testnet
```

#### Result
```
Scheduling a call: staking.kulapad-contract.testnet.reset_lock({"account_id": "alice.kulapad-contract.testnet"})
Doing account.functionCall()
Transaction Id 41FNpMXbXnQuzzHGpqkhgGpWaj3WhJB5boqbGGbgKfWm
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/41FNpMXbXnQuzzHGpqkhgGpWaj3WhJB5boqbGGbgKfWm
''
```

## Storage management
### Deposit storage fee to ```FT token``` for ```Staking contract```
```
near call token-kulapad.testnet storage_deposit --accountId staking.kulapad-contract.testnet --deposit 0.0125
```

#### Result
```
Scheduling a call: token-kulapad.testnet.storage_deposit() with attached 0.0125 NEAR
Doing account.functionCall()
Transaction Id Aj2n4BidZXduvmR613nXKMH12RJejh9NuJs6cyY1wtgu
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/Aj2n4BidZXduvmR613nXKMH12RJejh9NuJs6cyY1wtgu
{ total: '1250000000000000000000', available: '0' }
```

### Deposit storage fee to ```FT token``` for ```Alice```
```
near call token-kulapad.testnet storage_deposit --accountId alice.kulapad-contract.testnet --deposit 0.0125
```

### Get storage balance of ```Alice``` from ```FT token```
```
near view token-kulapad.testnet storage_balance_of '{""account_id"": ""alice.kulapad-contract.testnet""}'
```

#### Result
```
View call: token-kulapad.testnet.storage_balance_of({"account_id": "alice.kulapad-contract.testnet"})
{ total: '1250000000000000000000', available: '0' }
```

### Deposit storage fee to ```Staking contract``` for ```Alice```
```
near call staking.kulapad-contract.testnet storage_deposit --accountId alice.kulapad-contract.testnet --deposit 0.0125
```

#### Result
```
Scheduling a call: staking.kulapad-contract.testnet.storage_deposit() with attached 0.0125 NEAR
Doing account.functionCall()
Transaction Id BUaYP6V8j2ct3hvPzsHu6PwvmVkJ7Dt7mQ9DCMkhuXLC
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/BUaYP6V8j2ct3hvPzsHu6PwvmVkJ7Dt7mQ9DCMkhuXLC
''
```

### Get storage balance of ```Alice``` from ```Staking contract```
```
near view staking.kulapad-contract.testnet storage_balance_of '{""account_id"": ""alice.kulapad-contract.testnet""}'
```

#### Result
```
View call: staking.kulapad-contract.testnet.storage_balance_of({"account_id": "alice.kulapad-contract.testnet"})
'1'
```

## FT contract
### Claim token from ```FT contract```

```
near call token-kulapad.testnet claim_testnet_token --accountId alice.kulapad-contract.testnet
```

#### Result
```
Scheduling a call: token-kulapad.testnet.claim_testnet_token()
Doing account.functionCall()
Receipt: 813CZtSr8GTuUt227BDE5bKe6W25jAJhS7f69FuJ4dLs
        Log [token-kulapad.testnet]: Transfer 20000000000 from token-kulapad.testnet to alice.kulapad-contract.testnet
Transaction Id Bw6F8aSuWu6LQ36sJmkxfsGYrUbyBiSTinU4ddXzkVDh
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/Bw6F8aSuWu6LQ36sJmkxfsGYrUbyBiSTinU4ddXzkVDh
''
```

## Account info
### Get account info
```
near view staking.kulapad-contract.testnet get_account_info '{""account_id"": ""alice.kulapad-contract.testnet""}'
```

#### Result
```
View call: staking.kulapad-contract.testnet.get_account_info({"account_id": "alice.kulapad-contract.testnet"})
{
  account_id: 'alice.kulapad-contract.testnet',
  locked_balance: '0',
  unlocked_timestamp: 0,
  staked_balance: '0',
  unstaked_balance: '0',
  reward: '0',
  can_withdraw: true,
  start_unstake_timestamp: 0,
  unstake_available_epoch: 0,
  current_epoch: 1091,
  tier: 'Tier0',
  point: '0'
}
```

## Staking
### Stake by calling to ```FT contract```
```
near call token-kulapad.testnet ft_transfer_call '{"receiver_id": "staking.kulapad-contract.testnet", "amount": "10000000000", "msg": ""}' --accountId alice.kulapad-contract.testnet --depositYocto 1 --gas 50000000000000
```

#### Result
```
Scheduling a call: token-kulapad.testnet.ft_transfer_call({"receiver_id": "staking.kulapad-contract.testnet", "amount": "10000000000", "msg": ""}) with attached 0.000000000000000000000001 NEAR
Doing account.functionCall()
Receipts: 5QYJvq36NH1vPg4CDYtTkKTkfYQWS3aWPcVp1u6dvH8X, CaHrgdzAKnBdX48Vs7G7kz7ssNXF4tTo2u45BGXX4fDE, 4HSH8rZPWJhGwNwxXAs7TRW4pHoHbJc1tbeFcfaKUTSo
        Log [token-kulapad.testnet]: Transfer 10000000000 from alice.kulapad-contract.testnet to staking.kulapad-contract.testnet
Transaction Id t13XdD9e5mZHnNj2fUvgFmvH5jisSt34Etn38qziQUB
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/t13XdD9e5mZHnNj2fUvgFmvH5jisSt34Etn38qziQUB
'10000000000'
```

#### Result for ```Account Info```
```
View call: staking.kulapad-contract.testnet.get_account_info({"account_id": "alice.kulapad-contract.testnet"})
{
  account_id: 'alice.kulapad-contract.testnet',
  locked_balance: '0',
  unlocked_timestamp: 0,
  staked_balance: '10000000000',
  unstaked_balance: '0',
  reward: '6649',
  can_withdraw: true,
  start_unstake_timestamp: 0,
  unstake_available_epoch: 0,
  current_epoch: 1091,
  tier: 'Tier0',
  point: '0'
}
```

## Staking & lock by calling to ```FT contract```
```
near call token-kulapad.testnet ft_transfer_call '{""receiver_id"": ""staking.kulapad-contract.testnet"", ""amount"": ""1000000000"", ""msg"": ""lock:360""}' --accountId alice.kulapad-contract.testnet --depositYocto 1 --gas 50000000000000
```

#### Result
```
Scheduling a call: token-kulapad.testnet.ft_transfer_call({"receiver_id": "staking.kulapad-contract.testnet", "amount": "1000000000", "msg": "lock:360"}) with attached 0.000000000000000000000001 NEAR
Doing account.functionCall()
Receipts: DAHVwWJTaRSAQYinwtjrhiQvVrb5i5xwyvRKvy5vUcRe, 6A5wN1UhUANRW65mKCNEe2haSLsXJzwUqY7LUsnwGWrd, 9wvx4b8kxhA4pMxLFVz1dnmkAQDciWVC87LaW3h3aXL6
        Log [token-kulapad.testnet]: Transfer 1000000000 from alice.kulapad-contract.testnet to staking.kulapad-contract.testnet
Receipt: BYdTCKymjfoh7msRPnamS73ADY8aAjXCYGMkmrTn2j15
        Log [token-kulapad.testnet]: Lock amount of 1000000000 KULA for account alice.kulapad-contract.testnet in 360 day(s).
Transaction Id Emj1hTQFTrvkpG5USXf9geRL2Xpdp1xvZ8FqkC8E1VGD
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/Emj1hTQFTrvkpG5USXf9geRL2Xpdp1xvZ8FqkC8E1VGD
'1000000000'
```

#### Result for ```Account Info```
```
View call: staking.kulapad-contract.testnet.get_account_info({"account_id": "alice.kulapad-contract.testnet"})
{
  account_id: 'alice.kulapad-contract.testnet',
  locked_balance: '1000000000',
  unlocked_timestamp: 4697781694644183000,
  staked_balance: '11000000000',
  unstaked_balance: '0',
  reward: '20655',
  can_withdraw: true,
  start_unstake_timestamp: 0,
  unstake_available_epoch: 0,
  current_epoch: 1091,
  tier: 'Tier0',
  point: '1000000000'
}
```

## Lock (```if locked_balanced = 0```)
```
near call staking.kulapad-contract.testnet lock '{""amount"": ""5000000000"", ""locked_days"": 36}' --accountId alice.kulapad-contract.testnet
```

#### Result
```
Scheduling a call: staking.kulapad-contract.testnet.lock({"amount": "5000000000", "locked_days": 36})
Doing account.functionCall()
Transaction Id HnMvtNdknW5wahxAiQ4exUhXnPBqnaFjEDtUHWMfzj2Z
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/HnMvtNdknW5wahxAiQ4exUhXnPBqnaFjEDtUHWMfzj2Z
''
```

#### Result for ```Account Info```
```
View call: staking.kulapad-contract.testnet.get_account_info({"account_id": "alice.kulapad-contract.testnet"})
{
  account_id: 'alice.kulapad-contract.testnet',
  locked_balance: '5000000000',
  unlocked_timestamp: 1655240612985729000,
  staked_balance: '11000000000',
  unstaked_balance: '0',
  reward: '981129',
  can_withdraw: true,
  start_unstake_timestamp: 0,
  unstake_available_epoch: 0,
  current_epoch: 1092,
  tier: 'Tier0',
  point: '500000000'
}
```

## Extend locked time
```
near call staking.kulapad-contract.testnet lock '{""amount"": ""0"", ""locked_days"": 1}' --accountId alice.kulapad-contract.testnet
```

#### Result
```
Scheduling a call: staking.kulapad-contract.testnet.lock({"amount": "0", "locked_days": 1})
Doing account.functionCall()
Transaction Id 9rEd6ejByQH2NjoDVv2RjfZrq9MSoUpD1EHGPnW2ZEZB
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/9rEd6ejByQH2NjoDVv2RjfZrq9MSoUpD1EHGPnW2ZEZB
''
```

#### Result for ```Account Info```
```
View call: staking.kulapad-contract.testnet.get_account_info({"account_id": "alice.kulapad-contract.testnet"})
{
  account_id: 'alice.kulapad-contract.testnet',
  locked_balance: '5000000000',
  locked_days: 37,
  unlocked_timestamp: 1655325212985729000,
  staked_balance: '11000000000',
  unstaked_balance: '0',
  reward: '1131429',
  can_withdraw: true,
  start_unstake_timestamp: 0,
  unstake_available_epoch: 0,
  current_epoch: 1092,
  tier: 'Tier0',
  point: '513888888'
}
```

## Increase locked amount
```
near call staking.kulapad-contract.testnet lock '{""amount"": ""5000000000"", ""locked_days"": 0}' --accountId alice.kulapad-contract.testnet
```

#### Result
```
Scheduling a call: staking.kulapad-contract.testnet.lock({"amount": "5000000000", "locked_days": 0})
Doing account.functionCall()
Transaction Id 6X2cBpaVBmSuYCkwfcTAKSCNrMmF5Qs4BdJqodjVw1xT
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/6X2cBpaVBmSuYCkwfcTAKSCNrMmF5Qs4BdJqodjVw1xT
''
```

#### Result for ```Account Info```
```
View call: staking.kulapad-contract.testnet.get_account_info({"account_id": "alice.kulapad-contract.testnet"})
{
  account_id: 'alice.kulapad-contract.testnet',
  locked_balance: '6000000000',
  unlocked_timestamp: 4697781694644183000,
  staked_balance: '11000000000',
  unstaked_balance: '0',
  reward: '383075',
  can_withdraw: true,
  start_unstake_timestamp: 0,
  unstake_available_epoch: 0,
  current_epoch: 1091,
  tier: 'Tier4',
  point: '5999992042'
}
```
