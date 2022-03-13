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

fn test_internal_get_staking_tier_info(locked_amount: u128, locked_timestamp: Timestamp, locked_days: u32, 
    expected_staking_tier: StakingTier, expected_staking_tickets: TicketAmount, expected_allocations: TicketAmount) {
    let account_id = bob();

    let mut emulator = Emulator::default();
    emulator.update_context(account_id.clone(), account_id.clone(), 0);

    let account_json = get_account_json(&account_id, locked_amount, locked_timestamp);
    let locked_amount: u128 = account_json.lock_balance.into();
    let locked_amount = locked_amount as u64;
    
    let unlock_timestamp = account_json.unlock_timestamp;

    let current_timestamp = decrease_timestamp(&unlock_timestamp, locked_days.try_into().unwrap(), 0, 0, 0);
    emulator.set_block_timestamp(current_timestamp);

    let tier_info = emulator.contract.internal_get_staking_tier_info(locked_amount, unlock_timestamp, None);

    // TierInfoJson {
    //     tier: Tier1,
    //     locked_amount: locked_amount,
    //     locked_days: locked_days (10),
    //     calculating_time: current_timestamp,
    //     no_of_staking_tickets: 1,
    //     no_of_allocations: 0,
    // }

    assert_eq!(expected_staking_tier, tier_info.tier);
    assert_eq!(U64::from(locked_amount), tier_info.locked_amount);
    assert_eq!(locked_days, tier_info.locked_days);
    assert_eq!(current_timestamp, tier_info.calculating_time);
    assert_eq!(expected_staking_tickets, tier_info.no_of_staking_tickets);
    assert_eq!(expected_allocations, tier_info.no_of_allocations);
}

#[test]
fn test_internal_get_staking_tier_info_0() {
    let locked_amount: u128 = 199_00000000;
    let locked_timestamp: Timestamp = 1647879472091741700;
    let locked_days: u32 = 10;
    let expected_staking_tier = StakingTier::Tier0;
    let expected_staking_tickets: TicketAmount = 0;
    let expected_allocations: TicketAmount = 0;

    test_internal_get_staking_tier_info(locked_amount, locked_timestamp, locked_days, expected_staking_tier, expected_staking_tickets, expected_allocations);
}

#[test]
fn test_internal_get_staking_tier_info_1() {
    let locked_amount: u128 = 200_00000000;
    let locked_timestamp: Timestamp = 1647879472091741700;
    let locked_days: u32 = 10;
    let expected_staking_tier = StakingTier::Tier1;
    let expected_staking_tickets: TicketAmount = 1;
    let expected_allocations: TicketAmount = 0;

    test_internal_get_staking_tier_info(locked_amount, locked_timestamp, locked_days, expected_staking_tier, expected_staking_tickets, expected_allocations);
}

#[test]
fn test_internal_get_staking_tier_info_2() {
    let locked_amount: u128 = 1001_00000000;
    let locked_timestamp: Timestamp = 1647879472091741700;
    let locked_days: u32 = 15;
    let expected_staking_tier = StakingTier::Tier2;
    let expected_staking_tickets: TicketAmount = 12;
    let expected_allocations: TicketAmount = 0;

    test_internal_get_staking_tier_info(locked_amount, locked_timestamp, locked_days, expected_staking_tier, expected_staking_tickets, expected_allocations);
}

#[test]
fn test_internal_get_staking_tier_info_3() {
    let locked_amount: u128 = 5000_00000000;
    let locked_timestamp: Timestamp = 1647879472091741700;
    let locked_days: u32 = 31;
    let expected_staking_tier = StakingTier::Tier3;
    let expected_staking_tickets: TicketAmount = 140;
    let expected_allocations: TicketAmount = 0;

    test_internal_get_staking_tier_info(locked_amount, locked_timestamp, locked_days, expected_staking_tier, expected_staking_tickets, expected_allocations);
}

#[test]
fn test_internal_get_staking_tier_info_4() {
    let locked_amount: u128 = 10100_00000000;
    let locked_timestamp: Timestamp = 1647879472091741700;
    let locked_days: u32 = 15;
    let expected_staking_tier = StakingTier::Tier4;
    let expected_staking_tickets: TicketAmount = 70;
    let expected_allocations: TicketAmount = 1;

    test_internal_get_staking_tier_info(locked_amount, locked_timestamp, locked_days, expected_staking_tier, expected_staking_tickets, expected_allocations);
}

#[test]
fn test_get_staking_tier_info()
{
    let locked_amount: u64 = 1000_00_000_000;
    let locked_timestamp: Timestamp = 1649796512000000000;
    let locked_days: u32 = 10;
    let expected_staking_tier = StakingTier::Tier2;
    let expected_staking_tickets: TicketAmount = 6;
    let expected_allocations: TicketAmount = 0;
    let current_timestamp = decrease_timestamp(&locked_timestamp, locked_days as u16, 0, 0, 0);

    let mut emulator = Emulator::default();
    emulator.set_block_timestamp(current_timestamp);

    let tier_info = emulator.contract.get_staking_tier_info(U64::from(1000_00_000_000), locked_timestamp, None);
    
    assert_eq!(expected_staking_tier, tier_info.tier);
    assert_eq!(U64::from(locked_amount), tier_info.locked_amount);
    assert_eq!(locked_days, tier_info.locked_days);
    assert_eq!(current_timestamp, tier_info.calculating_time);
    assert_eq!(expected_staking_tickets, tier_info.no_of_staking_tickets);
    assert_eq!(expected_allocations, tier_info.no_of_allocations);
}