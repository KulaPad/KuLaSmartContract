use crate::tests::test_utils::*;
use crate::tests::test_emulator::*;
use crate::tests::test_project::*;
use crate::tests::test_staking_tier::*;

use crate::*;

use near_sdk::{env, AccountId, Timestamp};
use near_sdk::json_types::{U128, U64};

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
    let before_whitelist_time = decrease_timestamp(&whitelist_start_date, 1, 0, 0, 0);
    let out_of_whitelist_time = increase_timestamp(&whitelist_end_date, 0, 0, 10, 0);
    let valid_sales_time = increase_timestamp(&sale_start_date, 0, 0, 10, 0);

    let mut project = get_project_1();
    project.whitelist_start_date = whitelist_start_date;
    project.whitelist_end_date = whitelist_end_date;
    project.sale_start_date = sale_start_date;
    project.sale_end_date = sale_end_date;

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
    emulator.set_block_timestamp(before_whitelist_time);
    assert_eq!(before_whitelist_time, emulator.context.block_timestamp);

    emulator.set_account_id_and_desposit(owner(), owner(), 0);
    emulator.contract.update_project_whitelist_date(project_id, None, None);

    emulator.contract.change_project_status(project_id);

    let created_project = emulator.contract.get_project(project_id).unwrap();
    assert_eq!(ProjectStatus::Whitelist, created_project.status);

    // User A registers whitelist
    emulator.set_account_id_and_desposit(account_a(), account_a(), 0);
    emulator.contract.register_whitelist(project_id);
    println!("User A registers whitelist - {}", account_a());
    assert!(emulator.contract.is_whitelist(project_id, None));

    let projects_in_account = emulator.contract.projects_by_account.get(&account_a()).unwrap();
    let accounts_and_tickets_in_project = emulator.contract.accounts_by_project.get(&project_id).unwrap();

    assert!(projects_in_account.contains(&project_id));
    assert_eq!(1, accounts_and_tickets_in_project.len() as u32);

    let project = emulator.contract.get_project(project_id).unwrap();
    assert_eq!(1, project.whitelist_accounts);

    // User B registers whitelist

    // User C do not register whitelist
    let account_c = alice();
    emulator.set_account_id_and_desposit(account_c.clone(), account_c.clone(), 0);
    println!("User C registers whitelist - {}", account_c);
    assert!(!emulator.contract.is_whitelist(project_id, None));

    // User A stakes & locks Tier1 for 31 days => Cross contract call

    // User A updated staking tier => Cross contract call
    let locked_amount: u128 = 200_00000000;
    let locked_days: u16 = 10;
    let locked_timestamp: Timestamp = increase_timestamp(&whitelist_start_date, locked_days, 0, 0, 0);
    let expected_staking_tier = StakingTier::Tier1;
    let expected_staking_tickets: TicketNumber = 1;
    let expected_allocations: TicketNumber = 0;

    let account_json = get_account_json(&account_a(), locked_amount, locked_timestamp);

    emulator.contract.process_update_staking_tickets(project_id, account_a(), account_json);

    // Validate stored data
    let project = emulator.contract.projects.get(&project_id).unwrap();
    println!("Staking tier -> Total tickets");
    
    
    let account_tickets = emulator.contract.accounts_by_project.get(&project_id).unwrap();
    let tickets = account_tickets.get(&account_a()).unwrap();

    println!("Staking tier -> Account Staking Tickets");
    

    println!("Staking tier -> Account Staking Tickets - Allocations");


    // Validate response data
    emulator.set_account_id_and_desposit(account_a(), account_a(), 0);
    println!("Get project account info for Account A - {}", account_a());

    // Return data
    // ProjectAccountInfoJson {
    //     account_id: "bob",
    //     project_id: 1,
    //     project_status: Whitelist,
    //     whitelist_info: Some(
    //         ProjectWhitelistInfo {
    //             tier: Tier1,
    //             no_of_staking_tickets: 1,
    //             no_of_social_tickets: 0,
    //             no_of_referral_tickets: 0,
    //             no_of_allocations: 0,
    //         },
    //     ),
    //     sale_info: None,
    // }

    // Close whitelist
    emulator.set_account_id_and_desposit(owner(), owner(), 0);
    emulator.contract.update_project_sales_date(project_id);

    println!("update_project_sales_date");

    emulator.contract.close_project_whitelist(project_id);

    println!("close_project_whitelist");

    let project = emulator.contract.projects.get(&project_id).unwrap();
    assert_eq!(ProjectStatus::Sales, project.status);

    // User A deposit fund
    
    
    //assert!(false);
}

#[test]
fn test_project_ticket() {
    let mut emulator = Emulator::default();
    let project = get_project_1();
    let project_id = emulator.contract.create_project(project);

    println!("Start testing...");


    //assert!(false);
}

#[test]
fn test_rate() {
    let mut emulator = Emulator::default();
    let rate = Rate::new(1, 100);

    assert_eq!(1000, rate.multiply(100000));
    assert_eq!(100000, rate.devided_by(1000));
}