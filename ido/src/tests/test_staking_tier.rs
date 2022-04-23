use crate::tests::test_utils::*;
use crate::tests::test_emulator::*;
use crate::tests::test_project::*;

use crate::*;

pub(crate) fn get_sample_account_json(account_id: &AccountId) -> AccountJson {
    get_account_json(&account_id, 500_00000000, 1647879472091741700)
}

pub(crate) fn get_account_json(account_id: &AccountId, locked_balance: u128, locked_timestamp: Timestamp) -> AccountJson {
    AccountJson {
        account_id: account_id.clone(),
        lock_balance: U128::from(locked_balance),
        unlock_timestamp: locked_timestamp,
        stake_balance: U128::from(locked_balance),
        unstake_balance: U128::from(0),
        reward: U128::from(0),
        can_withdraw: true,
        start_unstake_timestamp: 0,
        unstake_available_epoch: 0,
        current_epoch: 980,
    }
}
