use crate::tests::test_emulator::*;
use crate::tests::test_utils::*;
use crate::modules::account::*;
use near_sdk::json_types::U128;

#[test]
fn test_buy_token() {
    let mut emulator = Emulator::default();
    emulator.update_context("alice".to_string(), "bob".to_string(), 15_000_000_000_000_000_000_000_000);
    emulator.contract.create_sample_projects();
    
    let project_id = 7;
    let account_token_sales = "bob".to_string();
    
    // TODO: 
}
