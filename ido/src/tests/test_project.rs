use crate::tests::test_utils::*;
use crate::tests::test_emulator::*;
use crate::modules::project::*;
use crate::*;

pub(crate) fn get_project_1() -> ProjectInput {
    ProjectInput {
        owner_id: "your.testnet".to_string(),
        whitelist_start_date: 1,
        whitelist_end_date: 2,
        sale_start_date: 3,
        sale_end_date: 5,
        token_contract_id: "your.testnet".to_string(),
        fund_contract_id: "usdc.testnet".to_string(),
        token_raised_amount: U128(40000),
        token_sale_rate_numberator: 10u64,
        token_sale_rate_denominator: 1u64,
        whitelist_type: WhitelistType::None,
        sale_type: SaleType::Shared {
            min_allocation_per_user: 50,
            max_allocation_per_user: 100,
        },
        distribution_type: DistributionType::Unlocked,
    }
}

#[test]
fn test_create_and_get_project() {
    let mut emulator = Emulator::default();

    let project = get_project_1();
    emulator.contract.create_project(get_project_1());
    let projects = emulator.contract.get_projects(None, None, None); 

    assert_eq!(1, projects.len(), "The number of projects in the contract is not correct!");

    let json_project = &projects[0];

    assert_eq!(1, json_project.id, "The created project id must equal to 1.");


    let new_projects = emulator.contract.get_projects(Some(ProjectStatus::Preparation), None, None); 
    let approved_projects = emulator.contract.get_projects(Some(ProjectStatus::Whitelist), None, None); 

    assert_eq!(1, new_projects.len(), "The number of NEW projects in the contract is not correct!");
    assert_eq!(0, approved_projects.len(), "The number of Whitelist projects in the contract is not correct!");
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

    let project_id = emulator.contract.create_project(project);
    let project = emulator.contract.internal_get_project_or_panic(project_id);

    // Preparation
    assert_eq!(ProjectStatus::Preparation, project.status);

    // Whitelist
    let before_whitelist_time = decrease_timestamp(&whitelist_start_date, 1, 0, 0, 0);
    emulator.set_block_timestamp(before_whitelist_time);
    emulator.set_account_id_and_desposit(owner(), owner(), 0);
    emulator.contract.update_project_whitelist_date(project_id, None, None);

    println!("Project's status: {:?}, Current Time: {}, Whitelist Start Time: {}", project.status, before_whitelist_time, whitelist_start_date);
    emulator.contract.change_project_status(project_id);
    
    let project = emulator.contract.internal_get_project_or_panic(project_id);
    assert_eq!(ProjectStatus::Whitelist, project.status);

    // Sales
    emulator.contract.update_project_sales_date(project_id);
    emulator.contract.change_project_status(project_id);

    let project = emulator.contract.internal_get_project_or_panic(project_id);
    assert_eq!(ProjectStatus::Sales, project.status);

    // Distribution
    emulator.contract.update_project_sales_date_to_end(project_id);
    emulator.contract.change_project_status(project_id);

    let project = emulator.contract.internal_get_project_or_panic(project_id);
    assert_eq!(ProjectStatus::Distribution, project.status);
}