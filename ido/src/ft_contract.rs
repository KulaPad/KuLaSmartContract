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

#[near_bindgen]
impl IDOContract {
    /// User can claim their bought unlocked token after sales.
    #[payable]
    pub fn claim(&mut self, project_id: ProjectId) -> Promise {
        let account_id: AccountId = env::predecessor_account_id();
        let project = self.get_project_or_panic(project_id);
        assert!(project.is_in_distribution_period(), "The project isn't in distribution period.");

        assert_one_yocto();

        let claim_amount = self.internal_claim(project_id, &account_id);

        // handle transfer withdraw
        ext_ft_contract::ft_transfer(
            account_id.clone(), 
            U128(claim_amount), 
            Some(format!("Claim {} {} from IDO Contract", claim_amount, project.token_symbol)), 
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
                // let upgradable_account: UpgradableAccount = self.accounts.get(&account_id).unwrap();
                // let mut account: Account = Account::from(upgradable_account);

                // // update account data
                // account.pre_reward = 0;
                // account.last_block_balance_change = env::block_index();

                // self.accounts.insert(&account_id, &UpgradableAccount::from(account));
                // self.total_paid_reward_balance += amount.0;

                // amount
                U128::from(0)
            },
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }

    pub(crate) fn internal_claim(&mut self, project_id: ProjectId, account_id: &AccountId) -> Balance{
        // Get deposit near amount

        // Calculate token to transfer for user

        // 
        
        0
    }
}