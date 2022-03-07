
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, PanicOnDefault, Timestamp, Balance, AccountId, CryptoHash};
use near_sdk::collections::{UnorderedMap, LookupMap, UnorderedSet};
// use near_sdk::json_types::{U128};

pub type ProjectId = u64;
pub type TicketId = u64;
pub type TicketsAmount = u64;

// project_id = 1; ticket_id = 10 
// => project_and_ticket_id = 1_10  
pub type ProjectAndTicketId = String;

use crate::structures::project::*;
use crate::structures::account::*;
use crate::structures::ticket::*;
use crate::utils::*;

mod structures;
mod utils;
mod tests;


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
        project_id_hash: CryptoHash
    },
    TicketsAmountPerOwnerIdKey,
    AccountProjectKey,
    AccountProjectInnerKey{
        account_id_hash: CryptoHash
    },
    TicketInfoKey,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct IDOContract{
    /// The owner of this contract.
    pub owner_id: AccountId,

    /// Stores the list of projects that belongs to this IDO contract.
    /// Only use when account-id owned some ticket in that project
    pub projects: UnorderedMap<ProjectId, ProjectInfo>,



    // Owner's tickets amount
    pub staking_tickets_amount_per_owner_id: LookupMap<AccountId,TicketsAmount>,

    /// The list of projects that that account has registered whitelist.
    pub account_projects: LookupMap<AccountId, UnorderedSet<ProjectId>>,

    /// Stores the list of tickets that belongs to each project.
    pub project_tickets: LookupMap<ProjectId, UnorderedSet<TicketId>>,

    pub ticket_info: LookupMap<ProjectAndTicketId, Ticket>,

    
    /// Stores the list of tickets that belongs to the specific account for each project.
    pub project_account_tickets: LookupMap<ProjectId, UnorderedMap<AccountId, AccountTickets>>,

    /// Stores the list of token sales of an account in each project.
    pub project_account_token_sales: LookupMap<ProjectId, UnorderedMap<AccountId, AccountTokenSales>>,

    

    
}

#[near_bindgen]
impl IDOContract{
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            staking_tickets_amount_per_owner_id: LookupMap::new(get_storage_key(StorageKey::TicketsAmountPerOwnerIdKey)),
            projects: UnorderedMap::new(get_storage_key(StorageKey::ProjectKey)),
            project_account_tickets: LookupMap::new(get_storage_key(StorageKey::ProjectTicketKey)),
            project_account_token_sales: LookupMap::new(get_storage_key(StorageKey::ProjectTokenSaleKey)),
            project_tickets: LookupMap::new(get_storage_key(StorageKey::ProjectTicketKey)),
            ticket_info: LookupMap::new(get_storage_key(StorageKey::TicketInfoKey)),
            account_projects: LookupMap::new(get_storage_key(StorageKey::AccountProjectKey)),
        }
    }

    pub fn get_owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }
}