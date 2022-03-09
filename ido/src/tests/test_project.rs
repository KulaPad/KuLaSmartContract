use crate::tests::test_utils::*;
use crate::tests::test_emulator::*;
use crate::structures::project::*;

fn get_project_1() -> ProjectInfo {
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
        current_ticket_id: 0,
    }
}

#[test]
fn test_create_and_get_project() {
    let mut emulator = Emulator::default();

    let project = get_project_1();
    let project_name = project.name.clone();

    emulator.contract.create_project(project);

    let projects = emulator.contract.get_projects(None, None, None); 

    assert_eq!(1, projects.len(), "The number of projects in the contract is not correct!");

    let json_project = &projects[0];

    assert_eq!(1, json_project.id, "The created project id must equal to 1.");
    assert_eq!(project_name, json_project.name, "The project name must be the same.");

    let new_projects = emulator.contract.get_projects(Some(ProjectStatus::Proposed), None, None); 
    let approved_projects = emulator.contract.get_projects(Some(ProjectStatus::Approved), None, None); 

    assert_eq!(1, new_projects.len(), "The number of NEW projects in the contract is not correct!");
    assert_eq!(0, approved_projects.len(), "The number of APPROVED projects in the contract is not correct!");
}

#[test]
fn test_create_sample_projects() {
    let mut emulator = Emulator::default();

    emulator.contract.create_sample_projects();

    let projects = emulator.contract.get_projects(None, None, None); 

    assert_eq!(10, projects.len(), "The number of projects in the contract is not correct!");
}