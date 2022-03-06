use crate::*;

pub fn get_storage_key(key: StorageKey) -> Vec<u8> {
    key.try_to_vec().unwrap()
}