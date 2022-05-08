use near_sdk::{init, env, near_bindgen, ext_contract};
use near_sdk::{PanicOnDefault, Timestamp, Balance, AccountId, CryptoHash, Promise, PromiseOrValue, PromiseResult, EpochHeight};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet, LookupMap};
use near_sdk::env::signer_account_id;
use near_sdk::json_types::{U128, U64};

pub type ProjectId = u64;
pub type AllocationNumber = u32;
pub type TicketNumber = u64;
pub type TicketId = String;
pub type ProjectAccountUnorderedMap = UnorderedMap<AccountId, ProjectAccount>;
pub type TicketAndAccountLookupMap = LookupMap<TicketId, AccountId>;
pub type ProjectIdUnorderedSet = UnorderedSet<ProjectId>;

use crate::modules::project::*;
use crate::modules::account::*;
use crate::modules::xtoken::*;
use crate::utils::*;
use crate::staking_contract::*;
use crate::ft_contract::*;
use crate::modules::tier::{Tier, TierConfigs, TierMinPointConfig, TierInfo, UserTierInfo};

mod modules;
mod utils;
mod tests;
mod staking_contract;
mod ft_contract;

pub const DEFAULT_PAGE_SIZE: u64 = 100;
pub const TOKEN_DECIMAL: u8 = 8;

pub const GAS_FUNCTION_CALL: u64 = 5_000_000_000_000;
pub const GAS_FUNCTION_CALL_UPDATE_STAKING_TIER: u64 = 50_000_000_000_000;
pub const GAS_FUNCTION_CALL_GET_USER_POINT: u64 = 50_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum StorageKey {
    FundingTokenIdKey,
    ProjectKey,
    AccountsByProjectKey,
    AccountsByProjectInnerKey(ProjectId),
    TicketsByProjectKey,
    TicketsByProjectInnerKey(ProjectId),
    ProjectsByAccountKey,
    ProjectsByAccountInnerKey {
        account_id_hash: CryptoHash
    },
    TierKey,
    TierTicketInnerKey (String),
    TierAllocationInnerKey (String),
    TierConfigsKey,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    /// the config for each user Tier
    pub tier_point: Vec<TierMinPointConfig>,
}

impl Config {
    pub fn get_default_tier_cfg() -> TierMinPointConfig {
        let mut cfg = TierConfigs::new(StorageKey::TierConfigsKey);
        cfg.insert(&Tier::Tier0, &TierInfo { ticket: 0, allocation: 0 });
        cfg.insert(&Tier::Tier1, &TierInfo { ticket: 1, allocation: 0 });
        cfg.insert(&Tier::Tier2, &TierInfo { ticket: 12, allocation: 0 });
        cfg.insert(&Tier::Tier3, &TierInfo { ticket: 100, allocation: 0 });
        cfg.insert(&Tier::Tier4, &TierInfo { ticket: 100, allocation: 1 });
        // cfg.extend(vec![
        //     (Tier::Tier0, 0_u64),
        //     (Tier::Tier1, 100_u64),
        //     (Tier::Tier2, 1000_u64),
        //     (Tier::Tier3, 5000_u64),
        //     (Tier::Tier4, 10000_u64),
        // ].into_iter());

        return cfg;
    }

    pub fn set_tier_cfg(&mut self, tier: Tier, tier_info: TierInfo) {
        self.tier_point.insert(&tier, &tier_info);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tier_point: Config::get_default_tier_cfg(),
        }
    }
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct IDOContract{
    /// The owner of this contract.
    pub owner_id: AccountId,

    /// The account id of staking contract. It used for cross-contract call
    pub staking_contract_id: AccountId,

    /// The list of token id that allows to buy an IDO project.
    pub funding_ft_token_ids: UnorderedSet<AccountId>,

    /// Stores the list of projects that belongs to this IDO contract.
    pub projects: UnorderedMap<ProjectId, Project>,

    /// Stores the list of accounts for each project.
    /// Each account contains the Sale and Distribution data.
    pub accounts_by_project: LookupMap<ProjectId, ProjectAccountUnorderedMap>,

    /// Stores the list of tickets that belongs to each project.
    /// Ex: Project 1: Tickets [{Id: L1, Account Id: account1.testnet }, {Id: S2, Account Id: account2.testnet }, ...]
    /// The user tickets were stored here during re-calculate
    pub tickets_by_project: LookupMap<ProjectId, TicketAndAccountLookupMap>,

    /// The list of projects that that account has registered whitelist.
    pub projects_by_account: LookupMap<AccountId, ProjectIdUnorderedSet>,

    /// The information of tiers that helps to identify the number of tickets to allocation to a specific user when they joined to a project
    //pub tiers: UnorderedMap<Tier, TierInfo>,

    pub test_mode_enabled: bool,

    pub config: Config,
}

#[near_bindgen]
impl IDOContract {
    #[init]
    pub fn new_with_default_config(owner_id: AccountId, staking_contract_id: AccountId, funding_ft_token_ids: Option<Vec<AccountId>>, test_mode_enabled: Option<bool>) -> Self {
        Self::new(owner_id, staking_contract_id, funding_ft_token_ids, test_mode_enabled, Config::default())
    }

