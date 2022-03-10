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
        total_allocations: 0,
        total_staking_tickets: 0,
        total_social_tickets: 0,
        total_referral_tickets: 0,
    }
}

#[test]
fn test_create_and_get_project() {
    let mut emulator = Emulator::default();

    let project = get_project_1();
    let project_name = project.name.clone();

    // Create a new project with status of Preparation
    emulator.contract.create_project(project);

    // Change project's status to Whitelist

    // User A registers whitelist

    // User B registers whitelist

    // User C do not register whitelist

    // User A stakes & locks Tier1 for 31 days

    // User A updated staking tier

    // Close whitelist

    // User A deposit fund

}
