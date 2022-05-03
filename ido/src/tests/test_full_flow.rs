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
    assert!(emulator.contract.is_whitelist(project_id));

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
    assert!(!emulator.contract.is_whitelist(project_id));

    // User A stakes & locks Tier1 for 31 days => Cross contract call

    // User A updated staking tier => Cross contract call
    let locked_amount: u128 = 200_00000000;
    let locked_days: u16 = 10;
    let locked_timestamp: Timestamp = increase_timestamp(&whitelist_start_date, locked_days, 0, 0, 0);
    let expected_staking_tier = Tier::Tier1;
    let expected_staking_tickets: TicketAmount = 1;
    let expected_allocations: TicketAmount = 0;

    let account_json = get_account_json(&account_a(), locked_amount, locked_timestamp);

    emulator.contract.process_update_staking_tickets(project_id, account_a(), account_json);

    // Validate stored data
    let project = emulator.contract.projects.get(&project_id).unwrap();
    println!("Staking tier -> Total tickets");
    assert_eq!(expected_staking_tickets as u64, project.total_staking_tickets);
    assert_eq!(expected_allocations, project.total_allocations);
    
    let account_tickets = emulator.contract.accounts_by_project.get(&project_id).unwrap();
    let tickets = account_tickets.get(&account_a()).unwrap();

    println!("Staking tier -> Account Staking Tickets");
    assert_eq!(expected_staking_tier, tickets.staking_tier);
    assert_eq!(expected_staking_tickets, tickets.staking_tickets.eligible_tickets);
    assert_eq!(expected_staking_tickets, tickets.staking_tickets.deposit_tickets);
    assert_eq!(expected_staking_tickets, tickets.staking_tickets.ticket_ids.len() as u32);
    assert_eq!(expected_staking_tickets, tickets.staking_tickets.win_ticket_ids.len() as u32);

    println!("Staking tier -> Account Staking Tickets - Allocations");

    assert_eq!(expected_allocations, tickets.allocations);
    assert_eq!(expected_allocations, tickets.deposit_allocations);

    let ticket_number = tickets.staking_tickets.ticket_ids[0];
    let ticket_id = build_ticket_id(TicketType::Staking, ticket_number);

    println!("Staking tier -> Project Ticket");
    let tickets_by_project = emulator.contract.tickets_by_project.get(&project_id);
    println!("Staking tier -> Project Ticket - Achieved object");
    if let Some(tickets_by_project) = tickets_by_project {
        let keys = tickets_by_project.keys_as_vector();
        let values = tickets_by_project.values_as_vector();
        println!("Staking tier -> Project Ticket - Ticket Owner - Len: {} - [({},{})]", &tickets_by_project.len(), 0, 0);
        
        // let ticket_owner = tickets_by_project.get(&ticket_id);
        // if let Some(ticket_owner) = ticket_owner {
        //     println!("Staking tier -> Project Ticket - Assert");
        //     assert_eq!(account_a, ticket_owner);
        // } else {
        //     panic!("Cannot get ticket owner.");
        // }
    } else {
        panic!("Cannot get project ticket.");
    }

    // Validate response data
    emulator.set_account_id_and_desposit(account_a(), account_a(), 0);
    println!("Get project account info for Account A - {}", account_a());
    let staking_tier_info: ProjectAccountInfoJson = emulator.contract.get_project_account_info(project_id, account_a());
    println!("Whitelist open - {:#?}", staking_tier_info);

    assert_eq!(account_a(), staking_tier_info.account_id);
    assert_eq!(project_id, staking_tier_info.project_id);
    assert_eq!(project.status, staking_tier_info.project_status);

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

    let staking_tier_info: ProjectAccountInfoJson = emulator.contract.get_project_account_info(project_id, account_a());
    println!("Whitelist closed - {:#?}", staking_tier_info);

    // User A deposit fund
    let expected_deposit_amount: Balance = project.token_sale_rate.multiply(ONE_NEAR) 
        * project.token_amount_per_sale_slot as u128
        * (expected_staking_tickets + expected_allocations) as u128;
    let actual_deposit_amount: Balance = emulator.contract.calculate_must_attach_deposit_amount_by_account_id(&account_a(), project_id).into();

    assert_eq!(expected_deposit_amount, actual_deposit_amount);

    emulator.set_account_id_and_desposit(account_a(), account_a(), expected_deposit_amount);
    emulator.contract.commit(project_id);

    // Assert Project
    let project = emulator.contract.projects.get(&project_id).unwrap();
    assert_eq!(expected_deposit_amount, project.total_fund_committed);
    
    // Assert Project Account Token Sales
    let actual_account_token_sales = emulator.contract.unwrap_project_account_token_sales(project_id, &account_a()).unwrap();
    assert_eq!(expected_deposit_amount, actual_account_token_sales.funding_amount);

    let staking_tier_info: ProjectAccountInfoJson = emulator.contract.get_project_account_info(project_id, account_a());
    println!("Sales period - Deposit - {:#?}", staking_tier_info);
    
    //assert!(false);
}

#[test]
fn test_project_ticket() {
    let mut emulator = Emulator::default();
    let project = get_project_1();
    let project_id = emulator.contract.create_project(project);

    println!("Start testing...");

    let mut project_ticket = emulator.contract.tickets_by_project.get(&project_id).unwrap();
    assert_eq!(0, project_ticket.len() as u32);

    project_ticket.insert(&"L1".to_string(), &"Account_1".to_string());
    assert_eq!(1, project_ticket.len() as u32);

    // Update
    emulator.contract.tickets_by_project.insert(&project_id, &project_ticket);

    // Reload
    let project_ticket = emulator.contract.tickets_by_project.get(&project_id).unwrap();
    let keys = project_ticket.keys_as_vector().to_vec();
    let values = project_ticket.values_as_vector().to_vec();

    println!("Key: {}, Value: {}", keys[0], values[0]);

    //assert!(false);
}


#[test]
fn test_key_storage() {
    let mut emulator = Emulator::default();

    println!("{:?}", StorageKey::ProjectTicketInnerKey(1).try_to_vec().unwrap());
    println!("{:?}", StorageKey::ProjectTicketInnerKey(2).try_to_vec().unwrap());
    println!("{:?}", StorageKey::ProjectTicketInnerKey(3).try_to_vec().unwrap());

    println!("");

    println!("{:?}", StorageKey::ProjectAccountTicketInnerKey(1).try_to_vec().unwrap());
    println!("{:?}", StorageKey::ProjectAccountTicketInnerKey(2).try_to_vec().unwrap());
    println!("{:?}", StorageKey::ProjectAccountTicketInnerKey(3).try_to_vec().unwrap());
    println!("{:?}", StorageKey::ProjectAccountTicketInnerKey(u64::MAX).try_to_vec().unwrap());

    assert!(false);
}

#[test]
fn test_rate() {
    let mut emulator = Emulator::default();
    let rate = Rate::new(1, 100);

    assert_eq!(1000, rate.multiply(100000));
    assert_eq!(100000, rate.devided_by(1000));
}