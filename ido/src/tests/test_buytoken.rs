use crate::tests::test_emulator::*;
use crate::modules::account::*;
use near_sdk::json_types::U128;
use crate::modules::project::ProjectStatus;

#[test]
fn test_buy_token_by_near() {
    let mut emulator = Emulator::default();
    emulator.update_context("alice".to_string(), "owner".to_string(), 0);
    emulator.contract.create_sample_projects();

    emulator.update_context("alice".to_string(), "bob".to_string(), 0);
    emulator.contract.internal_change_project_status(5);
    emulator.contract.register_whitelist(5);
    let is_whitelist = emulator.contract.is_whitelist(5,"bob".to_string());
    assert!(is_whitelist);
    emulator.update_context("alice".to_string(), "owner".to_string(), 0);
    emulator.contract.update_project_status(5, ProjectStatus::Sales);
    

    emulator.update_context("alice".to_string(), "bob".to_string(), 25_000_000_000_000_000_000_000_000);
    emulator.set_block_timestamp(1651804401000000000);
    
    let default_share_project_account_sale = AccountSale{
        committed_amount: 25_000_000_000_000_000_000_000_000,
        sale_data: AccountSaleData::Shared
    };
    let default_share_project_account_sale_json = AccountSaleJson::from(default_share_project_account_sale);

    emulator.contract.commit(5);
   
    let share_project_account_sale_json = emulator.contract.internal_get_project_account_info(5,"bob".to_string()).sale_data.unwrap();
    assert_eq!(default_share_project_account_sale_json.committed_amount,
                share_project_account_sale_json.committed_amount,
                "Committed amount project 5 not match");
}

#[test]
fn test_commit_shared_project_by_ft_token() {
    let mut emulator = Emulator::default();
    emulator.update_context("alice".to_string(), "owner".to_string(), 0);
    emulator.contract.create_sample_projects();

    emulator.update_context("alice".to_string(), "bob".to_string(), 0);
    emulator.contract.internal_change_project_status(1);
    emulator.contract.register_whitelist(1); 

    emulator.update_context("alice".to_string(), "owner".to_string(), 0);
    emulator.contract.update_project_status(1, ProjectStatus::Sales);
    

    emulator.update_context("alice".to_string(), "bob".to_string(), 20_000_000_000_000);
    emulator.set_block_timestamp(1651804401000000000);
    
    let default_share_project_account_sale = AccountSale{
        committed_amount: 25,
        sale_data: AccountSaleData::Shared
    };
    
    let default_share_project_account_sale_json = AccountSaleJson::from(default_share_project_account_sale);
    emulator.contract.internal_commit(1, &"bob".to_string(), 25);
   
    let share_project_account_sale_json = emulator.contract.internal_get_project_account_info(1,"bob".to_string()).sale_data.unwrap();
    assert_eq!(default_share_project_account_sale_json.committed_amount,
                share_project_account_sale_json.committed_amount,
                "Committed amount project 1 not match");
}

#[test]
fn test_commit_lottery_project_by_ft_token(){
    let mut emulator = Emulator::default();
    emulator.update_context("alice".to_string(), "owner".to_string(), 0);
    emulator.contract.create_sample_projects();

    emulator.update_context("alice".to_string(), "bob".to_string(), 0);
    emulator.contract.internal_change_project_status(2);
    emulator.contract.register_whitelist(2);

    emulator.update_context("alice".to_string(), "owner".to_string(), 0);
    emulator.contract.update_project_status(2, ProjectStatus::Sales);
    emulator.update_account_sale_ticket("bob".to_string(), 2, 6);

    emulator.update_context("alice".to_string(), "bob".to_string(), 20_000_000_000_000);
    emulator.set_block_timestamp(1651804401000000000);

    let mut default_lottery_project_account_sale = AccountSale{
        committed_amount: 20,
        sale_data: AccountSaleData::Lottery(
            LotteryAccountSaleData{
                eligible_tickets: 6,
                deposit_tickets: 2,
                ticket_ids: vec![0,1],
                win_ticket_ids: vec![]
            }
        )
    };

    let mut default_lottery_project_account_sale_json = AccountSaleJson::from(default_lottery_project_account_sale);
    emulator.contract.internal_commit(2, &"bob".to_string(), 25);
    let mut lottery_project_account_sale_json = emulator.contract.internal_get_project_account_info(2,"bob".to_string()).sale_data.unwrap();

    assert_eq!(default_lottery_project_account_sale_json.committed_amount,
        lottery_project_account_sale_json.committed_amount,
        "Committed amount project 2 phase 1 not match ");
    assert_eq!(default_lottery_project_account_sale_json.lottery_sale_data.unwrap(),
            lottery_project_account_sale_json.lottery_sale_data.unwrap(),
            "Lottery sale data not match ");

    emulator.update_context("alice".to_string(), "bob".to_string(), 20_000_000_000_000);
    emulator.set_block_timestamp(1651804401000000000);
    emulator.contract.internal_commit(2, &"bob".to_string(), 30);


    default_lottery_project_account_sale = AccountSale{
        committed_amount:50,
        sale_data: AccountSaleData::Lottery(
            LotteryAccountSaleData{
                eligible_tickets: 6,
                deposit_tickets: 5,
                ticket_ids: vec![0,1,2,3,4],
                win_ticket_ids: vec![]
            }
        )
    };
    default_lottery_project_account_sale_json = AccountSaleJson::from(default_lottery_project_account_sale);
    lottery_project_account_sale_json = emulator.contract.internal_get_project_account_info(2,"bob".to_string()).sale_data.unwrap();
    assert_eq!(default_lottery_project_account_sale_json.committed_amount,
        lottery_project_account_sale_json.committed_amount,
        "Committed amount project 2 phase 2 not match ");
    assert_eq!(default_lottery_project_account_sale_json.lottery_sale_data.unwrap(),
        lottery_project_account_sale_json.lottery_sale_data.unwrap(),
        "Lottery sale data not match ");

}