use crate::main::{init};
use near_sdk_sim::{call, view, DEFAULT_GAS, STORAGE_AMOUNT, to_yocto};
use near_sdk_sim::transaction::{ExecutionStatus};


#[test]
pub fn test_join_whitelist(){
    let (root,alice,ido,ft_contract,staking_contract,ido_contract) = init();
    
    let is_whitelisted : bool = root.view(
        ido_contract.account_id(),
        "is_whitelist",
        &json!({
            "project_id" : 8
        }).to_string().as_bytes(),
    ); // nếu lỗi thì thử .unwrap_json()

    assert_eq!(is_whitelisted,false, "Joined whitelist when not call");

    alice.call(
        ido_contract.account_id(),
        "register_whitelist",
        &json!({
            "project_id" : 8
        }).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    let is_whitelisted : bool = root.view(
        ido_contract.account_id(),
        "is_whitelist",
        &json!({

        }).to_string().as_bytes(),
    );

    assert!(is_whitelisted, "Not join whitelist after register");

} 


#[test]
pub fn test_resolve_get_account_info_for_updating_tickets(){
    let (root,alice,ido,ft_contract,staking_contract,ido_contract) = init();


}