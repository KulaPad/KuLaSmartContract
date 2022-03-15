use crate::tests::test_utils::*;
use crate::tests::test_emulator::*;
use crate::structures::project::*;
use crate::*;

pub(crate) fn get_project_1() -> ProjectInfo {
    ProjectInfo {
        owner_id: "your.testnet".to_string(),
        name: "YOUR".to_string(),
        logo_url: "https://your.com/logo.url".to_string(),
        description: "The description of project".to_string(),
        introduction: "The introduction about project".to_string(),
        categories: vec!["NFT".to_string(), "Metaverse".to_string(), "Datamining".to_string()],
        whitelist_start_date: 1,
        whitelist_end_date: 2,
        sale_start_date: 3,
        sale_end_date: 5,
        token_symbol: "YOUR".to_string(),
        token_contract_id: "your.testnet".to_string(),
        fund_symbol: "NEAR".to_string(),
        fund_contract_id: None,
        token_decimal: 8,
        token_amount_per_sale_slot: 100,
        token_raised_amount: 40000,
        token_sale_rate: Rate::new(10u64, 1u64),
        total_fund_received: 0,
        sale_type: SaleType::FullUnlocked,
        status: ProjectStatus::Proposed,
        configuration: ProjectConfiguration {
            max_staking_tickets_per_user: 300,
            max_win_tickets_per_user: 10,
        },
        total_allocations: 0,
        total_staking_tickets: 0,
        total_social_tickets: 0,
        total_referral_tickets: 0,
    }
}

#[test]
fn test_create_and_get_project() {
    let mut emulator = Emulator::default();

    let project_info = get_project_1();
    emulator.contract.create_project(get_project_1());
    let projects = emulator.contract.get_projects(None, None, None); 

    assert_eq!(1, projects.len(), "The number of projects in the contract is not correct!");

    let json_project = &projects[0];

    assert_eq!(1, json_project.id, "The created project id must equal to 1.");
    assert_eq!(project_info.name, json_project.name, "The project name must be the same.");

    let new_projects = emulator.contract.get_projects(Some(ProjectStatus::Proposed), None, None); 
    let approved_projects = emulator.contract.get_projects(Some(ProjectStatus::Approved), None, None); 

    assert_eq!(1, new_projects.len(), "The number of NEW projects in the contract is not correct!");
    assert_eq!(0, approved_projects.len(), "The number of APPROVED projects in the contract is not correct!");
}

#[test]
fn test_create_sample_projects() {
    let mut emulator = Emulator::default();

    emulator.contract.create_sample_projects();

    // let projects = emulator.contract.get_projects(None, None, None); 

    // assert_eq!(10, projects.len(), "The number of projects in the contract is not correct!");
}

#[test]
fn test_update_project_sales_date_to_end() {
    let mut emulator = Emulator::default();

    emulator.contract.create_sample_projects();

    let whitelist_start_date = 1640995200000000000;
    let whitelist_end_date = 1641250800000000000;
    let sale_start_date = 1641254400000000000;
    let sale_end_date = 1641340800000000000;
    let status = ProjectStatus::Preparation;

    let mut project = get_project_1();
    project.whitelist_start_date = whitelist_start_date;
    project.whitelist_end_date = whitelist_end_date;
    project.sale_start_date = sale_start_date;
    project.sale_end_date = sale_end_date;
    project.status = status.clone();

    let project_id = emulator.contract.create_project(project);
    let project = emulator.contract.get_project_or_panic(project_id);

    // Preparation
    assert_eq!(ProjectStatus::Preparation, project.status);

    // Whitelist
    let before_whitelist_time = decrease_timestamp(&whitelist_start_date, 1, 0, 0, 0);
    emulator.set_block_timestamp(before_whitelist_time);
    emulator.set_account_id_and_desposit(owner(), owner(), 0);
    emulator.contract.update_project_whitelist_date(project_id, None, None);

    println!("Project's status: {:?}, Current Time: {}, Whitelist Start Time: {}", project.status, before_whitelist_time, whitelist_start_date);
    emulator.contract.change_project_status(project_id);
    
    let project = emulator.contract.get_project_or_panic(project_id);
    assert_eq!(ProjectStatus::Whitelist, project.status);

    // Sales
    emulator.contract.update_project_sales_date(project_id);
    emulator.contract.change_project_status(project_id);

    let project = emulator.contract.get_project_or_panic(project_id);
    assert_eq!(ProjectStatus::Sales, project.status);

    // Distribution
    emulator.contract.update_project_sales_date_to_end(project_id);
    emulator.contract.change_project_status(project_id);

    let project = emulator.contract.get_project_or_panic(project_id);
    assert_eq!(ProjectStatus::Distribution, project.status);
}