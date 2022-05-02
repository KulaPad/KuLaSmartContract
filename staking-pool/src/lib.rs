mod modules;
mod util;
mod internal;
mod core_impl;
mod enumeration;

use near_sdk::collections::{UnorderedSet,LookupMap};
use near_sdk::{near_bindgen, AccountId, env, PanicOnDefault, Balance, EpochHeight, BlockHeight, BorshStorageKey, Promise, PromiseResult, PromiseOrValue, ext_contract};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize,};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{U128, U64};

use crate::modules::account::{Account, AccountJson, UpgradableAccount};
use crate::modules::tier::{TierMinPointConfig, Tier};
pub use crate::enumeration::PoolInfo;
use crate::util::*;


pub const NO_DEPOSIT: Balance = 0;
pub const DEPOSIT_ONE_YOCTOR: Balance = 1;
pub const NUM_EPOCHS_TO_UNLOCK: EpochHeight = 1;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    // Percent reward per 1 block
    pub reward_numerator: u64,
    /// What is?
    pub reward_denumerator: u64,
    pub total_apr: u64,

    /// the config for each user Tier
    pub tier_point_configs: Vec<TierMinPointConfig>,
}

impl Config {
    pub fn get_default_tier_min_point_cfg() -> Vec<TierMinPointConfig> {
       
        let mut cfg = Vec::new();
        cfg.push(TierMinPointConfig{
            tier: Tier::Tier0,
            min_point: 0
        });
        cfg.push(TierMinPointConfig{
            tier: Tier::Tier1,
            min_point: 100
        });
        cfg.push(TierMinPointConfig{
            tier: Tier::Tier2,
            min_point: 1_000
        });
        cfg.push(TierMinPointConfig{
            tier: Tier::Tier2,
            min_point: 5_000
        });
        cfg.push(TierMinPointConfig{
            tier: Tier::Tier3,
            min_point: 10_000
        });

        return cfg;
    }

    pub fn set_tier_min_point_cfg(&mut self, tier: Tier, min_point: u64) {
        let tier_min_point_configs = TierMinPointConfig{
            tier: tier,
            min_point: min_point
        };
        self.tier_point_configs.push(tier_min_point_configs);
    }

    pub fn reset_tier_min_point_cfg(&mut self) {
        self.tier_point_configs = Vec::new();
    }
}

impl Default for Config {
    fn default() -> Self {
        // By default APR 15%
        Self {
            reward_numerator: 715, reward_denumerator: 100000000000, total_apr: 15,
            tier_point_configs: Config::get_default_tier_min_point_cfg(),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey,
    PointConfigKey,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct StakingContract {
    pub owner_id: AccountId, // Owner of contract
    pub ft_contract_id: AccountId,
    pub config: Config, // Config reward and apr for contract
    pub total_stake_balance: Balance, // Total token balance lock in contract
    pub total_paid_reward_balance: Balance,
    pub total_staker: Balance, // TODO: integer
    pub pre_reward: Balance, // Pre reward before change total balance
    pub last_block_balance_change: BlockHeight,
    pub accounts: LookupMap<AccountId, UpgradableAccount>, // List staking user
    pub paused: bool, // Pause staking pool with limit reward,
    pub paused_in_block: BlockHeight
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
            paused_in_block: 0
        }
    }

    pub fn get_total_pending_reward(&self) -> U128 {
        assert_eq!(self.owner_id, env::predecessor_account_id(), "ERR_ONLY_OWNER_CONTRACT");
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
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner contract can be access");
    }

    #[init(ignore_state)]
    #[private]
    pub fn migrate() -> Self {
        let contract: StakingContract = env::state_read().expect("ERR_READ_CONTRACT_STATE");
        contract
    }

    /// create or update tier min point config
    pub fn set_tier_point(&mut self, tier: Tier, min_point: U64) {
        self.config.set_tier_min_point_cfg(tier, min_point.0);
    }

    /// Get user point(xKula) amount by account id
    pub fn get_user_point(&self, account_id: AccountId) -> U64 {
        let account: Option<UpgradableAccount> = self.accounts.get(&account_id);
        if account.is_some() {
            let acc: Account = Account::from(account.unwrap());
            U64(acc.point as u64)
        } else {
            U64(0)
        }
    }

    /// Get user staking tier and point by account id
    pub fn get_user_tier(&self, account_id: AccountId) -> (Tier, U64) {
        let point = self.get_user_point(account_id);
        let mut user_tier = Tier::Tier0;
        let configs = self.config.tier_point_configs.clone();
        for cfg in configs {
            if point.0 >= cfg.min_point {
                user_tier = cfg.tier
            } else {
                break
            }
        }

        (user_tier, point)
    }

    /// Get user staking tier trimming down:
    /// Use for calculating Ticket & Allocation
    /// Eg: 9.999 point =
    ///     5.000 + 4.000 + 999
    ///     1 x T3 = 100
    ///     4 x T2 = 48
    ///     9 x T1 = 9
    ///     100 + 48 + 9 = 157
    ///
    /// return Vec<(Tier, min_tier_point, total_valid_point_at_this_tier)>
    pub fn get_matched_tiers(&self, mut point: U64) -> Vec<(Tier, U64, U64)> {
        let mut tiers: Vec<(Tier, U64, U64)> = vec![];
        let configs = self.config.tier_point_configs.clone();
        for cfg in configs.iter().rev() {
            if cfg.min_point <= 0 {
                // tier0
                let tier_point = point;
                tiers.push((cfg.tier.clone(), U64(cfg.min_point), tier_point));

                point = U64(0);
            } else {
                let mut point_u64 = point.0;
                // has this tier
                if point_u64 >= cfg.min_point {
                    let tier_point = cfg.min_point * (point_u64 / cfg.min_point); // NOTE: u64 division so we don't need to floor
                    tiers.push((cfg.tier.clone() , U64(cfg.min_point), U64(tier_point)));

                    point_u64 -= tier_point;

                    point = U64(point_u64);
                }
            }
        }

        tiers
    }
}


#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.
        current_account_id(accounts(0))
        .signer_account_id(accounts(0))
        .predecessor_account_id(accounts(0))
        .is_view(is_view);

