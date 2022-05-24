use crate::tests::test_utils::*;
use crate::tests::test_emulator::*;
use crate::tests::test_project::*;

// Test get_project_account_info

#[test]
#[should_panic(expected = "Project does not exist.")]
fn test_get_project_account_info_project_not_exist() { // - Project doesn't exist
    let emulator = Emulator::default();

    emulator.contract.get_project_account_info(1, bob());
}

#[test]
fn test_get_project_account_info_project_exists_account_not() { // - Project existed, account doesn't exist
    let mut emulator = Emulator::default();
    let owner = emulator.contract.get_owner_id();

    emulator.update_context(owner.clone(), owner.clone(), 0);
    let project = get_project_1();
    emulator.contract.create_project(project);

    emulator.update_context(bob(), bob(), 0);

    let project = emulator.contract.get_project_account_info(1, bob());

    assert_eq!(1, project.project_id);
    assert_eq!(bob(), project.account_id);
    assert_eq!(false, project.is_whitelist);
    assert!(project.sale_data.is_none());
    assert!(project.distribution_data.is_none());
}

// TODO: Will do when implementing these statuses
// - Project existed with prepration status
// - Project existed with whitelist status
// - Project existed with sale status
// - Project existed with distribution status