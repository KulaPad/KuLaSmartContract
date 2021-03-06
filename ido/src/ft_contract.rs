use near_sdk::Gas;

use crate::*;

pub const FT_TRANSFER_GAS: Gas = 10_000_000_000_000;
pub const TRANSFER_CALLBACK_GAS: Gas = 10_000_000_000_000;

pub const DEPOSIT_ONE_YOCTOR: Balance = 1;

#[ext_contract(ext_ft_contract)]
pub trait FungibleTokenCore {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(ext_self)]
pub trait ExtStakingContract {
    fn ft_transfer_callback(&mut self, project_id: ProjectId, account_id: AccountId, claim_amount: U128);
}

// Firstly, user must call ft_transfer_call function from ft contract.
// Ft contract will send ft_on_transfer function to ido_contract
// This function will get msg from ft_transfer_call, handle it for getting deposit_amount, and do commit sale
// Example of msg will be: "project_id":1
pub trait IDOContractResolver{
    fn ft_on_transfer(&mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String
        )-> PromiseOrValue<U128>;
}

#[near_bindgen]
impl IDOContract {
    /// User can claim their bought unlocked token after sales.
    #[payable]
    pub fn claim(&mut self, project_id: ProjectId) -> Promise {
        let account_id: AccountId = env::predecessor_account_id();
        let project = self.internal_get_project_or_panic(project_id);
        assert!(project.is_in_distribution_period(), "The project isn't in distribution period.");

        assert_one_yocto();
        let claim_amount = self.internal_claim(project_id, &account_id);

        // TODO: Update the claimed amount in the project_account before starting to transfer token to user.
        // If the cross-call transaction failed, it's need to roll back the data that was updated.
        
        // handle transfer withdraw
        ext_ft_contract::ft_transfer(
            account_id.clone(), 
            U128(claim_amount), 
            Some(format!("Claim the amount of {} on contract {} from IDO Contract", claim_amount, project.token_contract_id)), 
            &project.token_contract_id, 
            DEPOSIT_ONE_YOCTOR, 
            FT_TRANSFER_GAS
        ).then(
            ext_self::ft_transfer_callback(
                project_id,
                account_id.clone(),
                U128(claim_amount),
                &env::current_account_id(),
                NO_DEPOSIT, 
                TRANSFER_CALLBACK_GAS
            )
        )
    }

    #[private]
    pub fn ft_transfer_callback(&mut self, project_id: ProjectId, account_id: AccountId, claim_amount: U128) -> U128 {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_value) => {
                let mut project_account_unordered_map = self.internal_get_accounts_by_project_or_panic(project_id);
                let mut project_account_1 = self.internal_get_account_by_project_or_panic(project_id,&account_id);
                let mut project_account_2 = self.internal_get_account_by_project_or_panic(project_id,&account_id);
                let account_distribution = project_account_1.distribution_data;
                if let Some(mut account_distribution) = account_distribution{
                    account_distribution.claimed_amount += claim_amount.0;
                    project_account_1.distribution_data = Some(account_distribution);
                    project_account_unordered_map.insert(&account_id,&project_account_1);
                    self.accounts_by_project.insert(&project_id,&project_account_unordered_map);
                } else {
                    let mut account_sale = project_account_2.sale_data.unwrap();
                    account_sale.committed_amount = 0;
                    project_account_2.sale_data = Some(account_sale);
                    project_account_unordered_map.insert(&account_id,&project_account_2);
                    self.accounts_by_project.insert(&project_id,&project_account_unordered_map);
                }
                
                U128::from(0)
            },
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }

    pub(crate) fn internal_claim(&mut self, project_id: ProjectId, account_id: &AccountId) -> Balance{
        // Get deposit token amount and project_ft_contract_id
        let project_account = self.internal_get_account_by_project_or_panic(project_id,&account_id);
        let sale_data = project_account.sale_data.unwrap();
        let distribution_data = project_account.distribution_data;
        // Calculate token to transfer for user
        let claim_amount = if let Some(distribution_data) = distribution_data {
            distribution_data.unlocked_amount - distribution_data.claimed_amount
        } else {
            sale_data.committed_amount
        };
        
        claim_amount 
    }

    pub fn ft_on_transfer(&mut self,sender_id: AccountId,amount: U128,msg: String)-> PromiseOrValue<U128>{
        let args: Vec<&str> = msg.split(":").collect();
        if args.len() >= 1 {
            match args[0] {
                "project_id" => {
                    let project_id : ProjectId = args[1].trim().parse::<u64>().unwrap();
                    let project = self.internal_get_project_or_panic(project_id);
                    let fund_contract_id = env::predecessor_account_id();
                    if project.fund_contract_id == fund_contract_id{ 
                        env::log(format!("Ft on transfer success: project_id={},sender_id={},amount={},fund_contract_id={}", project_id, sender_id, amount.0,env::predecessor_account_id()).as_bytes());
                        let committed = self.internal_commit(project_id, &sender_id, amount.0);
                        return PromiseOrValue::Value(U128(amount.0 - committed));
                    } else {
                        env::log(b"Transfer Error: fund_contract_id not match. Transfer back deposited token to signer");
                        return PromiseOrValue::Value(amount);
                    }
                },
                _ => {
                    env::log(b"Transfer Error: Unknown message sent");
                    return PromiseOrValue::Value(amount);
                }
            }
        }
         
        
        PromiseOrValue::Value(U128(0))
    }
}