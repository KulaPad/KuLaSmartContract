use crate::tests::test_utils::*;
use crate::tests::test_emulator::*;

#[test]
fn test_account() {
    let emulator = Emulator::default();

    assert_eq!(owner(), emulator.contract.get_owner_id(), "The owner id is not correct!")
}