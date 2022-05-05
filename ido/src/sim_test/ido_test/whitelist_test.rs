use crate::main::{init};
use near_sdk_sim::{call, view, DEFAULT_GAS, STORAGE_AMOUNT, to_yocto};
use near_sdk_sim::transaction::{ExecutionStatus};
use kulapad_ido::project::{ProjectJson};

pub fn init_ido_whitelist() -> (UserAccount,UserAccount,UserAccount,UserAccount,UserAccount,UserAccount){
    let (root,alice,ido,ft_contract,staking_contract,ido_contract) = init();

    ido.call(
        ido_contract.account_id(),
        "internal_change_project_status",
        &json!({
            "project_id" : 1
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    ido.call(
        ido_contract.account_id(),
        "internal_change_project_status",
        &json!({
            "project_id" : 2
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    ido.call(
        ido_contract.account_id(),
        "internal_change_project_status",
        &json!({
            "project_id" : 3
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    (root,alice,ido,ft_contract,staking_contract,ido_contract)
}

#[test]
pub fn test_join_whitelist(){
    let (root,alice,ido,ft_contract,staking_contract,ido_contract) = init_ido_whitelist();
    
    let is_whitelisted : bool = root.view(
        ido_contract.account_id(),
        "is_whitelist",
        &json!({
            "project_id" : 1
        }).to_string().as_bytes(),
    ); // if there is some error,try .unwrap_json()

    assert_eq!(is_whitelisted,false, "Joined whitelist when not call");

    alice.call(
        ido_contract.account_id(),
        "register_whitelist",
        &json!({
            "project_id" : 2
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    let is_whitelisted : bool = root.view(
        ido_contract.account_id(),
        "is_whitelist",
        &json!({
            "project_id" : 2
        }).to_string().as_bytes(),
    );

    assert!(is_whitelisted, "Not join whitelist after register");

} 


#[test]
pub fn test_resolve_get_account_point_for_register_whitelist(){
    let (root,alice,ido,ft_contract,staking_contract,ido_contract) = init_ido_whitelist();

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
        "ft_transfer_callback", 
        &json!({
            "receiver_id": staking_contract.account_id(),
            "amount": 1_000_000_000_000_000_000_000_000, //1000
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
            "amount" : 1_000_000_000_000_000_000_000_000,
            "locked_time": 8460000000000000 //about 100 days
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    let point_staking : U64 = root.view(
        ido_contract.account_id(),
        "get_user_point",
        &json!({
            "account_id" : alice.account_id()
        }).to_string().as_bytes(),
    );

    let project : ProjectJson = root.view(
        ido_contract.account_id(),
        "get_project",
        &json!({
            "project_id" : 3
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    ).unwrap();

    if let WhitelistType::XToken(xtoken) = project.whitelist_type{
        assert!(point_staking.0 >= (point_require.0 as u64),"Not enough XToken point");

        alice.call(
            ido_contract.account_id(),
            "register_whitelist",
            &json!({
                "project_id" : 3
            }).to_string().as_bytes(),
            DEFAULT_GAS,
            0
        );

        let is_whitelisted : bool = root.view(
            ido_contract.account_id(),
            "is_whitelist",
            &json!({
                "project_id" : 2
            }).to_string().as_bytes(),
        );
        
        assert_eq!(is_whitelisted,false, "Joined whitelist when not call");

    } else{
        panic!("Whitelist type is not require xtokens");
    };

    

    
}