use crate::utils::{init};
use kulapad_ido::staking_contract::AccountJson;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_sim::{DEFAULT_GAS, to_yocto};
use near_sdk_sim::transaction::ExecutionStatus;



const ALICE_DEPOSIT_AMOUNT: U128 = U128(10_000_000_000_000_000_000_000_000_000);

#[test]
pub fn test_deposit_and_stake(){
    let (root,alice,_,ft_contract,staking_contract,_) = init();
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
            "amount": ALICE_DEPOSIT_AMOUNT,
            "msg": ""
        }).to_string().as_bytes(), 
        DEFAULT_GAS, 
        1
    );

    let account_json: AccountJson = root.view(
        staking_contract.account_id(), 
        "get_account_info", 
        &json!({
            "account_id": alice.account_id()
        }).to_string().as_bytes()
    ).unwrap_json();

    assert_eq!(account_json.account_id, alice.account_id());
    assert_eq!(account_json.staked_balance, U128(10_000_000_000_000_000_000_000_000_000));
    assert!(account_json.reward.0 > 0);
    assert_eq!(account_json.unstaked_balance.0, 0);
}


#[test]
pub fn test_deposit_and_stake_without_storage() {
    let (_, alice,_, ft_contract, staking_contract,_) = init();

    // Storage deposit
    // alice.call(
    //     staking_contract.account_id(), 
    //     "storage_deposit", 
    //     &json!({}).to_string().as_bytes(), 
    //     DEFAULT_GAS,
    //     to_yocto("0.01") 
    // );

    // Deposit token
    let outcome = alice.call(
        ft_contract.account_id(), 
        "ft_transfer_call", 
        &json!({
            "receiver_id": staking_contract.account_id(),
            "amount": ALICE_DEPOSIT_AMOUNT,
            "msg": ""
        }).to_string().as_bytes(), 
        DEFAULT_GAS, 
        1
    );

    assert_eq!(outcome.promise_errors().len(), 1);

    // assert error type
    if let ExecutionStatus::Failure(error) = &outcome.promise_errors().remove(0).unwrap().outcome().status {
        println!("Excute error: {}", error.to_string());
        assert!(error.to_string().contains("ERR_NOT_FOUND_ACCOUNT"));
    } else {
        unreachable!()
    }
}