        builder
    }

    #[test]
    fn init_default_contract_test() {
        let context = get_context(false);
        testing_env!(context.build());

        let contract: StakingContract = StakingContract::new_default_config(accounts(1).to_string(), "ft_contract".to_string());

        assert_eq!(contract.owner_id, accounts(1).to_string(), "Contract owner should be equal {}", accounts(1).to_string());
        assert_eq!(contract.ft_contract_id, "ft_contract".to_string(), "FT contract id should be init data");
        assert_eq!(contract.config.reward_numerator, Config::default().reward_numerator, "Config must be equal default");
        assert_eq!(contract.paused, false);
    }

    #[test]
    fn init_contract_test() {
        let context = get_context(false);
        testing_env!(context.build());

        let contract: StakingContract = StakingContract::new(accounts(1).to_string(), "ft_contract".to_string(), Config {
            reward_numerator: 1500,
            reward_denumerator: 10000000,
            total_apr: 15,
            tier_point_configs: Config::get_default_tier_min_point_cfg(),
        });

        assert_eq!(contract.owner_id, accounts(1).to_string(), "Contract owner should be equal {}", accounts(1).to_string());
        assert_eq!(contract.ft_contract_id, "ft_contract".to_string(), "FT contract id should be init data");
        assert_eq!(contract.config.reward_numerator, 1500, "Config must be equal default");
        assert_eq!(contract.config.reward_denumerator, 10000000);
        assert_eq!(contract.paused, false);
    }

    #[test]
    fn deposit_and_stake_test() {
        let mut context = get_context(false);
        context.block_index(0);
        testing_env!(context.build());

        let mut contract: StakingContract = StakingContract::new_default_config(accounts(1).to_string(), accounts(1).to_string());
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

        assert_eq!(account.stake_balance, 10_000_000_000_000);
        assert_eq!(account.pre_reward, 0);
        assert_eq!(account.pre_stake_balance, 0);
        assert!(contract.internal_calculate_account_reward(&account) > 0);

        // test contract balance
        assert_eq!(contract.total_stake_balance, account.stake_balance);
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

        assert_eq!(account_update.stake_balance, 30_000_000_000_000);
        assert!(account_update.pre_reward > 0);
        assert_eq!(account_update.pre_stake_balance, 10_000_000_000_000);
        assert_eq!(account_update.last_block_balance_change, 10);
        assert!(contract.internal_calculate_account_reward(&account_update) > 0);

        // test contract balance
        assert_eq!(contract.total_stake_balance, account_update.stake_balance);
        assert_eq!(contract.total_staker, 1);
        assert!(contract.pre_reward > 0);
        assert_eq!(contract.last_block_balance_change, 10);
    }

    #[test]
    fn unstake_test() {
        let mut context = get_context(false);
        context.block_index(0);
        testing_env!(context.build());

        let mut contract: StakingContract = StakingContract::new_default_config(accounts(1).to_string(), accounts(1).to_string());
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

        assert_eq!(account.stake_balance, 20_000_000_000_000);
        assert_eq!(account.unstake_balance, 10_000_000_000_000);
        assert_eq!(account.last_block_balance_change, 10);
        assert_eq!(account.unstake_available_epoch_height, 11);
    }

    #[test]
    fn withdraw_test() {

    }

    #[test]
    fn update_tier_point_cfg_test() {

    }

    #[test]
    fn get_user_point_test() {

    }

    #[test]
    fn get_user_tier_test() {

    }

    #[test]
    fn get_user_staking_matched_tiers_test() {

    }
}