    #[init]
    pub fn new(owner_id: AccountId, staking_contract_id: AccountId, funding_ft_token_ids: Option<Vec<AccountId>>, test_mode_enabled: Option<bool>, config: Config) -> Self {
        env::log(format!("Creating contract...").as_bytes());
        let mut contract = Self {
            owner_id,
            staking_contract_id,
            funding_ft_token_ids: UnorderedSet::new(get_storage_key(StorageKey::FundingTokenIdKey)),
            projects: UnorderedMap::new(get_storage_key(StorageKey::ProjectKey)),
            accounts_by_project: LookupMap::new(get_storage_key(StorageKey::AccountsByProjectKey)),
            tickets_by_project: LookupMap::new(get_storage_key(StorageKey::TicketsByProjectKey)),
            projects_by_account: LookupMap::new(get_storage_key(StorageKey::ProjectsByAccountKey)),
            test_mode_enabled: test_mode_enabled.unwrap_or(true),
            config,
        };

        if let Some(funding_ft_token_ids) = funding_ft_token_ids {
            for i in 0..funding_ft_token_ids.len() {
                contract.funding_ft_token_ids.insert(&funding_ft_token_ids[i]);
            }
        }

        contract
    }

    // Owner functions

    pub fn get_owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn set_owner_id(&mut self, owner_id: AccountId) {
        assert_eq!(env::signer_account_id(), self.owner_id, "Just owner can do this action.");

        self.owner_id = owner_id;
    }

    // Project call functions

    pub fn create_project(&mut self, project: ProjectInput) -> ProjectId{
        self.internal_create_project(Project::from(project))
    }

    pub fn change_project_status(&mut self, project_id: ProjectId) {
        self.internal_change_project_status(project_id);
    }

    // Project view functions

    pub fn get_projects(&self, status: Option<ProjectStatus>, from_index: Option<u64>, limit: Option<u64>) -> Vec<ProjectJson>{
        self.projects
        .iter()
        .filter(|(_, project)| match &status { None => true, Some(s) => &project.status == s })
        .skip(from_index.unwrap_or(0) as usize)
        .take(limit.unwrap_or(DEFAULT_PAGE_SIZE) as usize)
        .map(|(project_id, project)| self.internal_get_project(&project_id, Some(project)).unwrap())
        .collect()
    }

    pub fn get_project(&self, project_id: ProjectId) -> Option<ProjectJson> {
        let project = self.projects.get(&project_id);

        self.internal_get_project(&project_id, project)
    }

    pub fn get_project_account_info(&self, project_id: ProjectId, account_id: Option<AccountId>) -> ProjectAccountJson {
        self.internal_get_project_account_info(project_id, account_id.unwrap_or(env::signer_account_id()))
    }

    // Project Whitelist

    /// Register an account for a project's whitelist
    /// User can only register the whitelist on the whitelist period of the project
    pub fn register_whitelist(&mut self, project_id: ProjectId) {
        self.internal_register_whitelist(project_id);
    }

    /// Check an account wherever registered for a project or not
    pub fn is_whitelist(&self, project_id: ProjectId, account_id: Option<AccountId>) -> bool {
        self.assert_project_exist(project_id);

        let account_id = account_id.unwrap_or(env::signer_account_id());
        let projects_in_account = self.projects_by_account.get(&account_id);

        if let Some(projects_in_account) = projects_in_account {
            if projects_in_account.contains(&project_id){
                return true;
            }
        }

        false
    }

    // Project Sale

    pub fn update_staking_tickets(&mut self, project_id: ProjectId) -> PromiseOrValue<bool> {
        let account_id = env::signer_account_id();

        // Verify project & account before calling to staking smart contrct
        let project = self.projects.get(&project_id);
        if let Some(project) = project {
            project.assert_whitelist_period();

            // Start processing
            return self.internal_update_staking_tickets(project_id, account_id);
        }

        panic_project_not_exist();
        PromiseOrValue::Value(false)
    }

    pub fn close_project_whitelist(&mut self, project_id: ProjectId) {
        println!("close_project_whitelist - inside");
        // Get project
        let mut project = self.internal_get_project_or_panic(project_id);
        let current_time = get_current_time();

        println!("close_project_whitelist - get_current_time");
        // Validate & update status to Sale ->
        assert!(project.status == ProjectStatus::Whitelist && project.whitelist_end_date <= current_time, "{}", format!("The project's status ({:?}) is not correct or the whitelist period (End: {} - Current: {}) is not end.", project.status, project.whitelist_end_date, current_time));

        // TODO

        println!("close_project_whitelist - end of for");

        project.status = ProjectStatus::Sales;
        self.projects.insert(&project_id, &project);
    }

    /// get UserTierInfo: tier, point, ticket, alloc
    pub fn get_user_tier_info(&self) -> UserTierInfo {
        let user: AccountId = env::predecessor_account_id();
        UserTierInfo::get_user_tier_info(user, self.config.tier_point)
    }
}
