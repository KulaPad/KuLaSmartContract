use crate::tests::test_emulator::*;
use crate::tests::test_utils::*;
use crate::structures::account::*;
use near_sdk::json_types::U128;

#[test]
fn test_buy_token() {
    let mut emulator = Emulator::default();
    emulator.update_context("alice".to_string(), "bob".to_string(), 15_000_000_000_000_000_000_000_000);
    

    emulator.contract.create_sample_projects();
    emulator.contract.create_default_account_token_sales(7,&"bob".to_string());
    emulator.contract.buy_token(7);
    
    let json_account_token_sales = JsonAccountTokenSales{
        funding_amount: U128::from(15_000_000_000_000_000_000_000_000),
        token_unlocked_amount: U128::from(0),
        token_locked_amount: U128::from(0),
        token_withdrawal_amount: U128::from(0),
    };
    assert_eq!(Some(json_account_token_sales),emulator.contract.get_account_token_sale_info(7));

}
