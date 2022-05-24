mod core_impl;
mod enumeration;
mod internal;
mod modules;
mod util;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BlockHeight, BorshStorageKey, EpochHeight,
    PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
};
use std::collections::HashMap;

pub use crate::enumeration::PoolInfo;
pub use crate::modules::account::{Account, AccountJson, UpgradableAccount};
use crate::modules::tier::{Tier, TierConfig, TierConfigsType};
use crate::util::*;

pub type DayType = u32;

pub const NO_DEPOSIT: Balance = 0;
pub const DEPOSIT_ONE_YOCTOR: Balance = 1;
pub const NUM_EPOCHS_TO_UNLOCK: EpochHeight = 1;
pub const ONE_DAY_IN_NANOSECOND: u64 = 84600_000_000_000;
pub const POINT_100_PERCENT_IN_NANOSECOND: u64 = ONE_DAY_IN_NANOSECOND * 360;
pub const DEFAULT_TOKEN_DECIMAL: u8 = 8;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    // Percent reward per 1 block
    pub reward_numerator: u64,
    pub reward_denumerator: u64,
    pub total_apr: u64,

    // The config for each user Tier
    pub tier_configs: TierConfigsType,
    pub min_locking_days: DayType,
    pub max_locking_days: DayType,
}

impl Config {
    fn new_default_config() -> Self {
        // By default APR 15%
        Self {
            reward_numerator: 715,
            reward_denumerator: 100000000000,
            total_apr: 15,
            tier_configs: TierConfig::get_default_tier_configs(),
            min_locking_days: 1,
            max_locking_days: 360,
        }
    }

