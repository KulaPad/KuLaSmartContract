use crate::*;
use near_sdk::PromiseResult;

#[ext_contract(ext_staking_contract)]
pub trait StakingContract{
    fn get_user_point(&self, account_id: AccountId) -> U64;
}

#[ext_contract(ext_self)]
pub trait IDOContractResolver{
    fn resolve_get_account_point_for_register_whitelist(&self, account_id: AccountId, project_id: ProjectId, xtoken: u128) -> bool;
}

#[near_bindgen]
impl IDOContract{
    pub(crate) fn internal_register_fixed_xtoken_project(&self, project_id: ProjectId, account_id: AccountId, xtoken: u128)-> PromiseOrValue<bool>{
        env::log(format!("register_fixed_xtoken_project(account_id: '{}', project_id: '{}')", account_id, project_id).as_bytes());
        ext_staking_contract::get_user_point(
            account_id.clone(),
            &self.staking_contract_id,
            NO_DEPOSIT, 
            GAS_FUNCTION_CALL
        ).then (
            ext_self::resolve_get_account_point_for_register_whitelist(
                account_id, 
                project_id, 
                xtoken,
                &env::current_account_id(), 
                NO_DEPOSIT, 
                GAS_FUNCTION_CALL_GET_USER_POINT
            )
        ).into()
    }


    pub(crate) fn resolve_get_account_point_for_register_whitelist(&mut self, account_id: AccountId, project_id: ProjectId, xtoken: u128) -> bool{
        assert_eq!(
            env::promise_results_count(),
            1,
            "This is a callback method"
        );

        match env::promise_result(0){
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed =>false,
            PromiseResult::Successful(result) => {
                let point = near_sdk::serde_json::from_slice::<U64>(&result).unwrap();
                env::log(format!("Prepared gas: {}, Used gas: {}",env::prepaid_gas(),env::used_gas()).as_bytes());
                let result = self.proccess_register_whitelist(account_id, project_id,point, xtoken);
                env::log(format!("Prepared gas: {}, Used gas: {}",env::prepaid_gas(),env::used_gas()).as_bytes());

                result
            }   
        }
    }

    pub (crate) fn proccess_register_whitelist(&mut self, account_id: AccountId, project_id: ProjectId, point: U64, xtoken: u128) -> bool{
        env::log(format!("proccess_register_whitelist(account_id: {}, project_id: {}, point: {})", account_id, project_id,point.0).as_bytes());
        assert!((point.0 as u128) >= xtoken,"User don't have enough XToken");
        self.internal_add_account(&account_id, project_id);

        true
    }
}