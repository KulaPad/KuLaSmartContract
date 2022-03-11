use near_sdk::{init, env, near_bindgen, ext_contract};
use near_sdk::{PanicOnDefault, Timestamp, Balance, AccountId, CryptoHash, Promise, PromiseOrValue, PromiseResult, EpochHeight};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet, LookupMap};
use near_sdk::json_types::{U128, U64};

pub type ProjectId = u64;
pub type TicketAmount = u32;
pub type TicketNumber = u64;
pub type TicketId = String;
pub type AccountTicketsType = UnorderedMap<AccountId, AccountTickets>;
pub type AccountTokenSalesType = UnorderedMap<AccountId, AccountTokenSales>;
pub type ProjectTicketType = UnorderedMap<TicketId, AccountId>;
pub type AccountProjectType = UnorderedSet<ProjectId>;

use crate::structures::project::*;
use crate::structures::account::*;
use crate::structures::ticket::*;
use crate::structures::staking::*;
use crate::utils::*;
use crate::staking_contract::*;

mod structures;
mod utils;
mod tests;
mod staking_contract;

pub const DEFAULT_PAGE_SIZE: u64 = 100;
pub const TOKEN_DECIMAL: u8 = 8;
pub const STAKING_CONTRACT_ID: &str = "staking-kulapad.testnet";

