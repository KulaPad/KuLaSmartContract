use near_sdk::{serde_json::json, json_types::U128};
use near_sdk_sim::{init_simulator, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT, to_yocto};
use near_sdk_sim::transaction::ExecutionStatus;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    IDO_WASM_FILE => "../res/kulapad_ido.wasm",
    STAKING_WASM_FILE => "../res/kulapad_staking.wasm",
    TOKEN_WASM_FILE => "../res/kulapad_token.wasm"
}

const STAKING_CONTRACT_ID: &str = "staking_contract";
const FT_CONTRACT_ID: &str = "ft_contract";
const IDO_CONTRACT_ID: &str = "ido_contract";
const FT_TOTAL_SUPPY: &str = "100_000_000_000_000_000_000_000_000_000";
const FT_STAKING_CONTRACT_BALANCE: &str = "50_000_000_000_000_000_000_000_000_000";


mod staking_test;
mod ido_test;

pub fn init() -> (UserAccount,UserAccount,UserAccount,UserAccount,UserAccount,UserAccount){
    let root = init_simulator(None);

    let alice = root.create_user("alice".to_string(), to_yocto("100"));
    let ido = root.create_user("ido".to_string(),to_yocto("100"));

    let ft_contract = root.deploy_and_init(
        &TOKEN_WASM_FILE,
        FT_CONTRACT_ID,
        "new_default_meta",
        &json!({
            "owner_id" : alice.account_id(),
            "total_supply" : FT_TOTAL_SUPPY
        }).to_string().as_bytes(),
        STORAGE_AMOUNT, 
        DEFAULT_GAS
    );

    let staking_contract = root.deploy_and_init(
        &STAKING_WASM_FILE,
        STAKING_CONTRACT_ID,
        "new_default_config",
        &json!({
            "owner_id": alice.account_id(),
            "total_supply" : FT_CONTRACT_ID
        }).to_string().as_bytes(),
        STORAGE_AMOUNT, 
        DEFAULT_GAS
    );

    let ido_contract = root.deploy_and_init(
        &IDO_WASM_FILE,
        IDO_CONTRACT_ID,
        "new",
        &json!({
            "owner_id" : ido.account_id(),
            "staking_contract_id" : staking_contract.account_id()
        }).to_string().as_bytes(),
        STORAGE_AMOUNT, 
        DEFAULT_GAS
    );
    
    ido.call(
        ido_contract.account_id(),
        "create_sample_projects",
        &json!({}).to_string().as_bytes(),
        DEFAULT_GAS,
        0
    );

    // storage deposit
    root.call(
        ft_contract.account_id(), 
        "storage_deposit", 
        &json!({
            "account_id": staking_contract.account_id()
        }).to_string().as_bytes(), 
        DEFAULT_GAS, 
        to_yocto("0.01")
    );

    // Transfer 50% total supply to staking contract
    alice.call(
        ft_contract.account_id(), 
        "ft_transfer", 
        &json!({
            "receiver_id": staking_contract.account_id(),
            "amount": FT_STAKING_CONTRACT_BALANCE
        }).to_string().as_bytes(), 
        DEFAULT_GAS, 
        1
    );

    (root,alice,ido,ft_contract,staking_contract,ido_contract)
}


#[test]
pub fn init_contract_test(){
    let (root,alice,ido,ft_contract,staking_contract,ido_contract) = init();

    // test deploy ft_contract
    let total_suppy: String = root.view(
        ft_contract.account_id(), 
        "ft_total_supply",
        &json!({}).to_string().as_bytes()
    ).unwrap_json();

    println!("Total supply: {}", total_suppy);
    assert_eq!(FT_TOTAL_SUPPY, total_suppy, "Total supply must equal {}", FT_TOTAL_SUPPY);
}