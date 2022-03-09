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

        let contract = IDOContract::new(owner);

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
}
