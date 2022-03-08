
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{init, env, near_bindgen, PanicOnDefault, Timestamp, Balance, AccountId, CryptoHash};
use near_sdk::collections::{UnorderedMap, LazyOption, LookupMap};
use near_sdk::json_types::{U128};

pub type ProjectId = u64;

use crate::structures::project::*;
use crate::structures::account::*;
use crate::structures::ticket::*;
use crate::structures::staking::*;
use crate::utils::*;

mod structures;
mod utils;
mod tests;

pub const DEFAULT_PAGE_SIZE: u64 = 100;
pub const TOKEN_DECIMAL: u8 = 8;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum StorageKey {
    ProjectKey,
    ProjectAccountTicketKey,
    ProjectAccountTicketInnerKey {
        account_id_hash: CryptoHash
    },
    ProjectTokenSaleKey,
    ProjectTokenSaleInnerKey {
        account_id_hash: CryptoHash
    },
    ProjectTicketKey,
    ProjectTicketInnerKey {

    },
    AccountProjectKey,
    TierKey,
    TierTicketInnerKey (String),
    TierAllocationInnerKey (String),
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct IDOContract{
    /// The owner of this contract.
    pub owner_id: AccountId,

    /// Stores the list of projects that belongs to this IDO contract.
    pub projects: UnorderedMap<ProjectId, ProjectInfo>,

    /// Stores the list of tickets that belongs to the specific account for each project.
    ///
    /// The user tickets info was re-calculated every time user access the UI, trigger the calculate_user_tickets()
    pub project_account_tickets: LookupMap<ProjectId, UnorderedMap<AccountId, AccountTickets>>,

    /// Stores the list of token sales of an account in each project.
    pub project_account_token_sales: LookupMap<ProjectId, UnorderedMap<AccountId, AccountTokenSales>>,

    /// Stores the list of tickets that belongs to each project.
    /// Ex: Project 1: Tickets [{Id: 1, Type: Staking, Account Id: account1.testnet }, {Id: 2, Type: Social, Account Id: account2.testnet }, ...]
    ///
    /// The user tickets were stored here during re-calculate
    pub project_tickets: LookupMap<ProjectId, LookupMap<TicketId, Ticket>>,

    /// The list of projects that that account has registered whitelist.
    pub account_projects: LookupMap<AccountId, Vec<ProjectId>>,

    /// Last increment ticket id
    pub last_ticket_id: TicketId,

    /// The information of tiers that helps to identify the number of tickets to allocation to a specific user when they joined to a project
    pub tiers: UnorderedMap<StakingTier, TierInfo>,
}

#[near_bindgen]
impl IDOContract{
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        env::log(format!("Creating contract...").as_bytes());
        Self {
            owner_id,
            projects: UnorderedMap::new(get_storage_key(StorageKey::ProjectKey)),
            project_account_tickets: LookupMap::new(get_storage_key(StorageKey::ProjectTicketKey)),
            project_account_token_sales: LookupMap::new(get_storage_key(StorageKey::ProjectTokenSaleKey)),
            project_tickets: LookupMap::new(get_storage_key(StorageKey::ProjectTicketKey)),
            account_projects: LookupMap::new(get_storage_key(StorageKey::AccountProjectKey)),
            tiers: initialize_tiers(TOKEN_DECIMAL),
            last_ticket_id: 0,
        }
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate(owner_id: AccountId) -> Self {
        env::log(format!("Migrating contract...").as_bytes());
        let contract = IDOContract::new(owner_id);
        env::log(format!("Contract migrated.").as_bytes());
        contract
    }

    pub fn get_owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }
    
   /// Register an account for a project's whitelist
    /// User can only register the whitelist on the whitelist period of the project
    /// Account id is env::signer_account_id()
    pub fn register_whitelist(&mut self, project_id: ProjectId) {

    }

    /// Check an account wherever registered for a project or not
    pub fn is_whitelist(&self, project_id: ProjectId) -> bool {
        false
    }

    /// User deposits the exact acount of funding to buy token
    /// Account id is env::signer_account_id()
    /// This function support NEAR deposit only
    #[payable]
    pub fn buy_token(&mut self, project_id: ProjectId) {

    }

    /// Get token sales info of an account. If it does not exits, return None.
    pub fn get_account_token_sale_info(& self, project_id: ProjectId) -> Option<JsonAccountTokenSales> {
        None
    }

    /// User can claim their bought unlocked token after sales.
    pub fn claim(&mut self, project_id: ProjectId) {

    }

    // ====> sample project go here

    // ====> whitelist go here

    // ====> ticket go here
    ///
    /// Ticket was issued:
    /// - Right after clicking Whitelist registration btn
    /// - UI call a fn to calculate user staking ticket
    /// - User stake more KULA
    /// ======> combine into only 1 fn (this fn), and be triggered by user from UI only
    ///
    /// Calculate ticket amount for Whitelisted users, base on:
    /// - Staking time
    /// - Stake amount
    /// NOTE: Max 50 or 100 tickets for each users
    ///
    /// For example:
    /// - This campaign need to raise $2M
    /// => If $100/ticket: Sale Up to 20,000 win tickets
    /// Normal ticket can buy up to $XXX === VIP ticket
    ///
    pub fn calculate_user_tickets(&mut self, project_id: ProjectId) -> JsonAccountTicketInfo {
        // assert!("TODO", "Whitelist is closed. Can calculate during whitelist phase only");
        // assert!("TODO", "User is not whitelisted, you must register the whitelist first");
        // assert!("TODO", "User have not staked, you must stake at least 100KULA to be able to get the campaign tickets");


        /**
        Config is defined here:
        https://docs.google.com/spreadsheets/d/1XWL2vtGIX89kGgj6M-X-ocCrQfz05fm9n4HncDrSuSU/edit#gid=778618928&range=G4
         */
        type TicketCount = u32;
        struct TicketAmountConfig {
            lock_day_count: Vec<u32>,
            tier1: Vec<TicketCount>,
            tier2: Vec<TicketCount>,
            tier3: Vec<TicketCount>,
            tier4: Vec<TicketCount>,
        }
        let ticketConfig = TicketAmountConfig {
            lock_day_count: vec![7, 14, 30, 90, 180, 365],
            tier1: vec![1, 2, 4, 8, 12, 20],
            tier2: vec![6, 12, 24, 48, 72, 120],
            tier3: vec![35, 70, 140, 280, 420, 700],
            tier4: vec![1, 1, 2, 2, 3, 3],
        };

        // TODO: Get from StakingPool contract
        let user_staking_info = UserStakingInfo {
            tier: StakingTier::Tier1,
            staked: 880,
            un_staked: 20,
            staked_at: 1644059117000000000,
            lock_day_count: 200
        };

        // Find the suitable lock day count index
        let mut valid_lock_day_count_idx = 0;
        for i in 1..ticketConfig.lock_day_count.len() {
            let max_count = ticketConfig.lock_day_count[i];
            if user_staking_info.lock_day_count >= max_count {
                valid_lock_day_count_idx = i
            }
        }

        let issue_amount = match user_staking_info.tier {
            StakingTier::Tier1 => ticketConfig.tier1[valid_lock_day_count_idx],
            StakingTier::Tier2 => ticketConfig.tier2[valid_lock_day_count_idx],
            StakingTier::Tier3 => ticketConfig.tier3[valid_lock_day_count_idx],
            StakingTier::Tier4 => ticketConfig.tier4[valid_lock_day_count_idx],
        };

        // let ticket_rank: TicketRank = if user_staking_info.tier == StakingTier.Tier4 { TicketRank.Vip } else { TicketRank.Normal };
        // let tickets = self.issue_staking_ticket(
        //     project_id: ProjectId,
        //     issue_amount,
        //     ticket_rank
        // );
        // let ticket_ids: Vec<TicketId> = tickets.iter().map(|&t| t.id).collect::<Vec<_>>();
        // let mut win_ticket_ids = vec![];
        // if ticket_rank == TicketRank.Vip {
        //     win_ticket_ids = ticket_ids.clone();
        // }



        let mut user_ticket_info = AccountTickets {
            staking_ticket_ids: Vec::new(), //ticket_ids,
            social_ticket_ids: vec![],
            referral_ticket_ids: vec![],
            win_ticket_ids: Vec::new(),
        };

        JsonAccountTicketInfo {
            staking_tickets: user_ticket_info.staking_ticket_ids.len() as u16,
            social_tickets: user_ticket_info.staking_ticket_ids.len() as u16,
            referral_tickets: user_ticket_info.staking_ticket_ids.len() as u16,
            win_tickets: user_ticket_info.staking_ticket_ids.len() as u8,
        }
    }

    // /// Batch issue ticket for users
    // /// Eg: issue ticket 100 -> 200 to to current user
    pub fn issue_staking_ticket(&mut self, project_id: ProjectId, tickets_count: u32, rank: TicketRank) -> Vec<Ticket> {
        let mut new_tickets: Vec<Ticket> = vec![];

        // take up some ticket
        let last_ticket_id = self.last_ticket_id;
        self.last_ticket_id = last_ticket_id + tickets_count as u64;


        let user = env::signer_account_id();

        // init if not exist
        let pt = self.project_tickets.get(&project_id);
        let mut project_tickets: LookupMap<TicketId, Ticket> = if pt.is_none() {
            //LookupMap::new(concat!("p", project_id as String, "_tickets_"))
            LookupMap::new(b"".to_vec())
        } else {
            pt.unwrap()
        };

        // init if not exist
        let pat = self.project_account_tickets.get(&project_id);
        let mut project_account_tickets: UnorderedMap<AccountId, AccountTickets> = if pat.is_none() {
            //UnorderedMap::new(concat!("p", project_id, "_a", user, "_tickets_"))
            UnorderedMap::new(b"".to_vec())
        } else {
            pat.unwrap()
        };

        // init if not exist
        let at = project_account_tickets.get(&user);
        let mut account_tickets: AccountTickets = if at.is_none() {
            AccountTickets {
                staking_ticket_ids: vec![],
                social_ticket_ids: vec![],
                referral_ticket_ids: vec![],
                win_ticket_ids: vec![],
            }
        } else {
            at.unwrap()
        };


        // issue ticket
        for i in last_ticket_id + 1..last_ticket_id + tickets_count as u64 + 1 {
            let ticket = Ticket {
                id: i,
                account_id: user.clone(),
                ticket_type: TicketType::Staking,
                //rank: rank,
                rank: TicketRank::Normal,
            };
            //new_tickets.push(ticket);
            project_tickets.insert(&i, &ticket);
            account_tickets.staking_ticket_ids.push(i);
        };


        // stored
        project_account_tickets.insert(&user, &account_tickets);
        self.project_account_tickets.insert(&project_id, &project_account_tickets);
        self.project_tickets.insert(&project_id, &project_tickets);


        new_tickets
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

    // ====> buy token go here

    // ====> token vesting (claim) go here
}
