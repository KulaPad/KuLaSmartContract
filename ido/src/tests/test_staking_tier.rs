use crate::tests::test_utils::*;
use crate::tests::test_emulator::*;
use crate::tests::test_project::*;

use crate::*;

pub(crate) fn get_sample_account_json(account_id: &AccountId) -> AccountJson {
    get_account_json(&account_id, 500_00000000, 30, 1647879472091741700)
}

pub(crate) fn get_account_json(account_id: &AccountId, locked_balance: u128, locked_days: u32, locked_timestamp: Timestamp) -> AccountJson {
    AccountJson {
        account_id: account_id.clone(),
        locked_balance: U128::from(locked_balance),
        locked_days: locked_days,
        unlocked_timestamp: locked_timestamp,
        staked_balance: U128::from(locked_balance),
        unstaked_balance: U128::from(0),
        reward: U128::from(0),
        can_withdraw: true,
        start_unstake_timestamp: 0,
        unstake_available_epoch: 0,
        current_epoch: 980,
        tier: Tier::Tier1,
        point: U128(500)
    }
}
