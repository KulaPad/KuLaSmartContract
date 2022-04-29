# IDO Contract Testing

## Deploy contract
```
near deploy --wasmFile res/kulapad_ido.wasm --accountId ido.testnet --initFunction new --initArgs '{"owner_id": "ido-kulapad.testnet", "staking_contract_id": "token-kulapad.testnet", "test_mode_enabled": true}'

```

### Result
```

```

## Create sample Projects (for testing only)
```
near call dev-1651056695904-82084500074801 create_sample_projects --accountId ido-kulapad.testnet
```

### Result
```
Scheduling a call: dev-1651056695904-82084500074801.create_sample_projects()
Doing account.functionCall()
Transaction Id BK3jCEXAJ2j8XvTKa3Z3x5LPeTsXUbDec86QWKfu56m7
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/BK3jCEXAJ2j8XvTKa3Z3x5LPeTsXUbDec86QWKfu56m7
''
```

## Create a new Project
```
near call dev-1651056695904-82084500074801 create_project '{"project": {
"owner_id": "owner_titan.testnet",
"whitelist_start_date": 1640995200000000000,
"whitelist_end_date": 1641250800000000000,
"sale_start_date": 1641254400000000000,
"sale_end_date": 1641340800000000000,
"token_contract_id": "titan.testnet",
"token_raised_amount": "1000000000000000",
"token_sale_rate_numberator": 1,
"token_sale_rate_denominator": 100,
"fund_contract_id": "usn.testnet",
"whitelist_type": "None",
"sale_type": {
	"Shared": {
		"min_allocation_per_user": 5000000000,
		"max_allocation_per_user": 10000000000
	}
},
"distribution_type": "Unlocked"
}}' --accountId ido-kulapad.testnet
```

### Result
```
Scheduling a call: dev-1651056695904-82084500074801.create_project({"project": {
"owner_id": "owner_titan.testnet",
"whitelist_start_date": 1640995200000000000,
"whitelist_end_date": 1641250800000000000,
"sale_start_date": 1641254400000000000,
"sale_end_date": 1641340800000000000,
"token_contract_id": "titan.testnet",
"token_raised_amount": "1000000000000000",
"token_sale_rate_numberator": 1,
"token_sale_rate_denominator": 100,
"fund_contract_id": "usn.testnet",
"whitelist_type": "None",
"sale_type": {
    "Shared": {
        "min_allocation_per_user": 5000000000,
        "max_allocation_per_user": 5000000000
    }
},
"distribution_type": "Unlocked"
}})
Doing account.functionCall()
Retrying request to broadcast_tx_commit as it has timed out [
  'EwAAAGlkby1rdWxhcGFkLnRlc3RuZXQAdC8UmrdUSHsndkCrjoDJHAG62JhFEO2pFjA1L7YoR+SfwqdfyEwAACAAAABkZXYtMTY1MTA1NjY5NTkwNC04MjA4NDUwMDA3NDgwMWexmEbAzCr310fIdzJ1D1etHkJF9iH99n1/IUy4wNqIAQAAAAIOAAAAY3JlYXRlX3Byb2plY3QVAgAAeyJwcm9qZWN0Ijp7Im93bmVyX2lkIjoib3duZXJfdGl0YW4udGVzdG5ldCIsIndoaXRlbGlzdF9zdGFydF9kYXRlIjoxNjQwOTk1MjAwMDAwMDAwMDAwLCJ3aGl0ZWxpc3RfZW5kX2RhdGUiOjE2NDEyNTA4MDAwMDAwMDAwMDAsInNhbGVfc3RhcnRfZGF0ZSI6MTY0MTI1NDQwMDAwMDAwMDAwMCwic2FsZV9lbmRfZGF0ZSI6MTY0MTM0MDgwMDAwMDAwMDAwMCwidG9rZW5fY29udHJhY3RfaWQiOiJ0aXRhbi50ZXN0bmV0IiwidG9rZW5fcmFpc2VkX2Ftb3VudCI6IjEwMDAwMDAwMDAwMDAwMDAiLCJ0b2tlbl9zYWxlX3JhdGVfbnVtYmVyYXRvciI6MSwidG9rZW5fc2FsZV9yYXRlX2Rlbm9taW5hdG9yIjoxMDAsImZ1bmRfY29udHJhY3RfaWQiOiJ1c24udGVzdG5ldCIsIndoaXRlbGlzdF90eXBlIjoiTm9uZSIsInNhbGVfdHlwZSI6eyJTaGFyZWQiOnsibWluX2FsbG9jYXRpb25fcGVyX3VzZXIiOjUwMDAwMDAwMDAsIm1heF9hbGxvY2F0aW9uX3Blcl91c2VyIjo1MDAwMDAwMDAwfX0sImRpc3RyaWJ1dGlvbl90eXBlIjoiVW5sb2NrZWQifX0A4FfrSBsAAAAAAAAAAAAAAAAAAAAAAAAA8rS00kgxM6DfWCgS8YT2bdF1mSFjzSZnH68sc4Rr2NXNsbPEKALqv6vjtXFB/WsbrFIAWLDCzCZio59EhAKJAg=='
]
Transaction Id 8GGyxFVTEiYyrHcpv8S87FytD1jEE4LnhfazzD3RV4aE
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/8GGyxFVTEiYyrHcpv8S87FytD1jEE4LnhfazzD3RV4aE
2
```

## Get a Project
```
near view dev-1651056695904-82084500074801 get_project '{"project_id": 1}'
```

### Result
```
View call: dev-1651056695904-82084500074801.get_project({"project_id": 1})
{
  id: 1,
  whitelist_start_date: 1640995200000000000,
  whitelist_end_date: 1641250800000000000,
  sale_start_date: 1641254400000000000,
  sale_end_date: 1641340800000000000,
  token_contract_id: 'titan.testnet',
  token_raised_amount: '1000000000000000',
  token_sale_rate: 10,
  fund_contract_id: 'usn.testnet',
  total_fund_committed: '0',
  hard_cap: '10000000000000000',
  whitelist_accounts: 0,
  status: 'Preparation',
  whitelist_type: 'None',
  sale_type: {
    Shared: {
      min_allocation_per_user: 5000000000,
      max_allocation_per_user: 10000000000
    }
  },
  distribution_type: 'Unlocked'
}
```

## Get Project Account Info
```
near call dev-1651056695904-82084500074801 get_project_account_info '{"project_id": 1}' --accountId dev-1651056695904-82084500074801
```

### Result
```
Scheduling a call: dev-1651056695904-82084500074801.get_project_account_info({"project_id": 1})
Doing account.functionCall()
Transaction Id 3xogsUEpj2CNRXtfQU8nWpQM8J8rkqX4y7ZVCA3b7Nem
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/3xogsUEpj2CNRXtfQU8nWpQM8J8rkqX4y7ZVCA3b7Nem
{
  project_id: 1,
  account_id: 'dev-1651056695904-82084500074801',
  is_whitelist: false,
  sale_data: null,
  distribution_data: null
}
```