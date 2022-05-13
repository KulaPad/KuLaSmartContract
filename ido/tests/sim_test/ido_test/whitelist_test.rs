use crate::utils::init;
use near_sdk::serde_json::json;
use near_sdk::json_types::{U64, U128};
use near_sdk_sim::{DEFAULT_GAS, to_yocto};
use kulapad_ido::modules::project::{ProjectJson,ProjectStatus,WhitelistType};
use kulapad_ido::modules::account::ProjectAccountJson;
use kulapad_ido::staking_contract::AccountJson;

#[test]
pub fn test_join_whitelist(){
    let (root,alice,ido,_,_,ido_contract) = init();
    
    ido.call(
        ido_contract.account_id(),
        "update_project_whitelist_date",
        &json!({
            "project_id" : 1,
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    ido.call(
        ido_contract.account_id(),
        "update_project_status",
        &json!({
            "project_id" : 1,
            "new_status": ProjectStatus::Whitelist
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    let project_json : ProjectJson = root.view(
        ido_contract.account_id(),
        "get_project",
        &json!({
            "project_id" : 1
        }).to_string().as_bytes()
    ).unwrap_json();
    println!("{:?}",project_json);
    assert_eq!(project_json.whitelist_type,WhitelistType::None,"Project 1 WhitelistType not match");

    let current_time : U64 = root.view(
        ido_contract.account_id(),
        "get_current_block_timestamp",
        &json!({}).to_string().as_bytes()
    ).unwrap_json();
    println!("Current time: {}",current_time.0);

    let mut is_whitelisted : bool = root.view(
        ido_contract.account_id(),
        "is_whitelist",
        &json!({
            "project_id" : 1,
            "account_id": alice.account_id()
        }).to_string().as_bytes()
    ).unwrap_json();

    
    assert_eq!(is_whitelisted,false, "Joined whitelist when not call");

    alice.call(
        ido_contract.account_id(),
        "internal_register_whitelist",
        &json!({
            "account_id" : alice.account_id(),
            "project_id" : 1
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    let project_account_json :ProjectAccountJson = root.view(
        ido_contract.account_id(),
        "get_project_account_info",
        &json!({
            "project_id" : 1,
            "account_id" : alice.account_id(),
        }).to_string().as_bytes()
    ).unwrap_json();
    println!("{:?}",project_account_json);
    // assert!(project_account_json.is_whitelist,"ProjectAccountJson not match");

    is_whitelisted = root.view(
        ido_contract.account_id(),
        "is_whitelist",
        &json!({
            "project_id" : 1,
            "account_id": alice.account_id()
        }).to_string().as_bytes()
    ).unwrap_json();

    assert!(is_whitelisted, "Not join whitelist after register");

} 


#[test]
pub fn test_resolve_get_account_point_for_register_whitelist(){
    let (root,alice,ido,ft_contract,staking_contract,ido_contract) = init();

    // Storage deposit
    alice.call(
        staking_contract.account_id(), 
        "storage_deposit", 
        &json!({}).to_string().as_bytes(), 
        DEFAULT_GAS,
        to_yocto("0.01") 
    );

    // Deposit token
    alice.call(
        ft_contract.account_id(), 
        "ft_transfer_call", 
        &json!({
            "receiver_id": staking_contract.account_id(),
            "amount": U128(1_000_000_000_000_000_000_000_000), //1000
            "msg": ""
        }).to_string().as_bytes(), 
        DEFAULT_GAS, 
        1
    );

    // Lock token
    alice.call(
        staking_contract.account_id(),
        "lock",
        &json!({
            "amount" : U128(1_000_000_000_000_000_000),
            "locked_days": 100 as u32
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    let account_json : AccountJson = root.view(
        staking_contract.account_id(),
        "get_account_info",
        &json!({
            "account_id" : alice.account_id()
        }).to_string().as_bytes(),
    ).unwrap_json();
    println!("AccountJson: {:?}",account_json);

    let project : ProjectJson = root.view(
        ido_contract.account_id(),
        "get_project",
        &json!({
            "project_id" : 3
        }).to_string().as_bytes(),
    ).unwrap_json();

    if let WhitelistType::XToken(xtoken) = project.whitelist_type{
        assert!(account_json.point.0 >= xtoken,"Not enough XToken point");

        ido.call(
            ido_contract.account_id(),
            "update_project_whitelist_date",
            &json!({
                "project_id" : 3,
            }).to_string().as_bytes(),
            DEFAULT_GAS,
            0
        );
    
        ido.call(
            ido_contract.account_id(),
            "update_project_status",
            &json!({
                "project_id" : 3,
                "new_status": ProjectStatus::Whitelist
            }).to_string().as_bytes(),
            DEFAULT_GAS,
            0
        );

        // Register Whitelist
        alice.call(
            ido_contract.account_id(),
            "register_whitelist",
            &json!({
                "project_id" : 3
            }).to_string().as_bytes(),
            DEFAULT_GAS,
            0
        );

        let mut is_whitelisted : bool = root.view(
            ido_contract.account_id(),
            "is_whitelist",
            &json!({
                "project_id" : 2,
                "account_id" : alice.account_id()
            }).to_string().as_bytes(),
        ).unwrap_json();
        
        assert_eq!(is_whitelisted,false, "Joined whitelist when not call");

        is_whitelisted = root.view(
            ido_contract.account_id(),
            "is_whitelist",
            &json!({
                "project_id" : 3,
                "account_id" : alice.account_id()
            }).to_string().as_bytes(),
        ).unwrap_json();

        assert!(is_whitelisted,"Not join whitelist after register")
    } else{
        panic!("Whitelist type is not require xtokens");
    };

    

    
}