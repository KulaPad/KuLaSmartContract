/*!
Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='500' height='500' style='shape-rendering:geometricPrecision;text-rendering:geometricPrecision;image-rendering:optimizeQuality;fill-rule:evenodd;clip-rule:evenodd'%3E%3Cpath style='opacity:1' d='M0 0h500v500H0V0Z'/%3E%3Cpath style='opacity:1' fill='%23fede2a' d='M156 84v8a401 401 0 0 0 1 40 71 71 0 0 0 9-11 71 71 0 0 0 8-12 391 391 0 0 0 22-31 7 7 0 0 1 4-3 342 342 0 0 1 37 0 4 4 0 0 1 3 3 22 22 0 0 1-3 5 36 36 0 0 0-6 8 252 252 0 0 0-26 33 4140 4140 0 0 0 45 96 4 4 0 0 1-2 2 840 840 0 0 1-41 1 9 9 0 0 1-7-3 76899 76899 0 0 1-14-33l-5-12a112 112 0 0 1-5-11l-2-1a165 165 0 0 0-17 19 382 382 0 0 0-2 38 4 4 0 0 1-1 2 164 164 0 0 1-25 1h-9l-9-2V77l2-2a420 420 0 0 1 41 0 11 11 0 0 1 2 9Z'/%3E%3Cpath style='opacity:1' fill='%23fddd2a' d='M308 84v89c1 8 4 14 12 18a11 11 0 0 0 7 2c12 1 20-4 24-15a2721 2721 0 0 0 2-101l2-2a289 289 0 0 1 34 0l1 2-1 104c-3 18-13 31-29 38a120 120 0 0 1-54 4 72 72 0 0 1-21-7 57 57 0 0 1-22-43V77l2-2h42a20 20 0 0 1 1 9Z'/%3E%3Cpath style='opacity:1' fill='%237e6e15' d='M156 84c6-2 9 1 9 8h-9v-8Z'/%3E%3Cpath style='opacity:1' fill='%237d6d15' d='m237 83 11 2c1 2 0 4-1 6l-2 2-14-2a36 36 0 0 1 6-8Z'/%3E%3Cpath style='opacity:1' fill='%237f6f15' d='M308 84c7-2 10 1 9 8h-9v-8Z'/%3E%3Cpath style='opacity:1' fill='%23a6911c' d='M156 92h9c-1 9 0 19 1 29a71 71 0 0 1-9 11 401 401 0 0 1-1-40Z'/%3E%3Cpath style='opacity:1' fill='%234d430c' d='M165 92c3-1 6 0 8 2l1 15a71 71 0 0 1-8 12c-1-10-2-20-1-29Z'/%3E%3Cpath style='opacity:1' fill='%23a48f1b' d='m231 91 14 2a1388 1388 0 0 1-29 35 12 12 0 0 0-2 5 4140 4140 0 0 0 45 96 4 4 0 0 1-2 2 840 840 0 0 1-41 1c-6-1-9-4-9-9l41-1a4 4 0 0 0 2-2 4140 4140 0 0 1-45-96 252 252 0 0 1 26-33Z'/%3E%3Cpath style='opacity:1' fill='%234c420c' d='m247 91 9 2a5 5 0 0 1 0 5 1069 1069 0 0 0-34 43 11581 11581 0 0 0 44 92v6a334 334 0 0 1-44 0 12 12 0 0 1-6-7l41-1a4 4 0 0 0 2-2 4140 4140 0 0 1-45-96 12 12 0 0 1 2-5 1388 1388 0 0 0 29-35l2-2Z'/%3E%3Cpath style='opacity:1' fill='%23a7921c' d='M308 92h9v90a32 32 0 0 0 3 9c-8-4-11-10-12-18V92Z'/%3E%3Cpath style='opacity:1' fill='%234e440d' d='M317 92c3-1 6 0 8 2a5230 5230 0 0 0 2 99l-7-2a32 32 0 0 1-3-9V92Z'/%3E%3Cpath style='opacity:1' fill='%23a6911c' d='m390 77 1 7v8h8v90c-1 22-11 37-30 46a115 115 0 0 1-62 2 99 99 0 0 1-12-5 32 32 0 0 1-10-9 72 72 0 0 0 21 7c18 3 36 1 54-4 16-7 26-20 29-38l1-104Z'/%3E%3Cpath style='opacity:1' fill='%23766813' d='M391 84c6-2 9 1 9 8a2026 2026 0 0 1-1 90V92h-8v-8Z'/%3E%3Cpath style='opacity:1' fill='%234c420c' d='M400 92a15 15 0 0 1 7 1 1667 1667 0 0 1 0 100c-2 25-16 40-40 46a110 110 0 0 1-54-2 38 38 0 0 1-18-12 99 99 0 0 0 12 5c20 5 41 4 62-2 19-9 29-24 30-46a2026 2026 0 0 0 1-90Z'/%3E%3Cpath style='opacity:1' fill='%23a6911b' d='M181 175a4 4 0 0 0-4 2l-12 16a724 724 0 0 1-1 37 4 4 0 0 1-2 1l-33 1v-9l25-1a4 4 0 0 0 1-2 382 382 0 0 1 2-38 165 165 0 0 1 17-19l2 1a112 112 0 0 0 5 11Z'/%3E%3Cpath style='opacity:1' fill='%234c420c' d='m181 175 5 12a78 78 0 0 0-12 14l-1 36-1 1a216 216 0 0 1-41 1 9 9 0 0 1-2-7l33-1a4 4 0 0 0 2-1 724 724 0 0 0 1-37l12-16a4 4 0 0 1 4-2Z'/%3E%3Cpath style='opacity:1' fill='%23807015' d='M120 223h9v9c-8 1-11-2-9-9Z'/%3E%3Cpath style='opacity:1' fill='%23fcdc2a' d='M158 256v108l9 1h58l2 10-1 22-1 1-96 1h-9c-4 0-7-1-9-3V249l3-3h42l2 10Z'/%3E%3Cpath style='opacity:1' fill='%23fddd2a' d='M344 256a5628 5628 0 0 1 48 138l-1 4-34 1h-6l-6-1a59 59 0 0 1-3-10 71 71 0 0 1-3-9 56 56 0 0 0-4-10 552 552 0 0 0-47 0 339 339 0 0 0-11 28 9 9 0 0 1-2 1 113 113 0 0 1-15 1h-15a7 7 0 0 1-7-2c0-5 1-9 3-13a10865 10865 0 0 1 48-135l3-3h45c4 2 6 5 7 10Z'/%3E%3Cpath style='opacity:1' fill='%237d6d15' d='M158 256c7-2 10 1 9 9h-9v-9Z'/%3E%3Cpath style='opacity:1' fill='%23a7911c' d='M158 265h9v100l-9-1v-99Z'/%3E%3Cpath style='opacity:1' fill='%234d430d' d='M167 265c3-1 6 0 8 2l1 98h-9V265Z'/%3E%3Cpath style='opacity:1' fill='%23a38e1b' d='M344 256a12 12 0 0 1 9 9 8548 8548 0 0 1 48 139l-1 4-39 1h-1a43 43 0 0 0-3-10l34-1 1-4a5628 5628 0 0 0-48-138Z'/%3E%3Cpath style='opacity:1' fill='%234c420c' d='M353 265c4 1 7 3 8 8a6951 6951 0 0 1 49 140l-1 4a484 484 0 0 1-44 0 11 11 0 0 1-4-8l39-1 1-4a8548 8548 0 0 0-48-139Z'/%3E%3Cpath style='opacity:1' fill='%23a7921c' d='M318 310a129 129 0 0 0-9 24h15-27a308 308 0 0 1 13-39c0-2 1-2 2 0a134 134 0 0 0 6 15Z'/%3E%3Cpath style='opacity:1' fill='%234f450d' d='M318 310a285 285 0 0 0 6 18 9 9 0 0 0-3 6h-12a129 129 0 0 1 9-24Z'/%3E%3Cpath style='opacity:1' fill='%23131003' d='M324 328v6h-3a9 9 0 0 1 3-6Z'/%3E%3Cpath style='opacity:1' fill='%23a7921c' d='M339 379h-42a703 703 0 0 0-11 28 148 148 0 0 1-29 2 43 43 0 0 0 3-10 113 113 0 0 0 15-1 9 9 0 0 0 2-1 339 339 0 0 1 11-28 552 552 0 0 1 47 0 56 56 0 0 1 4 10Z'/%3E%3Cpath style='opacity:1' fill='%237c6c15' d='M228 375c6-1 9 2 8 9h-8v-9Z'/%3E%3Cpath style='opacity:1' fill='%234e440d' d='M339 379a71 71 0 0 0 3 9h-36a778 778 0 0 1-12 29h-36c-3-3-3-5-1-8l29-2a703 703 0 0 1 11-28h42Z'/%3E%3Cpath style='opacity:1' fill='%23a5901b' d='M227 375h1v9h8l-1 23-1 1-104 1h-1v-10l96-1 1-1 1-22Z'/%3E%3Cpath style='opacity:1' fill='%234d430d' d='M236 384h5a31 31 0 0 0-3 13 7 7 0 0 0 7 2 145 145 0 0 1-1 17l-1 1H132a11 11 0 0 1-2-8l104-1 1-1 1-23Z'/%3E%3Cpath style='opacity:1' fill='%23796a14' d='M120 399h9v10c-8 0-11-3-9-10Z'/%3E%3Cpath style='opacity:1' fill='%237d6e15' d='M248 399h12a43 43 0 0 1-3 10c-9 1-12-2-9-10Z'/%3E%3Cpath style='opacity:1' fill='%237c6c15' d='M351 399h6a43 43 0 0 1 3 10l-6-1a151 151 0 0 1-3-9Z'/%3E%3C/svg%3E";

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Example NEAR fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: ValidAccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(owner_id.as_ref());
        this.token.internal_deposit(owner_id.as_ref(), total_supply.into());
        this
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }

    // Claim testnet tokens
    pub fn claim_testnet_token(&mut self) {
        let sender_id = env::current_account_id();
        let receiver_id = env::signer_account_id();
        assert_ne!(
            sender_id, receiver_id,
            "Sender and receiver should be different"
        );

        // Check if receiver end with ".testnet"
        if !receiver_id.ends_with(".testnet") {
            env::panic(b"Receiver should be testnet account");
        }

        let transfer_amount: Balance = 20000000000;
        self.token
            .internal_transfer(&sender_id, &receiver_id, transfer_amount.into(), None);
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, Balance};

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}
