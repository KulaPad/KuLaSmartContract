use crate::utils::{init,print_result};
use kulapad_ido::modules::account::{ProjectAccountJson,AccountSale, AccountSaleData, AccountSaleJson};
use kulapad_ido::modules::project::{ProjectJson};
use near_sdk::serde_json::json;
use near_sdk::serde::{Serialize,Deserialize};
use near_sdk::json_types::{U64,U128};
use near_sdk_sim::{DEFAULT_GAS, to_yocto};
use near_sdk_sim::UserAccount;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Message{
    pub project_id: u64
}

pub fn init_whitelisting_project()-> (UserAccount, UserAccount, UserAccount,UserAccount, UserAccount, UserAccount){
    let (root,alice,ido,ft_contract,staking_contract,ido_contract) = init();
    root.call(
        ido_contract.account_id(),
        "change_project_status",
        &json!({
            "project_id" : 1
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    root.call(
        ido_contract.account_id(),
        "change_project_status",
        &json!({
            "project_id" : 2
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    alice.call(
        ido_contract.account_id(),
        "register_whitelist",
        &json!({
            "project_id" : 1
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    let mut is_whitelisted : bool = root.view(
        ido_contract.account_id(),
        "is_whitelist",
        &json!({
            "project_id" : 1,
            "account_id" : alice.account_id()
        }).to_string().as_bytes(),
    ).unwrap_json();
    
    assert!(is_whitelisted,"Not join whitelist project 1 after register");

    alice.call(
        ido_contract.account_id(),
        "register_whitelist",
        &json!({
            "project_id" : 1
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    is_whitelisted = root.view(
        ido_contract.account_id(),
        "is_whitelist",
        &json!({
            "project_id" : 1,
            "account_id" : alice.account_id()
        }).to_string().as_bytes(),
    ).unwrap_json();
    
    assert!(is_whitelisted,"Not join whitelist project 2 after register");

    let current_time : U64 = root.call(
        ido_contract.account_id(),
        "get_current_block_timestamp",
        &json!({}).to_string().as_bytes(),
        DEFAULT_GAS,0
    ).unwrap_json();
    println!("Current time: {}", current_time.0);

    ido.call(
        ido_contract.account_id(),
        "update_project_sales_date",
        &json!({
            "project_id": 1
        }).to_string().as_bytes(),
        DEFAULT_GAS, 0
    );

    root.call(
        ido_contract.account_id(),
        "change_project_status",
        &json!({
            "project_id" : 1
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    let mut project_json : ProjectJson = root.view(
        ido_contract.account_id(),
        "get_project",
        &json!({
            "project_id" : 1
        }).to_string().as_bytes()
    ).unwrap_json();
    println!("{:?}\n",project_json);
    

    ido.call(
        ido_contract.account_id(),
        "update_project_sales_date",
        &json!({
            "project_id": 2
        }).to_string().as_bytes(),
        DEFAULT_GAS, 0
    );
    
    root.call(
        ido_contract.account_id(),
        "change_project_status",
        &json!({
            "project_id" : 2
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    project_json = root.view(
        ido_contract.account_id(),
        "get_project",
        &json!({
            "project_id" : 2
        }).to_string().as_bytes()
    ).unwrap_json();
    println!("{:?}\n",project_json);

    (root,alice,ido,ft_contract,staking_contract,ido_contract)
}

#[test]
pub fn test_commit_sale_share_project(){
    let (root,alice,ido,ft_contract,staking_contract,ido_contract) = init_whitelisting_project();

    let default_share_project_account_sale = AccountSale{
        committed_amount: 100,
        sale_data: AccountSaleData::Shared
    };
    let default_share_project_account_sale_json = AccountSaleJson::from(default_share_project_account_sale);


    let outcome = alice.call(
        ft_contract.account_id(),
        "ft_transfer_call",
        &json!({
            "receiver_id" : ido_contract.account_id(),
            "amount" : U128(100),
            "msg": "project_id:1"
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        1
    );

    
    print_result(&outcome);
    outcome.assert_success();

    let project_account_json :ProjectAccountJson = root.view(
        ido_contract.account_id(),
        "get_project_account_info",
        &json!({
            "project_id" : 1,
            "account_id" : alice.account_id(),
        }).to_string().as_bytes()
    ).unwrap_json();
    println!("{:?}",project_account_json);

    let share_project_account_sale_json = project_account_json.sale_data.unwrap();
    assert_eq!(default_share_project_account_sale_json.committed_amount,
        share_project_account_sale_json.committed_amount,
        "Committed amount project 1 not match");

    let project_json : ProjectJson = root.view(
        ido_contract.account_id(),
        "get_project",
        &json!({
            "project_id" : 1
        }).to_string().as_bytes()
    ).unwrap_json();
    assert_eq!(project_json.total_fund_committed,U128(100),"Total fund committed not match");
}