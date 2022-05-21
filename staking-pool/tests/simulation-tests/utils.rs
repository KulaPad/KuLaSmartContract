use near_sdk::{Balance, serde_json::json};
use near_sdk::json_types::{U128};
use near_sdk_sim::{init_simulator, to_yocto, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT, ExecutionResult};
use staking_contract::AccountJson;
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FT_METADATA_SPEC,
};
near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    FT_CONTRACT_WASM_FILE => "../res/kulapad_token.wasm",
    STAKING_CONTRACT_WASM_FILE => "../res/kulapad_staking.wasm"
}

const FT_CONTRACT_ID: &str = "ft_contract";
const STAKING_CONTRACT_ID: &str = "staking_contract";
pub const FT_TOTAL_SUPPY: &str = "100000000000000000";
pub const FT_STAKING_CONTRACT_BALANCE: &str = "50000000000000000";
pub const ALICE_DEPOSIT_BALANCE: Balance = 100_000_000_000;

pub fn init() -> (UserAccount, UserAccount, UserAccount, UserAccount) {
    let root = init_simulator(None);

    let alice = root.create_user("alice".to_string(), to_yocto("100"));

    let ft_token_metadata = FungibleTokenMetadata {
        spec: FT_METADATA_SPEC.to_string(),
        name: "KULA fungible token".to_string(),
        symbol: "KULA".to_string(),
        icon: None,
        reference: None,
        reference_hash: None,
        decimals: 8,
    };

    // Deploy and init 1B Token
    let ft_contract = root.deploy_and_init(
        &FT_CONTRACT_WASM_FILE,
        FT_CONTRACT_ID.to_string(),
        "new",
        &json!({
            "owner_id": alice.account_id(),
            "total_supply": FT_TOTAL_SUPPY,
            "metadata": ft_token_metadata,
        })
        .to_string()
        .as_bytes(),
        STORAGE_AMOUNT,
        DEFAULT_GAS,
    );

    // Deploy and init staking contract
    let staking_contract = root.deploy_and_init(
        &STAKING_CONTRACT_WASM_FILE,
        STAKING_CONTRACT_ID.to_string(),
        "new_default_config",
        &json!({
            "owner_id": alice.account_id(),
            "ft_contract_id": ft_contract.account_id()
        })
        .to_string()
        .as_bytes(),
        STORAGE_AMOUNT,
        DEFAULT_GAS,
    );

    // storage deposit
    root.call(
        ft_contract.account_id(),
        "storage_deposit",
        &json!({
            "account_id": staking_contract.account_id()
        })
        .to_string()
        .as_bytes(),
        DEFAULT_GAS,
        to_yocto("0.01"),
    );

    // Transfer 50% total supply to staking contract
    alice.call(
        ft_contract.account_id(),
        "ft_transfer",
        &json!({
            "receiver_id": staking_contract.account_id(),
            "amount": FT_STAKING_CONTRACT_BALANCE
        })
        .to_string()
        .as_bytes(),
        DEFAULT_GAS,
        1,
    );

    (root, ft_contract, staking_contract, alice)
}

pub fn storage_deposit(
    contract: &UserAccount,
    account: &UserAccount,
    amount: Balance,
) -> ExecutionResult {
    account.call(
        contract.account_id(),
        "storage_deposit",
        &json!({}).to_string().as_bytes(),
        DEFAULT_GAS,
        amount,
    )
}

pub fn ft_transfer_call(
    ft_contract: &UserAccount,
    receiver_contract: &UserAccount,
    account: &UserAccount,
    amount: Balance,
    msg: &str,
) -> ExecutionResult {
    account.call(
        ft_contract.account_id(),
        "ft_transfer_call",
        &json!({
            "receiver_id": receiver_contract.account_id(),
            "amount": U128(amount),
            "msg": msg
        })
        .to_string()
        .as_bytes(),
        DEFAULT_GAS,
        1,
    )
}

pub fn get_account_info(staking_contract: &UserAccount, account: &UserAccount) -> AccountJson {
    let account_json: AccountJson = staking_contract
        .view(
            staking_contract.account_id(),
            "get_account_info",
            &json!({
                "account_id": account.account_id()
            })
            .to_string()
            .as_bytes(),
        )
        .unwrap_json();
    account_json
}

pub fn print_result(result: &ExecutionResult) {
    println!("{:?}", result);
    println!("{:?}", result.promise_results());

    for receipt_result in result.get_receipt_results() {
        if let Some(receipt_result) = receipt_result {
            print_result(&receipt_result);
        }
    }
}