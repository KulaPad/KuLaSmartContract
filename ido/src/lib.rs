
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{init, env, near_bindgen, PanicOnDefault, Timestamp, Balance, AccountId, CryptoHash};

use near_sdk::collections::{UnorderedMap, LazyOption, LookupMap};

pub type ProjectId = u64;
pub type TicketId = u64;

use crate::structures::project::*;
use crate::structures::account::*;
use crate::structures::ticket::*;
use crate::structures::internal::*;
use crate::utils::*;

mod structures;
mod utils;
mod tests;

pub const DEFAULT_PAGE_SIZE: u64 = 100;

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
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct IDOContract{
    /// The owner of this contract.
    pub owner_id: AccountId,

    /// Stores the list of projects that belongs to this IDO contract.
    pub projects: UnorderedMap<ProjectId, ProjectInfo>,

    /// Stores the list of tickets that belongs to the specific account for each project.
    pub project_account_tickets: LookupMap<ProjectId, UnorderedMap<AccountId, AccountTickets>>,

    /// Stores the list of token sales of an account in each project.
    pub project_account_token_sales: LookupMap<ProjectId, UnorderedMap<AccountId, AccountTokenSales>>,

    /// Stores the list of tickets that belongs to each project.
    /// Ex: Project 1: Tickets [{Id: 1, Type: Staking, Account Id: account1.testnet }, {Id: 2, Type: Social, Account Id: account2.testnet }, ...]
    pub project_tickets: LookupMap<ProjectId, LookupMap<TicketId, Ticket>>,

    /// The list of projects that that account has registered whitelist.
    pub account_projects: LookupMap<AccountId, Vec<ProjectId>>,
}

#[near_bindgen]
impl IDOContract{
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            projects: UnorderedMap::new(get_storage_key(StorageKey::ProjectKey)),
            project_account_tickets: LookupMap::new(get_storage_key(StorageKey::ProjectTicketKey)),
            project_account_token_sales: LookupMap::new(get_storage_key(StorageKey::ProjectTokenSaleKey)),
            project_tickets: LookupMap::new(get_storage_key(StorageKey::ProjectTicketKey)),
            account_projects: LookupMap::new(get_storage_key(StorageKey::AccountProjectKey)),
        }
    }

    pub fn get_owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }

    // ====> sample project go here

    // ====> whitelist go here

    // ====> ticket go here

    // ====> buy token go here

    // ====> token vesting (claim) go here
}
