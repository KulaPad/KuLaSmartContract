use crate::tests::test_utils::*;
use crate::tests::test_emulator::*;
use crate::structures::project::*;
use crate::tests::test_project::*;
use crate::staking_contract::*;
use crate::structures::staking::*;

use near_sdk::{env, AccountId};
use near_sdk::json_types::{U128, U64};

fn get_sample_account_json(account_id: &AccountId) -> AccountJson {
    let account_json = AccountJson {
        account_id: account_id.clone(),
        lock_balance: U128::from(500_00000000),
        unlock_timestamp: 1647879472091741700,
        stake_balance: U128::from(1100_00000000),
        unstake_balance: U128::from(0),
        reward: U128::from(12058904),
        can_withdraw: true,
        start_unstake_timestamp: 0,
        unstake_available_epoch: 0,
        current_epoch: 980,
    };

    account_json
}

#[test]
fn test_happy_case() {
    let mut emulator = Emulator::default();

    // Whitelist start date:    2022-01-01 07:00:00 - 1640995200000000000
    // Whitelist end date:      2022-01-04 06:00:00 - 1641250800000000000
    // Sales start date:        2022-01-04 07:00:00 - 1641254400000000000
    // Sales end date:          2022-01-05 07:00:00 - 1641340800000000000
    // Status:                  Preparation
    let whitelist_start_date = 1640995200000000000;
    let whitelist_end_date = 1641250800000000000;
    let sale_start_date = 1641254400000000000;
    let sale_end_date = 1641340800000000000;
    let status = ProjectStatus::Preparation;

    let valid_whitelist_time = increase_timestamp(&whitelist_start_date, 1, 0, 0, 0);
    let out_of_whitelist_time = increase_timestamp(&whitelist_end_date, 0, 0, 10, 0);
    let valid_sales_time = increase_timestamp(&sale_start_date, 0, 0, 10, 0);

    let mut project = get_project_1();
    project.whitelist_start_date = whitelist_start_date;
    project.whitelist_end_date = whitelist_end_date;
    project.sale_start_date = sale_start_date;
    project.sale_end_date = sale_end_date;
    project.status = status.clone();

    env::log(format!("Before create a project").as_bytes());

    // Create a new project with status of Preparation
    let project_id = emulator.contract.create_project(project);

    let created_project = emulator.contract.get_project(project_id);
    if let Some(created_project) = created_project {
        assert_eq!(project_id, created_project.id);
        assert_eq!(whitelist_start_date, created_project.whitelist_start_date);
        assert_eq!(whitelist_end_date, created_project.whitelist_end_date);
        assert_eq!(sale_start_date, created_project.sale_start_date);
        assert_eq!(sale_end_date, created_project.sale_end_date);
        assert_eq!(status, created_project.status);
    } else {
        panic!("Cannot get project after created.");
    }
    
    // Change project's status to Whitelist
    emulator.set_block_timestamp(valid_whitelist_time);
    assert_eq!(valid_whitelist_time, emulator.context.block_timestamp);

    emulator.contract.change_project_status(project_id);
    let created_project = emulator.contract.get_project(project_id).unwrap();
    assert_eq!(ProjectStatus::Whitelist, created_project.status);

    // User A registers whitelist
    let account_a = bob();
    emulator.set_account_id_and_desposit(account_a.clone(), account_a.clone(), 0);
    emulator.contract.register_whitelist(project_id);
    println!("User A registers whitelist");
    assert!(emulator.contract.is_whitelist(project_id));

    // User B registers whitelist

    // User C do not register whitelist
    let account_c = alice();
    emulator.set_account_id_and_desposit(account_c.clone(), account_c.clone(), 0);
    println!("User C registers whitelist");
    assert!(!emulator.contract.is_whitelist(project_id));

    // User A stakes & locks Tier1 for 31 days => Cross contract call

    // User A updated staking tier => Cross contract call
    let account_json = get_sample_account_json(&account_a);

    emulator.contract.process_update_staking_tickets(project_id, account_a.clone(), account_json);

    // Close whitelist

    // User A deposit fund

}

#[test]
fn test_internal_get_staking_tier_info() {
    let account_id = bob();

    let mut emulator = Emulator::default();
    emulator.update_context(account_id.clone(), account_id.clone(), 0);
    
    let account_json = get_sample_account_json(&account_id);
    let locked_amount: u128 = account_json.lock_balance.into();
    let locked_amount = locked_amount as u64;
    let locked_days: u32 = 10;
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

    assert_eq!(StakingTier::Tier1, tier_info.tier);
    assert_eq!(U64::from(locked_amount), tier_info.locked_amount);
    assert_eq!(locked_days, tier_info.locked_days);
    assert_eq!(current_timestamp, tier_info.calculating_time);
    assert_eq!(1, tier_info.no_of_staking_tickets);
    assert_eq!(0, tier_info.no_of_allocations);
}