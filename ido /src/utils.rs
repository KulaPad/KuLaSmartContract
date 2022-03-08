use crate::*;

pub(crate) const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;

pub(crate) fn get_storage_key(key: StorageKey) -> Vec<u8> {
    key.try_to_vec().unwrap()
}

pub(crate) fn get_token_raised_amount(human_amount: u64, token_decimal: u8) -> Balance {
    human_amount as u128 * u128::pow(10, token_decimal as u32) 
}

pub(crate) fn get_token_sale_rate(system_amount: u128, numberator: u64, denominator: u64) -> Balance {
    system_amount * numberator as u128 / denominator as u128
}

pub(crate) fn get_token_raised_human_amount(amount: u128, token_decimal: u8) -> u128 {
    amount / u128::pow(10, token_decimal as u32) 
}

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash{
    let mut hash = CryptoHash::default();
    // Hash account ID rồi return chính nó
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));

    hash
}