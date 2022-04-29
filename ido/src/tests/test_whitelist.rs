use crate::tests::test_emulator::*;
use crate::tests::test_utils::*;


#[test]
fn test_register_whitelist() {
    let mut emulator = Emulator::default();
    emulator.update_context("alice".to_string(), "owner".to_string(), 0);

    emulator.contract.create_sample_projects();
    emulator.update_context("alice".to_string(), "bob".to_string(), 0);
    emulator.contract.register_whitelist(1);
    emulator.contract.register_whitelist(2);
    assert_eq!(emulator.contract.projects_by_account.get(&"bob".to_string()).unwrap().len(),2);
}

#[test]
fn test_is_whitelist(){
    let mut emulator = Emulator::default();
    emulator.update_context("alice".to_string(), "owner".to_string(), 0);

    emulator.contract.create_sample_projects();
    emulator.update_context("alice".to_string(), "bob".to_string(), 0);
    emulator.contract.register_whitelist(1);
    assert!(emulator.contract.is_whitelist(1,None));
    assert!(!emulator.contract.is_whitelist(1,Some("alice".to_string())));
}