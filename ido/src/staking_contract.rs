use crate::*;

#[derive(Serialize, Deserialize, Debug)]
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
            GAS_FUNCTION_CALL_UPDATE_STAKING_TIER
        )).into()
    }

    #[private]
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
                
                env::log(format!("Prepared gas: {}, Used gas: {}", env::prepaid_gas(), env::used_gas()).as_bytes());

                let result = self.process_update_staking_tickets(project_id, account_id, account_info);

                env::log(format!("Prepared gas: {}, Used gas: {}", env::prepaid_gas(), env::used_gas()).as_bytes());

                result
            },
        }
    }

    pub(crate) fn process_update_staking_tickets(&mut self, project_id: ProjectId, account_id: AccountId, staking_account_info: AccountJson) -> bool {
        env::log(format!("process_update_staking_tickets(project_id: {}, account_id: {}, staking_info: {:#?})", project_id, account_id, staking_account_info).as_bytes());

        // Verify staking_info.account_id vs account_id 
        assert_eq!(account_id.clone(), staking_account_info.account_id, "The staking account is not equal to current account id.");

        let project = self.internal_get_project_or_panic(project_id);

        // Project's status must be whitelist & current time is between whitelist_start_date and whitelist_end_date
        project.assert_whitelist_period();
        
        // TODO: Generate tickets
        
        // Update project
        self.projects.insert(&project_id, &project);

        true
    }
}