    fn new(
        reward_numerator: u64,
        reward_denumerator: u64,
        total_apr: u64,
        tier_configs: TierConfigsType,
        min_locking_days: DayType,
        max_locking_days: DayType,
    ) -> Self {
        Self {
            reward_numerator,
            reward_denumerator,
            total_apr,
            tier_configs,
            min_locking_days,
            max_locking_days
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new_default_config()
    }
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct StakingContract {
    pub owner_id: AccountId, // Owner of contract
    pub ft_contract_id: AccountId,
    pub config: Config,               // Config reward and apr for contract
    pub total_stake_balance: Balance, // Total token balance lock in contract
    pub total_paid_reward_balance: Balance,
    pub total_staker: Balance, // TODO: integer
    pub pre_reward: Balance,   // Pre reward before change total balance
    pub last_block_balance_change: BlockHeight,
    pub accounts: LookupMap<AccountId, UpgradableAccount>, // List staking user
    pub paused: bool,                                      // Pause staking pool with limit reward,
    pub paused_in_block: BlockHeight,
}

#[near_bindgen]
impl StakingContract {
    #[init]
    pub fn new_default_config(owner_id: AccountId, ft_contract_id: AccountId) -> Self {
        Self::new(owner_id, ft_contract_id, Config::default())
    }

    #[init]
    pub fn new(owner_id: AccountId, ft_contract_id: AccountId, config: Config) -> Self {
        StakingContract {
            owner_id,
            ft_contract_id,
            config,
            total_stake_balance: 0,
            total_paid_reward_balance: 0,
            total_staker: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_index(),
            accounts: LookupMap::new(StorageKey::AccountKey),
            paused: false,
            paused_in_block: 0,
        }
    }

    pub fn get_total_pending_reward(&self) -> U128 {
        self.assert_owner();
        U128(self.pre_reward + self.internal_calculate_global_reward())
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
        assert_at_least_one_yocto();
        let account = account_id.unwrap_or_else(|| env::predecessor_account_id());

        let account_stake: Option<UpgradableAccount> = self.accounts.get(&account);
        if account_stake.is_some() {
            refund_deposit(0);
        } else {
            let before_storage_usage = env::storage_usage();
            self.internal_create_account(account.clone());
            let after_storage_usage = env::storage_usage();

            refund_deposit(after_storage_usage - before_storage_usage);
        }
    }

    // View func get storage balance, return 0 if account need deposit to interact
    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
        let account: Option<UpgradableAccount> = self.accounts.get(&account_id);
        if account.is_some() {
            U128(1)
        } else {
            U128(0)
        }
    }

    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only contract owner can be access."
        );
    }

    // Owner

    /// create or update tier min point config
    pub fn set_tier_config(&mut self, tier: Tier, config: TierConfig) {
        self.assert_owner();
        self.config.tier_configs.insert(tier, config);
    }

    pub fn reset_lock(&mut self, account_id: AccountId) {
        self.assert_owner();
        self.internal_unlock(account_id);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);

        builder
    }

    #[test]
    fn init_default_contract_test() {
        let context = get_context(false);
        testing_env!(context.build());

        let contract: StakingContract =
            StakingContract::new_default_config(accounts(1).to_string(), "ft_contract".to_string());

        assert_eq!(
            contract.owner_id,
            accounts(1).to_string(),
            "Contract owner should be equal {}",
            accounts(1).to_string()
        );
        assert_eq!(
            contract.ft_contract_id,
            "ft_contract".to_string(),
            "FT contract id should be init data"
        );
        assert_eq!(
            contract.config.reward_numerator,
            Config::default().reward_numerator,
            "Config must be equal default"
        );
        assert_eq!(contract.paused, false);
    }

    #[test]
    fn init_contract_test() {
        let context = get_context(false);
        testing_env!(context.build());

        let contract: StakingContract = StakingContract::new(
            accounts(1).to_string(),
            "ft_contract".to_string(),
            Config {
                reward_numerator: 1500,
                reward_denumerator: 10000000,
                total_apr: 15,
                tier_configs: TierConfig::get_default_tier_configs(),
                min_locking_days: 1,
                max_locking_days: 360,
            },
        );

        assert_eq!(
            contract.owner_id,
            accounts(1).to_string(),
            "Contract owner should be equal {}",
            accounts(1).to_string()
        );
        assert_eq!(
            contract.ft_contract_id,
            "ft_contract".to_string(),
            "FT contract id should be init data"
        );
        assert_eq!(
            contract.config.reward_numerator, 1500,
            "Config must be equal default"
        );
        assert_eq!(contract.config.reward_denumerator, 10000000);
        assert_eq!(contract.paused, false);
    }

    #[test]
    fn test_internal_get_tier() {
        let context = get_context(false);
        testing_env!(context.build());

        let multiplier = 10u128.pow(DEFAULT_TOKEN_DECIMAL as u32);
        let contract = StakingContract::new_default_config(accounts(1).to_string(), accounts(1).to_string());

        // Tier0
        assert_eq!(Tier::Tier0, contract.internal_get_tier(0));
        assert_eq!(Tier::Tier0, contract.internal_get_tier(1 * multiplier));
        assert_eq!(Tier::Tier0, contract.internal_get_tier(100 * multiplier - 1));

        // Tier1
        assert_eq!(Tier::Tier1, contract.internal_get_tier(100 * multiplier));
        assert_eq!(Tier::Tier1, contract.internal_get_tier(100 * multiplier + 1));
        assert_eq!(Tier::Tier1, contract.internal_get_tier(1_000 * multiplier - 1));

        // Tier2
        assert_eq!(Tier::Tier2, contract.internal_get_tier(1_000 * multiplier));
        assert_eq!(Tier::Tier2, contract.internal_get_tier(1_000 * multiplier + 1));
        assert_eq!(Tier::Tier2, contract.internal_get_tier(5_000 * multiplier - 1));

        // Tier3
        assert_eq!(Tier::Tier3, contract.internal_get_tier(5_000 * multiplier));
        assert_eq!(Tier::Tier3, contract.internal_get_tier(5_000 * multiplier + 1));
        assert_eq!(Tier::Tier3, contract.internal_get_tier(10_000 * multiplier - 1));

        // Tier4
        assert_eq!(Tier::Tier4, contract.internal_get_tier(10_000 * multiplier));
        assert_eq!(Tier::Tier4, contract.internal_get_tier(10_000 * multiplier + 1));
    }

    #[test]
    fn deposit_and_stake_test() {
        let mut context = get_context(false);
        context.block_index(0);
        testing_env!(context.build());

        let mut contract: StakingContract =
            StakingContract::new_default_config(accounts(1).to_string(), accounts(1).to_string());
        contract.internal_create_account(env::predecessor_account_id());

        // Deposit and stake function call from FT contract
        context.predecessor_account_id(accounts(1));
        testing_env!(context.build());
        contract.internal_deposit_and_stake(accounts(0).to_string(), 10_000_000_000_000);

        context.block_index(10);
        context.predecessor_account_id(accounts(0));
        testing_env!(context.build());

        // Test deposit balance and
        let upgradable_account = contract.accounts.get(&accounts(0).to_string()).unwrap();
        let account: Account = Account::from(upgradable_account);

        assert_eq!(account.staked_balance, 10_000_000_000_000);
        assert_eq!(account.pre_reward, 0);
        assert_eq!(account.pre_stake_balance, 0);
        assert!(contract.internal_calculate_account_reward(&account) > 0);

        // test contract balance
        assert_eq!(contract.total_stake_balance, account.staked_balance);
        assert_eq!(contract.total_staker, 1);
        assert_eq!(contract.pre_reward, 0);
        assert_eq!(contract.last_block_balance_change, 0);

        // Test update stake balance of account
        // Deposit and stake function call from FT contract
        context.predecessor_account_id(accounts(1));
        testing_env!(context.build());
        contract.internal_deposit_and_stake(accounts(0).to_string(), 20_000_000_000_000);

        context.block_index(20);
        context.predecessor_account_id(accounts(0));
        testing_env!(context.build());

        // Test deposit balance and
        let upgradable_account_2 = contract.accounts.get(&accounts(0).to_string()).unwrap();
        let account_update: Account = Account::from(upgradable_account_2);

        assert_eq!(account_update.staked_balance, 30_000_000_000_000);
        assert!(account_update.pre_reward > 0);
        assert_eq!(account_update.pre_stake_balance, 10_000_000_000_000);
        assert_eq!(account_update.last_block_balance_change, 10);
        assert!(contract.internal_calculate_account_reward(&account_update) > 0);

        // test contract balance
        assert_eq!(contract.total_stake_balance, account_update.staked_balance);
        assert_eq!(contract.total_staker, 1);
        assert!(contract.pre_reward > 0);
        assert_eq!(contract.last_block_balance_change, 10);
    }

    #[test]
    fn unstake_test() {
        let mut context = get_context(false);
        context.block_index(0);
        testing_env!(context.build());

        let mut contract: StakingContract =
            StakingContract::new_default_config(accounts(1).to_string(), accounts(1).to_string());
        contract.internal_create_account(env::predecessor_account_id());

        // Deposit and stake function call from FT contract
        context.predecessor_account_id(accounts(1));
        testing_env!(context.build());
        contract.internal_deposit_and_stake(accounts(0).to_string(), 30_000_000_000_000);

        context.block_index(10);
        context.epoch_height(10);
        context.predecessor_account_id(accounts(0));
        testing_env!(context.build());

        contract.internal_unstake(accounts(0).to_string(), 10_000_000_000_000);

        // Test deposit balance and
        let upgradable_account = contract.accounts.get(&accounts(0).to_string()).unwrap();
        let account: Account = Account::from(upgradable_account);

        assert_eq!(account.staked_balance, 20_000_000_000_000);
        assert_eq!(account.unstaked_balance, 10_000_000_000_000);
        assert_eq!(account.last_block_balance_change, 10);
        assert_eq!(account.unstake_available_epoch_height, 11);
    }

    #[test]
    fn withdraw_test() {}

    #[test]
    fn update_tier_point_cfg_test() {}

    #[test]
    fn get_user_point_test() {}

    #[test]
    fn get_user_tier_test() {}
}