pub const GAS_FUNCTION_CALL: u64 = 5_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum StorageKey {
    ProjectKey,
    ProjectAccountTicketKey,
    ProjectAccountTicketInnerKey(ProjectId),
    ProjectTokenSaleKey,
    ProjectTokenSaleInnerKey(ProjectId),
    ProjectTicketKey,
    ProjectTicketInnerKey(ProjectId),
    AccountProjectKey,
    AccountProjectKeyInnerKey { 
        account_id_hash: CryptoHash
    },
    TierKey,
    TierTicketInnerKey (String),
    TierAllocationInnerKey (String),
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct IDOContract{
    /// The owner of this contract.
    pub owner_id: AccountId,

    /// The account id of staking contract. It used for cross-contract call
    pub staking_contract_id: AccountId,

    /// Stores the list of projects that belongs to this IDO contract.
    pub projects: UnorderedMap<ProjectId, ProjectInfo>,

    /// Stores the list of tickets that belongs to the specific account for each project.
    /// The user tickets info was re-calculated every time user access the UI, trigger the calculate_user_tickets()
    pub project_account_tickets: LookupMap<ProjectId, AccountTicketsType>,

    /// Stores the list of token sales of an account in each project.
    pub project_account_token_sales: LookupMap<ProjectId, AccountTokenSalesType>,

    /// Stores the list of tickets that belongs to each project.
    /// Ex: Project 1: Tickets [{Id: 1, Type: Staking, Account Id: account1.testnet }, {Id: 2, Type: Social, Account Id: account2.testnet }, ...]
    /// The user tickets were stored here during re-calculate
    pub project_tickets: LookupMap<ProjectId, ProjectTicketType>,
    
    /// The list of projects that that account has registered whitelist.
    pub account_projects: LookupMap<AccountId, AccountProjectType>,

    /// The information of tiers that helps to identify the number of tickets to allocation to a specific user when they joined to a project
    pub tiers: UnorderedMap<StakingTier, TierInfo>,
}

#[near_bindgen]
impl IDOContract{
    #[init]
    pub fn new(owner_id: AccountId, staking_contract_id: Option<AccountId>) -> Self {
        env::log(format!("Creating contract...").as_bytes());
        Self {
            owner_id,
            staking_contract_id: staking_contract_id.unwrap_or(STAKING_CONTRACT_ID.to_string()),
            projects: UnorderedMap::new(get_storage_key(StorageKey::ProjectKey)),
            project_account_tickets: LookupMap::new(get_storage_key(StorageKey::ProjectTicketKey)),
            project_account_token_sales: LookupMap::new(get_storage_key(StorageKey::ProjectTokenSaleKey)),
            project_tickets: LookupMap::new(get_storage_key(StorageKey::ProjectTicketKey)),
            account_projects: LookupMap::new(get_storage_key(StorageKey::AccountProjectKey)),
            tiers: initialize_tiers(TOKEN_DECIMAL),
        }
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate(owner_id: AccountId, staking_contract_id: Option<AccountId>) -> Self {
        env::log(format!("Migrating contract...").as_bytes());
        let contract = IDOContract::new(owner_id, staking_contract_id);
        env::log(format!("Contract migrated.").as_bytes());
        contract
    }

    pub fn get_owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn set_owner_id(&mut self, owner_id: AccountId) {
        assert_eq!(env::signer_account_id(), self.owner_id, "Just owner can do this action.");

        self.owner_id = owner_id;
    }
    
    /// Register an account for a project's whitelist
    /// User can only register the whitelist on the whitelist period of the project
    /// Account id is env::signer_account_id()
    pub fn register_whitelist(&mut self, project_id: ProjectId) {
        let project_info = self.get_project_info(&project_id);                         
        assert_eq!(project_info.status, ProjectStatus::Whitelist,"Project isn't on whitelist");
        let current_time = env::block_timestamp();
        assert!((project_info.whitelist_start_date <= current_time)
                &&(project_info.whitelist_end_date >= current_time),
                "Project isn't on whitelist time");

        let account_id = env::signer_account_id();
        let mut account_projects = self.unwrap_account_project(&account_id);

        assert!(!account_projects.contains(&project_id),"Already register whitelist this project");
        account_projects.insert(&project_id);
        self.account_projects.insert(&account_id,&account_projects);
    }

    /// Check an account wherever registered for a project or not
    pub fn is_whitelist(&self, project_id: ProjectId) -> bool {

        self.assert_project_exist(project_id);

        let account_id = env::signer_account_id();
        let projects_in_account = self.account_projects.get(&account_id);

        if let Some(projects_in_account) = projects_in_account {
            if projects_in_account.contains(&project_id){
                return true;
            }
        }
        
        false
    }

    /// User deposits the exact acount of funding to buy token
    /// Account id is env::signer_account_id()
    /// This function support NEAR deposit only
    #[payable]
    pub fn buy_token(&mut self, project_id: ProjectId)-> Balance {
        
        let project_info = self.projects.get(&project_id).expect("No project found");
        assert!(project_info.status == ProjectStatus::Sales,"Project is not on sale");
        let current_time = env::block_timestamp();
        assert!((project_info.sale_start_date <= current_time)
                &&(project_info.sale_end_date >= current_time),
                "Project isn't on sale time");
        let account_id = env::signer_account_id();
        let mut project_account_token_sales = self.unwrap_project_account_token_sales(project_id);         
        
        // Transfer deposit Near to contract owner
        let account_tickets = self.unwrap_project_account_ticket(project_id, &account_id);
        let tickets_win = account_tickets.staking_tickets.win_ticket_ids.len();
        assert!(tickets_win>0,"Account did not win the whitelist");

        let must_attach_deposit = project_info.token_sale_rate
                                        .multiply(project_info.token_amount_per_sale_slot as u128)
                                        *(tickets_win as u128);
        let deposit_amount = env::attached_deposit();
        assert_eq!(deposit_amount,must_attach_deposit,"Must deposit {} NEAR",must_attach_deposit);
        
        // TODO: Increase total fund of NEAR that user deposited for this project to buy token

        project_account_token_sales.insert(&account_id,&AccountTokenSales{
            funding_amount: deposit_amount,
            token_unlocked_amount: 0,
            token_locked_amount: 0,
            token_withdrawal_amount: 0,
        });
        self.project_account_token_sales.insert(&project_id,&project_account_token_sales);
        
        // Return deposited_near
        deposit_amount
    }

    /// Get token sales info of an account. If it does not exits, return None.
    pub fn get_account_token_sale_info(& self, project_id: ProjectId) -> Option<JsonAccountTokenSales> {
        let account_id = env::signer_account_id();
        let project_account_token_sales = self.project_account_token_sales.get(&project_id);
        if let Some(project_account_token_sales) = project_account_token_sales{
            let account_token_sales = project_account_token_sales.get(&account_id);
            if let Some(account_token_sales)= account_token_sales{
                Some(
                    JsonAccountTokenSales{
                        funding_amount: U128::from(account_token_sales.funding_amount),
                        token_unlocked_amount: U128::from(account_token_sales.token_unlocked_amount),
                        token_locked_amount: U128::from(account_token_sales.token_locked_amount),
                        token_withdrawal_amount: U128::from(account_token_sales.token_withdrawal_amount)
                    }
                )
            }else{
                None
            }
        }else{
            None
        }
        
    }

    /// User can claim their bought unlocked token after sales.
    pub fn claim(&mut self, project_id: ProjectId) {

    }

    /// Usecase 1: Display on the right section of staking page - https://web-app-1vi.pages.dev/#/staking
    ///  * Input: locked_amount, locked_timestamp
    ///  * Output: TierInfo: Tier, Staking Tickets, Allocation
    pub fn get_staking_tier_info(&self, locked_amount: U64, locked_timestamp: Timestamp) -> TierInfoJson {
        self.internal_get_staking_tier_info(locked_amount.into(), locked_timestamp, None)
    }
     
    /// Usecase 2: Display on project details
    ///  * Input: ProjectId, AccountId
    ///  * Output: ProjectAccountInfoJson: Project, Status, Account, WhitelistInfo, SaleInfo, DistributionInfo
    pub fn get_project_account_info(&self, project_id: ProjectId) -> ProjectAccountInfoJson {
        let account_id = env::signer_account_id();
        self.internal_get_project_staking_tier_info(project_id, account_id)
    }

    pub fn update_staking_tickets(&mut self, project_id: ProjectId) -> PromiseOrValue<bool> {
        let account_id = env::signer_account_id();

        // Verify project & account before calling to staking smart contrct
        let project = self.projects.get(&project_id);
        if let Some(project) = project {
            assert_project_whitelist_period(&project);

            // Start processing
            return self.internal_update_staking_tickets(project_id, account_id);
        }
        
        panic_project_not_exist();
        PromiseOrValue::Value(false)
    }

    pub fn close_project_whitelist(&mut self, project_id: ProjectId) {
        // Get project
        let mut project = self.get_project_or_panic(project_id);
        let current_time = get_current_time();

        // Validate & update status to Sale
        assert!(project.status == ProjectStatus::Whitelist && project.whitelist_end_date <= current_time, "The project's status is not correct or the whitelist period is not end.");

        
        // Random list of tickets
        let tickets = self.get_project_ticket_or_panic(project_id);
        let mut account_tickets = self.get_project_account_ticket_or_panic(project_id);
        let no_of_win_tickets = std::cmp::min(project.total_staking_tickets, project.get_available_sales_slots() as u64);

        for i in 0..no_of_win_tickets {
            let ticket_number = i + 1;
            let ticket_id = build_ticket_id(TicketType::Staking, ticket_number);
            
            let account_id = tickets.get(&ticket_id);
            if let Some(account_id) = account_id {
                let account = account_tickets.get(&account_id);
                if let Some(mut account) = account {
                    if account.staking_tickets.ticket_ids.contains(&ticket_number) {
                        account.staking_tickets.win_ticket_ids.push(ticket_number);

                        account_tickets.insert(&account_id, &account);
                        self.project_account_tickets.insert(&project_id, &account_tickets);
                    }
                }
            }
        }

        project.status = ProjectStatus::Sales;
        self.projects.insert(&project_id, &project);
    }

    pub fn get_user_tickets(&self) -> AccountId {
        self.owner_id.clone()
    }

    /// There 2 type of ticket:
    /// - VIP ticket: For staking Tier 4: This ticket can always join the campaign sale event
    /// - normal ticket: This ticket need to pass a lucky round to be chosen
    ///
    /// Only the chosen normal ticket can join the sale round
    ///
    /// Every one got the WIN ticket can join into the token sale, FCFS
    pub fn select_win_tickets(&self) -> AccountId {
        self.owner_id.clone()
    }

    /// User buy power = max buy amount in USD <=> NEAR
    /// Base on the winning ticket of user => buy power
    /// Each winning ticket will have right to buy $100
    pub fn get_user_buy_power(&self) -> AccountId {
        self.owner_id.clone()
    }

}
