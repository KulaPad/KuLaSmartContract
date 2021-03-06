use near_sdk::{AccountId, MockedBlockchain, PromiseResult, VMContext, testing_env};
use near_sdk::{Balance, BlockHeight, EpochHeight};

use crate::*;
use crate::tests::test_utils::*;

pub struct Emulator {
    pub contract: IDOContract,
    pub context: VMContext,
}

impl Emulator {
    pub fn new(owner: String) -> Self {
        let context = VMContextBuilder::new()
            .current_account_id(owner.clone())
            .finish();

        testing_env!(context.clone());

        let contract = IDOContract::new(owner, ft_token_id(), None, None, None);

        Emulator {
            contract,
            context,
        }
    }

    pub fn default() -> Self {
        Emulator::new(owner().clone())
    }

    pub fn update_context(&mut self, predecessor_account_id: String,signer_account_id:String, deposit: Balance) {
        self.context = VMContextBuilder::new()
            .current_account_id(staking())
            .predecessor_account_id(predecessor_account_id)
            .signer_account_id(signer_account_id)
            .attached_deposit(deposit)
            .finish();
        testing_env!(self.context.clone());
        println!(
            "Print something here"
        );
    }

    pub fn set_block_timestamp(&mut self, timestamp: Timestamp) {
        self.context.block_timestamp = timestamp;

        println!("block_timestamp: {}", timestamp);

        testing_env!(self.context.clone());
    }

    /// Keep the remaining properties the same.
    pub fn set_account_id_and_desposit(&mut self, predecessor_account_id: String, signer_account_id:String, deposit: Balance) {
        self.context.predecessor_account_id = predecessor_account_id.clone();
        self.context.signer_account_id = signer_account_id.clone();
        self.context.attached_deposit = deposit;
        
        println!("predecessor_account_id: {}, signer_account_id: {}, attached_deposit: {}", predecessor_account_id, signer_account_id, deposit);

        testing_env!(self.context.clone());
    }

    pub fn update_account_sale_ticket(&mut self, account_id: AccountId, project_id: ProjectId, ticket_nums: u64){
        self.contract.assert_test_mode_and_owner();
        let mut project_account_unordered_map = self.contract.internal_get_accounts_by_project_or_panic(project_id);
        let account_sale = AccountSale{
            committed_amount: 0,
            sale_data: AccountSaleData::Lottery(
                LotteryAccountSaleData{
                    eligible_tickets: ticket_nums,
                    deposit_tickets: 0,
                    ticket_ids: vec![],
                    win_ticket_ids: vec![]
                }
            )
        };

        let project_account = ProjectAccount{
            sale_data: Some(account_sale),
            distribution_data: None
        };

        project_account_unordered_map.insert(&account_id, &project_account);
        self.contract.accounts_by_project.insert(&project_id,&project_account_unordered_map);   
    }
}
