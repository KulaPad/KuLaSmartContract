use near_sdk::{serde_json::json, json_types::U128};
use near_sdk_sim::{to_yocto};
use near_sdk_sim::transaction::ExecutionStatus;
use crate::utils::{init, storage_deposit, ft_transfer_call, get_account_info, print_result, 
    FT_TOTAL_SUPPY, FT_STAKING_CONTRACT_BALANCE, ALICE_DEPOSIT_BALANCE, 
};

#[test]
fn init_contract_test() {
    let (root, ft_contract, staking_contract, alice) = init();

    // test deploy ft_contract
    let total_suppy: String = root.view(
        ft_contract.account_id(), 
        "ft_total_supply",
        &json!({}).to_string().as_bytes()
    ).unwrap_json();

    println!("Total supply: {}", total_suppy);
    assert_eq!(FT_TOTAL_SUPPY, total_suppy, "Total supply must equal {}", FT_TOTAL_SUPPY);

    // test alice balance
    let alice_balance: String = root.view(
        ft_contract.account_id(), 
        "ft_balance_of", 
        &json!({
            "account_id": alice.account_id()
        }).to_string().as_bytes()
    ).unwrap_json();

    println!("Alice balance: {}", alice_balance);
    assert_eq!(FT_STAKING_CONTRACT_BALANCE, alice_balance, "Alice balance must equal {}", FT_STAKING_CONTRACT_BALANCE);

    // test staking contract balance
    let staking_balance: String = root.view(
        ft_contract.account_id(), 
        "ft_balance_of", 
        &json!({
            "account_id": staking_contract.account_id()
        }).to_string().as_bytes()
    ).unwrap_json();

    println!("Staking contract balance: {}", staking_balance);
    assert_eq!(FT_STAKING_CONTRACT_BALANCE, staking_balance, "Staking contract balance must equal {}", FT_STAKING_CONTRACT_BALANCE);
}

#[test]
fn deposit_and_stake_test() {
    let (_, ft_contract, staking_contract, alice) = init();

    // staking contract storage deposit
    storage_deposit(&staking_contract, &alice, to_yocto("0.01"));

    ft_transfer_call(&ft_contract, &staking_contract, &alice, ALICE_DEPOSIT_BALANCE, "");

    let account_json = get_account_info(&staking_contract, &alice);

    assert_eq!(account_json.account_id, alice.account_id());
    assert_eq!(account_json.staked_balance, U128(100000000000));
    assert!(account_json.reward.0 > 0);
    assert_eq!(account_json.unstaked_balance.0, 0);
}

#[test]
fn deposit_and_stake_and_lock_test() {
    let (_, ft_contract, staking_contract, alice) = init();

    // staking contract storage deposit
    storage_deposit(&staking_contract, &alice, to_yocto("0.01"));

    let outcome = ft_transfer_call(&ft_contract, &staking_contract, &alice, ALICE_DEPOSIT_BALANCE, "lock:36");

    print_result(&outcome);

    outcome.assert_success();

    let account_json = get_account_info(&staking_contract, &alice);

    assert_eq!(account_json.account_id, alice.account_id());
    assert_eq!(account_json.staked_balance, U128(100_000_000_000));
    assert_eq!(account_json.locked_balance, U128(100_000_000_000));
    assert_eq!(account_json.locked_days, 36);
    assert_eq!(account_json.point, U128(10_000_000_000));
    assert!(account_json.reward.0 > 0);
    assert_eq!(account_json.unstaked_balance.0, 0);
}

#[test]
fn deposit_and_stake_error_storage_test() {
    let (_, ft_contract, staking_contract, alice) = init();

    // Deposit without storage deposit
    let outcome = ft_transfer_call(&ft_contract, &staking_contract, &alice, ALICE_DEPOSIT_BALANCE, "");

    // Have one error
    assert_eq!(outcome.promise_errors().len(), 1);

    // assert error type
    if let ExecutionStatus::Failure(error) = &outcome.promise_errors().remove(0).unwrap().outcome().status {
        println!("Error: {}", error.to_string());
        assert!(error.to_string().contains("ERR_NOT_FOUND_ACCOUNT"));
    } else {
        unreachable!();
    }
}