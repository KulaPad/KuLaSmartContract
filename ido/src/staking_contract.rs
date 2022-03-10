use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountJson {
    pub account_id: AccountId,
    pub lock_balance: U128,
    pub unlock_timestamp: Timestamp,
    pub stake_balance: U128,
    pub unstake_balance: U128,
    pub reward: U128,
    pub can_withdraw: bool,
    pub start_unstake_timestamp: Timestamp,
    pub unstake_available_epoch: EpochHeight,
    pub current_epoch: EpochHeight
}

#[ext_contract(ext_staking_contract)]
pub trait StakingContract {
    fn get_account_info(&self, account_id: AccountId) -> AccountJson;
}

#[ext_contract(ext_self)]
pub trait IDOContractResolver {
    fn resolve_get_account_info_for_updating_tickets(&mut self, project_id: ProjectId, account_id: AccountId) -> Option<AccountJson>;
}

#[near_bindgen]
impl IDOContract {
    pub(crate) fn internal_update_staking_tickets(&self, project_id: ProjectId, account_id: AccountId) -> PromiseOrValue<bool> {
        env::log(format!("update_staking_tickets(account_id: '{}')", account_id).as_bytes());

        ext_staking_contract::get_account_info(
            account_id.clone(),
            &self.staking_contract_id,
            NO_DEPOSIT,
            GAS_FUNCTION_CALL
        )
        .then(ext_self::resolve_get_account_info_for_updating_tickets(
            project_id,
            account_id,
            &env::current_account_id(),
            NO_DEPOSIT,
            GAS_FUNCTION_CALL
        )).into()
    }

    pub fn resolve_get_account_info_for_updating_tickets(&mut self, project_id: ProjectId, account_id: AccountId) -> bool {
        assert_eq!(
            env::promise_results_count(),
            1,
            "This is a callback method"
        );

        // handle the result from the cross contract call this method is a callback for
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => false,
            PromiseResult::Successful(result) => {
                let account_info = near_sdk::serde_json::from_slice::<AccountJson>(&result).unwrap();
                self.process_update_staking_tickets(project_id, account_id, account_info)
            },
        }
    }

    fn process_update_staking_tickets(&mut self, project_id: ProjectId, account_id: AccountId, staking_account_info: AccountJson) -> bool {
        // Verify staking_info.account_id vs account_id 
        assert_eq!(account_id, staking_account_info.account_id, "The staking account is not equal to current account id.");

        let project = self.projects.get(&project_id);

        if let Some(project) = project {
            // Project's status must be whitelist & current time is between whitelist_start_date and whitelist_end_date
            assert_project_whitelist_period(&project);
            
            // Calculate staking tickets & allocation -> Tier, StakingTicket, Allocations
            // staking_info.lock_balance: The staked amount of token that has been locked by the user.
            // staking_info.unlock_timestamp: The time that user can unlock their locked balance. It is nanosecond unix time.
            let locked_amount: u128 = staking_account_info.lock_balance.into();
            let tier_info = self.internal_get_staking_tier_info(locked_amount as u64, staking_account_info.unlock_timestamp, Some(project.whitelist_start_date.clone()));

            // Get AccountTickets of this project
            let mut accounts_of_current_project = self.project_account_tickets.get(&project_id).unwrap();
            
            // Tickets' information from AccountTickets
            let mut account_tickets = accounts_of_current_project.get(&account_id).unwrap_or_else(|| AccountTickets::default());
            account_tickets.staking_tickets.eligible_tickets = tier_info.no_of_staking_tickets;
            account_tickets.allocations = tier_info.no_of_allocations;
            account_tickets.staking_tier = tier_info.tier;

            // Update Account Tickets
            accounts_of_current_project.insert(&account_id, &account_tickets);

            // Update Project Account Tickets
            self.project_account_tickets.insert(&project_id, &accounts_of_current_project);
        } else {
            return false;
        }

        true
    }
}